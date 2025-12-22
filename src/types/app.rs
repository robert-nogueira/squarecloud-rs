use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
    name: String,
    id: String,
    owner: String,
    cluster: String,
    ram: u32,
    language: String,
    domain: Option<String>,
    custom: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppNetwork {
    total: String,
    now: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppStatus {
    cpu: String,
    ram: String,
    storage: String,
    network: AppNetwork,
    #[serde(with = "ts_milliseconds")]
    uptime: DateTime<Utc>,
}
