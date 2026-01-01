use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub date: DateTime<Utc>,
    pub cpu: f32,
    pub ram: f32,
    pub net: [u32; 2],
}
