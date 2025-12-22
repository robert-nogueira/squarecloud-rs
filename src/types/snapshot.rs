use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    name: String,
    size: u64,
    modified: DateTime<Utc>,
    key: String,
}
