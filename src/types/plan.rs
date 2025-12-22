use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanMemory {
    limit: u64,
    available: u64,
    used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Plan {
    name: String,
    memory: PlanMemory,
    #[serde(with = "ts_milliseconds")]
    duration: DateTime<Utc>,
}
