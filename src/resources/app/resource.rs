use std::{borrow::Cow, sync::Arc};

use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        ApiClient,
        errors::{ApiError, CommitError},
    },
    resources::FileResource,
    types::{AppInfo, AppLogs, AppMetrics, RuntimeStats},
};

/// A handle to a specific SquareCloud application.
///
/// Obtain an `AppResource` by calling [`ApiClient::app`] with the application
/// ID, or via [`AppInfo::into_resource`](crate::types::AppInfo::into_resource).
/// The handle holds the underlying [`ApiClient`] behind an [`Arc`], so it is
/// cheap to clone and can be shared across tasks.
///
/// Methods are spread across multiple `impl` blocks in submodules:
///
/// | Source file | Methods |
/// |-------------|---------|
/// | `resource.rs` | lifecycle, status, logs, commit, delete |
/// | `deploy.rs` | deploy history, webhook integration |
/// | `env.rs` | environment variable management |
/// | `network.rs` | analytics, DNS, custom domain, cache purge |
/// | `snaphots.rs` | snapshot management |
pub struct AppResource {
    /// The application's unique identifier.
    pub id: String,
    /// Shared reference to the underlying HTTP client.
    pub client: Arc<ApiClient>,
}

impl AppResource {
    /// Creates a new `AppResource` bound to the given client and application
    /// ID.
    ///
    /// Prefer [`ApiClient::app`] over calling this directly.
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    /// Returns a [`FileResource`] scoped to the given path within this
    /// application's filesystem.
    pub fn file(&self, path: &str) -> FileResource {
        FileResource::new(self.client.clone(), path, &self.id)
    }

    /// Starts the application.
    ///
    /// Returns `Ok(true)` when the start command is accepted by the platform.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn start(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_start(&self.id))
            .await?
            .into_bool_result()
    }

    /// Restarts the application.
    ///
    /// The application is stopped and then started again. Returns `Ok(true)`
    /// when the restart command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn restart(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
    }

    /// Stops the application.
    ///
    /// Returns `Ok(true)` when the stop command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_stop(&self.id))
            .await?
            .into_bool_result()
    }

    /// Returns the current runtime status of the application.
    ///
    /// The returned [`RuntimeStats`] includes CPU usage, RAM usage, storage,
    /// network throughput, and the time the process last started.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn status(&self) -> Result<RuntimeStats, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_status(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns static metadata about the application.
    ///
    /// The returned [`AppInfo`] includes the display name, owner ID, cluster,
    /// memory allocation, and language runtime. Unlike
    /// [`status`](AppResource::status), this does not reflect whether the
    /// application is currently running.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn info(&self) -> Result<AppInfo, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_info(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns historical resource-usage metrics for the application.
    ///
    /// Each [`AppMetrics`] entry covers a 5-minute window. Up to 288 data
    /// points (24 hours) are returned. Results are cached for 2.5 minutes
    /// per application.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn metrics(&self) -> Result<Vec<AppMetrics>, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_metrics(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns the most recent log output from the application as a plain
    /// string.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn logs(&self) -> Result<String, ApiError> {
        let r: AppLogs = self
            .client
            .request_endpoint(Endpoint::app_logs(&self.id))
            .await?
            .into_result_t()?;
        Ok(r.logs)
    }

    /// Commits a new version of the application by uploading a ZIP archive.
    ///
    /// The archive is sent as a `multipart/form-data` request. The `bytes`
    /// parameter accepts anything that converts to a `Cow<'static, [u8]>`,
    /// such as a `Vec<u8>`.
    ///
    /// Unlike [`ApiClient::upload_app`](crate::ApiClient::upload_app),
    /// `commit` updates an existing application in place rather than creating
    /// a new one.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::Io`] if constructing the multipart request
    /// fails, [`CommitError::Api`] wrapping [`ApiError::Transport`] on network
    /// failure, or [`CommitError::Api`] wrapping [`ApiError::Api`] if the
    /// archive is rejected by the API.
    pub async fn commit(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<bool, CommitError> {
        let endpoint = Endpoint::app_commit(&self.id);
        let form = Form::new().part(
            "file",
            Part::bytes(bytes)
                .file_name("app.zip")
                .mime_str("application/zip")
                .unwrap(),
        );

        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .multipart(form)
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
            .map_err(|error| CommitError::Api(error))
    }

    /// Permanently deletes the application and all associated data.
    ///
    /// This action cannot be undone. Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn delete(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_delete(&self.id))
            .await?
            .into_bool_result()
    }
}
