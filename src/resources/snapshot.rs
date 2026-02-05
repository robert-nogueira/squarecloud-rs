use std::sync::Arc;

use crate::http::ApiClient;

pub struct SnapshotResource {
    pub client: Arc<ApiClient>,
}
impl SnapshotResource {
    pub fn new(http: Arc<ApiClient>) -> Self {
        Self { client: http }
    }
}
