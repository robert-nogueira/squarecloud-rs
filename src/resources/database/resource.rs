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

    pub fn create(
        &self,
        name: &str,
        memory: u32,
        db_type: &str,
        version: &str,
    ) {
    }
}
