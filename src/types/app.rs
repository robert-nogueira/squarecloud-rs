use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub id: String,
    pub owner: String,
    pub cluster: String,
    pub ram: u32,
    pub language: String,
    pub domain: Option<String>,
    pub custom: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppNetwork {
    pub total: String,
    pub now: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppStatus {
    pub cpu: String,
    pub ram: String,
    pub storage: String,
    pub network: AppNetwork,
    #[serde(with = "ts_milliseconds")]
    pub uptime: DateTime<Utc>,
}
