use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deploy {
    id: String,
    state: String,
    date: DateTime<Utc>,
}
