use std::borrow::Cow;

use reqwest::{
    Client, Request,
    header::{HeaderMap, HeaderValue},
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;

use super::{
    Endpoint,
    errors::{ApiError, ApiErrorCode},
};
use crate::{
    resources::{AppResource, DatabaseResource, WorkspaceResource},
    settings::SETTINGS,
    types::{
        AccountInfo, AppDomain, Database, DatabaseType, RuntimeStatsListItem,
        ServiceStatus, Snapshot, SnapshotScope, UploadedApp, WorkspaceInfo,
    },
};

/// Raw envelope returned by every SquareCloud API endpoint.
///
/// The wire format uses `"status": "success"` or `"status": "error"` as the
/// discriminator. Callers convert it to a `Result` using
/// [`ApiResponse::into_result_t`] or [`ApiResponse::into_bool_result`].
#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ApiResponse<T> {
    Success {
        #[serde(skip_serializing_if = "Option::is_none")]
        response: Option<T>,
    },
    Error {
        code: ApiErrorCode,
    },
}

impl<T> ApiResponse<T> {
    /// Unwraps the `response` field on success, or maps the error code to
    /// [`ApiError::Api`].
    ///
    /// # Panics
    ///
    /// Panics if the API returns a success envelope without a `response` body.
    pub fn into_result_t(self) -> Result<T, ApiError> {
        match self {
            ApiResponse::Error { code } => Err(ApiError::Api { code }),
            ApiResponse::Success { response } => {
                response.ok_or_else(|| panic!("Expected response data"))
            }
        }
    }

