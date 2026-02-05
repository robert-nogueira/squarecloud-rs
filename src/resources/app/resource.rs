use std::{borrow::Cow, sync::Arc};

use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        ApiClient,
        errors::{ApiError, CommitError},
    },
    resources::FileResource,
    types::{AppInfo, AppStatus},
};

pub struct AppResource {
    pub id: String,
    pub client: Arc<ApiClient>,
}

impl AppResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    pub fn file(&self, path: &str) -> FileResource {
        FileResource::new(self.client.clone(), path, &self.id)
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_start(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn restart(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_stop(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn status(&self) -> Result<AppStatus, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_status(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn info(&self) -> Result<AppInfo, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_info(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn logs(&self) -> Result<String, ApiError> {
        self.client
            .request_endpoint(Endpoint::app_logs(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn commit(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<bool, CommitError> {
        let endpoint = Endpoint::app_commit(&self.id);
        let form = Form::new().part("file", Part::bytes(bytes));

        let request = endpoint
            .request_builder(&self.client.http_client)
            .multipart(form)
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
            .map_err(|error| CommitError::Api(error))
    }

    pub async fn delete(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::app_delete(&self.id))
            .await?
            .into_bool_result()
    }
}
