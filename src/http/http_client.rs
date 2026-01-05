use std::sync::Arc;

use reqwest::{
    Client, Request,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::{
    Endpoint,
    errors::{ApiError, ApiErrorCode},
};
use crate::{
    resources::{
        app::AppResource, database::DatabaseResource,
        snapshot::SnapshotResource, workspace::WorkspaceResource,
    },
    settings::SETTINGS,
    types::account_info::AccountInfo,
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
    pub fn into_result_t(self) -> Result<T, ApiErrorCode> {
        match self {
            ApiResponse::Error { code, .. } => Err(code),
            ApiResponse::Success { response, .. } => {
                response.ok_or_else(|| panic!("Expected response data"))
            }
        }
    }
    pub fn into_bool_result(self) -> Result<bool, ApiErrorCode> {
        match self {
            ApiResponse::Success { success, .. } => Ok(success),
            ApiResponse::Error { code, .. } => Err(code),
        }
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
        self.request_endpoint(Endpoint::me())
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn app(self, id: &str) -> AppResource {
        AppResource::new(Arc::new(self), id)
    }

    pub async fn snapshot(self) -> SnapshotResource {
        SnapshotResource::new(Arc::new(self))
    }

    pub async fn workspace(self) -> WorkspaceResource {
        WorkspaceResource::new(Arc::new(self))
    }

    pub async fn database(self) -> DatabaseResource {
        DatabaseResource::new(Arc::new(self))
    }
}
