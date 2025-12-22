use chrono::{DateTime, Utc};

pub struct DatabaseMetrics {
    date: DateTime<Utc>,
    cpu: f32,
    ram: f32,
    net: [u32; 2],
}
