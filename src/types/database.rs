use std::sync::Arc;

use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

use crate::{http::ApiClient, resources::DatabaseResource};

#[derive(Serialize, Deserialize)]
pub struct DatabaseNetwork {
    pub total: String,
    pub now: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseResumedStatus {
    pub id: String,
    pub running: bool,
    pub cpu: String,
    pub ram: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub cpu: String,
    pub ram: String,
    pub status: String,
    pub network: DatabaseNetwork,
    #[serde(with = "ts_milliseconds")]
    pub uptime: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub date: DateTime<Utc>,
    pub cpu: f32,
    pub ram: f32,
    pub net: [u32; 2],
}

#[derive(Serialize, Deserialize)]
pub enum DatabaseType {
    Redis,
    Postgres,
    Mongo,
}

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
