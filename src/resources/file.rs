use std::sync::Arc;

use crate::{
    http::{ApiClient, Endpoint, errors::ApiError},
    types::file::FileInfo,
};

pub struct FileResource {
    pub path: String,
    pub info: FileInfo,
    pub app_id: String,
    api: Arc<ApiClient>,
}

impl FileResource {
    pub fn new(
        api: Arc<ApiClient>,
        path: &str,
        app_id: &str,
        info: FileInfo,
    ) -> Self {
        Self {
            api,
            info,
            app_id: app_id.to_string(),
            path: path.to_string(),
        }
    }

    pub async fn write_content(
        &self,
        content: &str,
    ) -> Result<bool, ApiError> {
        let endpoint =
            Endpoint::put_app_file(&self.app_id, &self.path, content);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn read(&self) -> Result<Vec<u8>, ApiError> {
        let endpoint = Endpoint::read_app_file(&self.app_id, &self.path);
        self.api
            .request_endpoint(endpoint)
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
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

    pub fn find_by_path<'a>(
        files: &'a Vec<FileResource>,
        path: &str,
    ) -> Option<&'a FileResource> {
        let mut file = files.iter().filter(|file| file.path == path);
        file.next()
    }
}
