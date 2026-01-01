use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub name: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub key: String,
}
