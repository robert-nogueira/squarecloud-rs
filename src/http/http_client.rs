use std::{borrow::Cow, sync::Arc};

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
        AccountInfo, AppStatus, Database, DatabaseResumedStatus, DatabaseType,
    },
};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        response: Option<T>,
    },
    Error {
        success: bool,
        code: ApiErrorCode,
    },
}

impl<T> ApiResponse<T> {
    pub fn into_result_t(self) -> Result<T, ApiError> {
        match self {
            ApiResponse::Error { code, .. } => Err(ApiError::Api { code }),
            ApiResponse::Success { response, .. } => {
                response.ok_or_else(|| panic!("Expected response data"))
            }
        }
    }
    pub fn into_bool_result(self) -> Result<bool, ApiError> {
        match self {
            ApiResponse::Success { success, .. } => Ok(success),
            ApiResponse::Error { code, .. } => Err(ApiError::Api { code }),
        }
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ApiClient {
    pub base_url: String,
    pub(crate) http_client: Client,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&SETTINGS.api_token).unwrap(),
        );
        let client: Client =
            Client::builder().default_headers(headers).build().unwrap();
        ApiClient {
            base_url: SETTINGS.base_url.clone(),
            http_client: client,
        }
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
        let response = self
            .http_client
            .request(endpoint.method, endpoint.path)
            .send()
            .await?;
        let response: ApiResponse<T> = response.json().await?;
        Ok(response)
    }

    pub async fn me(&self) -> Result<AccountInfo, ApiError> {
        self.request_endpoint(Endpoint::me()).await?.into_result_t()
    }

    pub async fn upload_app(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::upload_app();
        let form = Form::new().part("file", Part::bytes(bytes));

        let request = endpoint
            .request_builder(&self.http_client)
            .multipart(form)
            .build()?;
        self.execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    pub async fn all_apps_status(&self) -> Result<Vec<AppStatus>, ApiError> {
        self.request_endpoint(Endpoint::all_apps_status())
            .await?
            .into_result_t()
    }

    pub async fn create_database(
        &self,
        name: String,
        memory: u32,
        r#type: DatabaseType,
        version: String,
    ) -> Result<Database, ApiError> {
        let endpoint = Endpoint::create_database();
        let request = endpoint
            .request_builder(&self.http_client)
            .json(&json!({
                "name": name,
                "memory": memory,
                "type": r#type,
                "version": version}))
            .build()?;
        self.execute_request(request).await?.into_result_t()
    }

    pub async fn all_database_status(
        &self,
    ) -> Result<Vec<DatabaseResumedStatus>, ApiError> {
        self.request_endpoint(Endpoint::all_database_status())
            .await?
            .into_result_t()
    }

    pub async fn app(self, id: &str) -> AppResource {
        AppResource::new(Arc::new(self), id)
    }

    pub async fn workspace(self) -> WorkspaceResource {
        WorkspaceResource::new(Arc::new(self))
    }

    pub async fn database(self, id: &str) -> DatabaseResource {
        DatabaseResource::new(Arc::new(self), id)
    }
}
