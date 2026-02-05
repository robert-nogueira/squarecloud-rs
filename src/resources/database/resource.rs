use std::sync::Arc;

use crate::http::ApiClient;

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
}
