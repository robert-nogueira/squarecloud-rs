use std::sync::Arc;

use crate::http::{ApiClient, Endpoint, errors::ApiError};

pub struct FileResource {
    pub path: String,
    pub app_id: String,
    api: Arc<ApiClient>,
}

impl FileResource {
    pub fn new(api: Arc<ApiClient>, path: String, app_id: &str) -> Self {
        Self {
            api,
            path,
            app_id: app_id.to_string(),
        }
    }

    pub async fn delete(&self) -> Result<bool, ApiError> {
        let endpoint = Endpoint::delete_app_file(&self.app_id);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn move_to(
        &self,
        destination_path: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::move_app_file(
            &self.app_id,
            &self.path,
            destination_path,
        );

        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }
}
