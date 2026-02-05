use std::sync::Arc;

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
    types::{DatabaseMetrics, DatabaseStatus},
};

pub struct DatabaseResource {
    pub client: Arc<ApiClient>,
    pub id: String,
}

impl DatabaseResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::start_database(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::stop_database(&self.id))
            .await?
            .into_bool_result()
    }

    pub async fn status(&self) -> Result<DatabaseStatus, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_status(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn metrics(&self) -> Result<Vec<DatabaseMetrics>, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_metrics(&self.id))
            .await?
            .into_result_t()
    }
}
