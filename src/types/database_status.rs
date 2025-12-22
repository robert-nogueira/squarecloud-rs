use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DatabaseNetwork {
    total: String,
    now: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseResumedStatus {
    id: String,
    running: bool,
    cpu: String,
    ram: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseStatus {
    cpu: String,
    ram: String,
    status: String,
    network: DatabaseNetwork,
    #[serde(with = "ts_milliseconds")]
    uptime: DateTime<Utc>,
}
