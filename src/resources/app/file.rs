use crate::{
    Endpoint,
    http::errors::ApiError,
    resources::FileResource,
    types::{FileContent, FileInfo},
};

use super::AppResource;

impl AppResource {
    pub async fn read_file(
        &self,
        path: &str,
    ) -> Result<FileContent, ApiError> {
        self.api
            .request_endpoint(Endpoint::read_app_file(&self.id, path))
            .await?
            .into_result_t()
    }

    pub async fn file_list(
        &self,
        path: &str,
    ) -> Result<Vec<FileResource>, ApiError> {
        let endpoint = Endpoint::list_app_files(&self.id, path);
        let files: Vec<FileInfo> =
            self.api.request_endpoint(endpoint).await?.into_result_t()?;
        let mut file_resources: Vec<FileResource> = vec![];
        for file in files {
            file_resources.push(FileResource::new(
                self.api.clone(),
                path,
                &self.id,
                file,
            ));
        }
        Ok(file_resources)
    }

    pub async fn create_file(
        &self,
        path: &str,
        content: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::put_app_file(&self.id, path, content);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }

    pub async fn delete(&self) -> Result<bool, ApiError> {
        let endpoint = Endpoint::app_delete(&self.id);
        self.api
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }
}
