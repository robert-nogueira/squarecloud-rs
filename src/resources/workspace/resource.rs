use std::sync::Arc;

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
};

pub struct WorkspaceResource {
    pub client: Arc<ApiClient>,
    pub id: String,
}

impl WorkspaceResource {
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    pub async fn delete(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::delete_workspace())
            .await?
            .into_bool_result()
    }
}
