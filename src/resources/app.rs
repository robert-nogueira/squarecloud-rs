use std::{borrow::Cow, sync::Arc};

use reqwest::multipart::{Form, Part};
use serde_json::{Value, json};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    http::{
        ApiClient, ApiResponse, Endpoint,
        errors::{ApiError, CommitError},
    },
    resources::file::FileResource,
    types::{Analytics, AppInfo, AppStatus, Deploy, DnsRecord, FileInfo},
};

pub struct AppResource {
    pub id: String,
    api: Arc<ApiClient>,
}

impl AppResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            api: http,
            id: id.to_string(),
        }
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_start(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn restart(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_stop(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn all_apps_status(&self) -> Result<Vec<AppStatus>, ApiError> {
        self.api
            .request_endpoint(Endpoint::all_apps_status())
            .await?
            .into_result_t()
    }

    pub async fn status(&self) -> Result<AppStatus, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_status(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn info(&self) -> Result<AppInfo, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_info(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn logs(&self) -> Result<String, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_logs(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn analytics(&self) -> Result<Analytics, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_analytics(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn dns_record(&self) -> Result<DnsRecord, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_dns_record(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn current_deploy(&self) -> Result<Deploy, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_current_app_deploy(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn list_deploys(&self) -> Result<Vec<Deploy>, ApiError> {
        self.api
            .request_endpoint(Endpoint::list_app_deploys(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn set_webhook_integration(
        &self,
        access_token: String,
    ) -> Result<String, ApiError> {
        let endpoint = Endpoint::set_webhook_integration(&self.id);
        let request = endpoint
            .request_builder(&self.api.http_client)
            .json(&json!({"access_token": access_token}))
            .build()?;
        let response: ApiResponse<Value> =
            self.api.execute_request(request).await?;
        let value = response.into_result_t()?;
        let webhook = value.get("webhook").and_then(Value::as_str).unwrap();
        Ok(webhook.to_string())
    }

    pub async fn commit(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<bool, CommitError> {
        let endpoint = Endpoint::app_commit(&self.id);
        let form = Form::new().part("file", Part::bytes(bytes));

        let request = endpoint
            .request_builder(&self.api.http_client)
            .multipart(form)
            .build()?;
        self.api
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
            .map_err(|error| CommitError::Api(error))
    }

    pub async fn commit_file(
        &self,
        mut file: File,
    ) -> Result<bool, CommitError> {
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).await?;
        self.commit(buffer).await
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
