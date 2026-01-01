use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub visits: u32,
    pub requests: u32,
    pub bytes: u64,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Analytics {
    pub visits: Vec<AnalyticsItem>,
    pub countries: Vec<AnalyticsItem>,
    pub devices: Vec<AnalyticsItem>,
    pub os: Vec<AnalyticsItem>,
    pub browsers: Vec<AnalyticsItem>,
    pub protocols: Vec<AnalyticsItem>,
    pub methods: Vec<AnalyticsItem>,
    pub paths: Vec<AnalyticsItem>,
    pub referers: Vec<AnalyticsItem>,
    pub providers: Vec<AnalyticsItem>,
}
