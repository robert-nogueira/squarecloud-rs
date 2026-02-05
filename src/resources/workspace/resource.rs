use std::sync::Arc;

use crate::http::ApiClient;

pub struct WorkspaceResource {
    pub client: Arc<ApiClient>,
}

impl WorkspaceResource {
    pub fn new(http: Arc<ApiClient>) -> Self {
        Self { client: http }
    }
}