    /// Returns `Ok(true)` on success, or propagates the API error code.
    pub fn into_bool_result(self) -> Result<bool, ApiError> {
        match self {
            ApiResponse::Success { .. } => Ok(true),
            ApiResponse::Error { code } => Err(ApiError::Api { code }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ApiResponse;
    use crate::http::errors::{ApiError, ApiErrorCode};

    #[test]
    fn into_result_t_success_returns_inner_value() {
        let resp: ApiResponse<u32> =
            ApiResponse::Success { response: Some(42) };
        assert_eq!(resp.into_result_t().unwrap(), 42);
    }

    #[test]
    fn into_result_t_error_returns_api_error() {
        let resp: ApiResponse<u32> = ApiResponse::Error {
            code: ApiErrorCode::NotFound,
        };
        assert!(matches!(
            resp.into_result_t(),
            Err(ApiError::Api {
                code: ApiErrorCode::NotFound
            })
        ));
    }

    #[test]
    fn into_bool_result_success_returns_true() {
        let resp: ApiResponse<()> = ApiResponse::Success { response: None };
        assert_eq!(resp.into_bool_result().unwrap(), true);
    }

    #[test]
    fn into_bool_result_error_returns_api_error() {
        let resp: ApiResponse<()> = ApiResponse::Error {
            code: ApiErrorCode::RateLimit,
        };
        assert!(matches!(
            resp.into_bool_result(),
            Err(ApiError::Api {
                code: ApiErrorCode::RateLimit
            })
        ));
    }

    #[test]
    fn api_client_default_is_same_as_new() {
        unsafe { std::env::set_var("API_TOKEN", "test") };
        let client = crate::http::ApiClient::default();
        assert!(!client.base_url.is_empty());
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Authenticated HTTP client for the SquareCloud API.
///
/// `ApiClient` is the root entry point for this library. Construct one with
/// [`ApiClient::new`], which reads credentials from the environment, then call
/// methods directly for account-wide operations, or use the resource factory
/// methods ([`app`](ApiClient::app), [`database`](ApiClient::database),
/// [`workspace`](ApiClient::workspace)) to obtain handles scoped to a specific
/// entity.
///
/// # Cloning
///
/// `ApiClient` implements [`Clone`]. The underlying HTTP connection pool
/// (from [`reqwest`]) is shared across clones, so cloning is cheap. The
/// factory methods [`app`](ApiClient::app), [`database`](ApiClient::database),
/// and [`workspace`](ApiClient::workspace) clone the client internally:
///
/// ```no_run
/// # use squarecloud::ApiClient;
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ApiClient::new();
/// let me = client.me().await?;
/// let app = client.app("my-app-id");   // client still usable after this
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct ApiClient {
    pub base_url: String,
    pub(crate) http_client: Client,
}

impl ApiClient {
    /// Creates a new `ApiClient` by reading credentials from the environment.
    ///
    /// On first call, the `API_TOKEN` environment variable is loaded (a `.env`
    /// file in the current directory is automatically sourced via [`dotenvy`]).
    /// The API token is set as the default `Authorization` header on all
    /// subsequent requests.
    ///
    /// # Panics
    ///
    /// Panics if `API_TOKEN` is not set in the environment, or if it contains
    /// non-ASCII characters.
    pub fn new() -> ApiClient {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&SETTINGS.api_token).unwrap(),
        );
        let client: Client = Client::builder()
            .default_headers(headers)
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .http1_only()
            .build()
            .unwrap();
        ApiClient {
            base_url: "https://api.squarecloud.app/v2".to_string(),
            http_client: client,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    pub async fn execute_request<T: DeserializeOwned>(
        &self,
        request: Request,
    ) -> Result<ApiResponse<T>, reqwest::Error> {
        let response = self.http_client.execute(request).await?;
        let response: ApiResponse<T> = response.json().await?;
        Ok(response)
    }

    pub async fn request_endpoint<T: DeserializeOwned>(
        &self,
        endpoint: Endpoint,
    ) -> Result<ApiResponse<T>, reqwest::Error> {
        let mut req = self
            .http_client
            .request(endpoint.method, self.url(&endpoint.path));
        if let Some(body) = endpoint.json_body {
            req = req.json(&body);
        }
        let response: ApiResponse<T> = req.send().await?.json().await?;
        Ok(response)
    }

    /// Returns a resource handle scoped to the application identified by `id`.
    ///
    /// This method **consumes** `self`; call any account-level methods before
    /// converting the client into a resource handle. See the struct-level
    /// documentation for details.
    pub fn app(&self, id: &str) -> AppResource {
        AppResource::new(self.clone(), id)
    }

    /// Returns a resource handle scoped to the workspace identified by `id`.
    pub fn workspace(&self, id: &str) -> WorkspaceResource {
        WorkspaceResource::new(self.clone(), id)
    }

    /// Returns a resource handle scoped to the database identified by `id`.
    pub fn database(&self, id: &str) -> DatabaseResource {
        DatabaseResource::new(self.clone(), id)
    }

    /// Returns the current operational status of the SquareCloud platform.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the HTTP request fails, or
    /// [`ApiError::Api`] if the API responds with an error code.
    pub async fn service_status(&self) -> Result<ServiceStatus, ApiError> {
        let endpoint = Endpoint::service_status();
        let req = self
            .http_client
            .request(endpoint.method, self.url(&endpoint.path));
        Ok(req.send().await?.json().await?)
    }

    /// Returns the account information associated with the current API token.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// if the token is invalid ([`ApiErrorCode::InvalidAccessToken`]).
    pub async fn me(&self) -> Result<AccountInfo, ApiError> {
        self.request_endpoint::<AccountInfo>(Endpoint::me())
            .await?
            .into_result_t()
    }

    /// Uploads a new application from a ZIP archive and returns its metadata.
    ///
    /// The `bytes` parameter accepts any value that converts to a
    /// `Cow<'static, [u8]>`, such as a `Vec<u8>` read from disk. The archive
    /// must contain a valid `squarecloud.app` configuration file at its root.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with one of the following codes if the
    /// uploaded archive is malformed: [`ApiErrorCode::MissingConfig`],
    /// [`ApiErrorCode::MissingMain`], [`ApiErrorCode::InvalidMain`],
    /// [`ApiErrorCode::MissingDisplayName`],
    /// [`ApiErrorCode::InvalidDisplayName`], [`ApiErrorCode::MissingMemory`],
    /// [`ApiErrorCode::InvalidMemory`], [`ApiErrorCode::MissingVersion`],
    /// [`ApiErrorCode::InvalidVersion`], [`ApiErrorCode::FewMemory`], or
    /// [`ApiErrorCode::BadMemory`].
    pub async fn upload_app(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<UploadedApp, ApiError> {
        let endpoint = Endpoint::upload_app();
        let form = Form::new().part(
            "file",
            Part::bytes(bytes)
                .file_name("app.zip")
                .mime_str("application/zip")
                .unwrap(),
        );

        let request = endpoint
            .request_builder(&self.http_client, &self.base_url)
            .multipart(form)
            .build()?;
        self.execute_request(request).await?.into_result_t()
    }

    /// Returns all hostnames across every application owned by the account.
    ///
    /// Each [`AppDomain`] entry contains the owning `app_id`, the
    /// `hostname`, and whether it is a `"subdomain"` or `"custom"` domain.
    /// Applications without a web-accessible domain (workers, bots) are
    /// excluded.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn all_domains(&self) -> Result<Vec<AppDomain>, ApiError> {
        self.request_endpoint(Endpoint::app_domains())
            .await?
            .into_result_t()
    }

    /// Returns the runtime status of every application owned by the account.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn all_apps_status(
        &self,
    ) -> Result<Vec<RuntimeStatsListItem>, ApiError> {
        self.request_endpoint(Endpoint::all_apps_status())
            .await?
            .into_result_t()
    }

    /// Provisions a new managed database and returns its full configuration.
    ///
    /// # Parameters
    ///
    /// - `name` — A human-readable name for the database.
    /// - `memory` — RAM allocation in megabytes.
    /// - `type` — The database engine ([`DatabaseType`]).
    /// - `version` — The engine version string (e.g. `"7.0"` for Redis).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with [`ApiErrorCode::InvalidMemory`] or
    /// [`ApiErrorCode::FewMemory`] if the memory allocation is not permitted,
    /// or [`ApiError::Transport`] on network failure.
    pub async fn create_database(
        &self,
        name: String,
        memory: u32,
        r#type: DatabaseType,
        version: String,
    ) -> Result<Database, ApiError> {
        let endpoint = Endpoint::create_database();
        let request = endpoint
            .request_builder(&self.http_client, &self.base_url)
            .json(&json!({
                "name": name,
                "memory": memory,
                "type": r#type,
                "version": version}))
            .build()?;
        self.execute_request(request).await?.into_result_t()
    }

    /// Returns a summary of the runtime status for every database owned by
    /// the account.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn all_database_status(
        &self,
    ) -> Result<Vec<RuntimeStatsListItem>, ApiError> {
        self.request_endpoint(Endpoint::all_database_status())
            .await?
            .into_result_t()
    }

    /// Creates a new workspace with the given display name.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] if the name is invalid or the account has
    /// reached its workspace quota, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn create_workspace(
        &self,
        name: String,
    ) -> Result<WorkspaceInfo, ApiError> {
        let endpoint = Endpoint::create_workspace();
        let request = endpoint
            .request_builder(&self.http_client, &self.base_url)
            .json(&json!({"name": name}))
            .build()?;
        self.execute_request(request).await?.into_result_t()
    }

    /// Returns all snapshots owned by the account, optionally filtered by
    /// resource type.
    ///
    /// Pass `Some(SnapshotScope::Applications)` or
    /// `Some(SnapshotScope::Databases)` to restrict the results. Pass `None`
    /// to retrieve snapshots for all resource types.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn all_snapshots(
        &self,
        scope: Option<SnapshotScope>,
    ) -> Result<Vec<Snapshot>, ApiError> {
        self.request_endpoint(Endpoint::list_all_snapshots(scope))
            .await?
            .into_result_t()
    }

    /// Returns all workspaces the account belongs to.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn all_workspaces(
        &self,
    ) -> Result<Vec<WorkspaceInfo>, ApiError> {
        self.request_endpoint(Endpoint::list_workspaces())
            .await?
            .into_result_t()
    }
}
