use std::sync::Arc;

use crate::http::ApiClient;

pub struct WorkspaceResource {
    http: Arc<ApiClient>,
}
impl WorkspaceResource {
    pub fn new(http: Arc<ApiClient>) -> Self {
        Self { http }
    }
}
