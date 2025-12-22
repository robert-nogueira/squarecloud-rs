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

pub type AnalyticsVisits = Vec<AnalyticsItem>;
pub type AnalyticsCountries = Vec<AnalyticsItem>;
pub type AnalyticsDevices = Vec<AnalyticsItem>;
pub type AnalyticsOS = Vec<AnalyticsItem>;
pub type AnalyticsBrowsers = Vec<AnalyticsItem>;
pub type AnalyticsProtocols = Vec<AnalyticsItem>;
pub type AnalyticsMethods = Vec<AnalyticsItem>;
pub type AnalyticsPaths = Vec<AnalyticsItem>;
pub type AnalyticsReferers = Vec<AnalyticsItem>;
pub type AnalyticsProviders = Vec<AnalyticsItem>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Analytics {
    pub visits: AnalyticsVisits,
    pub countries: AnalyticsCountries,
    pub devices: AnalyticsDevices,
    pub os: AnalyticsOS,
    pub browsers: AnalyticsBrowsers,
    pub protocols: AnalyticsProtocols,
    pub methods: AnalyticsMethods,
    pub paths: AnalyticsPaths,
    pub referers: AnalyticsReferers,
    pub providers: AnalyticsProviders,
}
