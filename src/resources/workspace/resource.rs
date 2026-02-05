use std::sync::Arc;

use crate::http::ApiClient;

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
}
