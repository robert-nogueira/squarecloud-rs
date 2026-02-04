use std::sync::Arc;

use crate::http::ApiClient;

pub struct DatabaseResource {
    http: Arc<ApiClient>,
}

pub enum DatabaseType {
    Redis,
    Postgres,
    Mongo,
}

impl DatabaseResource {
    pub fn new(http: Arc<ApiClient>) -> Self {
        Self { http }
    }
}
