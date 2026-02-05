use std::sync::Arc;

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
};

pub struct DatabaseResource {
    api: Arc<ApiClient>,
    id: String,
}

impl DatabaseResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            api: http,
            id: id.to_string(),
        }
    }

    pub async fn start(&self) -> Result<bool, ApiError> {
        self.api
            .request_endpoint::<()>(Endpoint::start_database(&self.id))
            .await?
            .into_bool_result()
    }
}
