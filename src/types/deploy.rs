use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deploy {
    pub id: String,
    pub state: String,
    pub date: DateTime<Utc>,
}
