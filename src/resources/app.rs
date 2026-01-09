use std::{borrow::Cow, sync::Arc};

use reqwest::multipart::{Form, Part};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    http::{
        ApiClient, Endpoint,
        errors::{ApiError, CommitError},
    },
    types::{
        analytics::Analytics,
        app::{AppInfo, AppStatus},
        deploy::Deploy,
        dns_record::DnsRecord,
    },
};

use super::file::FileResource;

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

    pub async fn file(self, path: &str) -> FileResource {
        let api_clone = Arc::clone(&self.api);
        FileResource::new(api_clone, path.to_string(), &self.id)
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_start(&self.id))
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn restart(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::app_stop(&self.id))
            .await?
            .into_bool_result()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn all_apps_status(&self) -> Result<Vec<AppStatus>, ApiError> {
        self.api
            .request_endpoint(Endpoint::all_apps_status())
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn status(&self) -> Result<AppStatus, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_status(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn info(&self) -> Result<AppInfo, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_info(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn logs(&self) -> Result<String, ApiError> {
        self.api
            .request_endpoint(Endpoint::app_logs(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn analytics(&self) -> Result<Analytics, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_analytics(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn dns_record(&self) -> Result<DnsRecord, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_dns_record(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
    }

    pub async fn current_deploy(&self) -> Result<Deploy, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_current_app_deploy(&self.id))
            .await?
            .into_result_t()
            .map_err(|code| ApiError::Api { code })
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
            .map_err(|code| CommitError::Api(ApiError::Api { code }))
    }

    pub async fn commit_file(
        &self,
        mut file: File,
    ) -> Result<bool, CommitError> {
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).await?;
        self.commit(buffer).await
    }
}
