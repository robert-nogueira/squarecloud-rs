use std::sync::Arc;

use crate::{
    http::{ApiClient, Endpoint, errors::ApiError},
    types::file::File,
};

pub struct FileResource {
    pub app_id: String,
    pub path: String,
    api: Arc<ApiClient>,
}

impl FileResource {
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
    ) -> Result<File, ApiError> {
        let endpoint = Endpoint::move_app_file(
            &self.app_id,
            &self.path,
            destination_path,
        );

        self.api
            .request_endpoint(endpoint)
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }
}
