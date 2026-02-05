use std::sync::Arc;

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
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
}
