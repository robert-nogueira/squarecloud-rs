use std::sync::Arc;

use crate::http::ApiClient;

pub struct SnapshotResource {
    http: Arc<ApiClient>,
}
impl SnapshotResource {
    pub fn new(http: Arc<ApiClient>) -> Self {
        Self { http }
    }
}
