use std::sync::Arc;

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
    types::{FileContent, FileInfo},
};

pub struct FileResource {
    pub path: String,
    pub app_id: String,
    api: Arc<ApiClient>,
}

impl FileResource {
    pub fn new(api: Arc<ApiClient>, path: &str, app_id: &str) -> Self {
        Self {
            api,
            app_id: app_id.to_string(),
            path: path.to_string(),
        }
    }

    pub async fn write(&self, content: &str) -> Result<bool, ApiError> {
        let endpoint =
            Endpoint::put_app_file(&self.app_id, &self.path, content);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }

    pub async fn read(&self, path: &str) -> Result<FileContent, ApiError> {
        self.api
            .request_endpoint(Endpoint::read_app_file(&self.app_id, path))
            .await?
            .into_result_t()
    }

    pub async fn delete(&self) -> Result<bool, ApiError> {
        let endpoint = Endpoint::delete_app_file(&self.app_id);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
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
    }

    pub fn find_by_path<'a>(
        files: &'a [FileResource],
        path: &'a str,
    ) -> Option<&'a FileResource> {
        files.iter().find(|file| file.path == path)
    }

    pub async fn all_files(
        &self,
        path: &str,
    ) -> Result<Vec<FileInfo>, ApiError> {
        let endpoint = Endpoint::list_app_files(&self.app_id, path);
        self.api.request_endpoint(endpoint).await?.into_result_t()
    }
}
