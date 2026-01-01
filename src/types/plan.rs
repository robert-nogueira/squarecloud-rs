use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanMemory {
    pub limit: u64,
    pub available: u64,
    pub used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub memory: PlanMemory,
    #[serde(with = "ts_milliseconds")]
    pub duration: DateTime<Utc>,
}
