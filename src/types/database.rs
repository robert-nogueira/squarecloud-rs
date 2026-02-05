use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{http::ApiClient, resources::DatabaseResource};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub memory: u32,
    pub cpu: u8,
    #[serde(rename = "type")]
    pub db_type: String,
    pub password: String,
    pub certificate: String,
    pub connection_url: String,
}

impl Database {
    pub fn into_resource(&self, api: Arc<ApiClient>) -> DatabaseResource {
        DatabaseResource::new(api, &self.id)
    }
}
