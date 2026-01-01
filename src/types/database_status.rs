use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

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
