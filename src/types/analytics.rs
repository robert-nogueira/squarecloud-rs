use serde::{Deserialize, Serialize};

/// A single data point within an [`Analytics`] breakdown.
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsItem {
    /// The dimension label for this item (e.g. a country code, browser name,
    /// or HTTP method).
    #[serde(rename = "type")]
    pub item_type: String,
    /// Total number of unique visits attributed to this item.
    pub visits: u64,
    /// Total number of HTTP requests attributed to this item.
    pub requests: u64,
    /// Total bytes transferred for this item.
    pub bytes: u64,
    /// The time period these metrics cover, as an ISO 8601 date string.
    pub date: String,
}

/// Edge-network analytics for an application, broken down by multiple
/// dimensions.
///
/// Returned by [`AppResource::analytics`](crate::resources::AppResource::analytics).
#[derive(Debug, Deserialize, Serialize)]
pub struct Analytics {
    /// Traffic over time.
    #[serde(default)]
    pub visits: Vec<AnalyticsItem>,
    /// Traffic by visitor country.
    #[serde(default)]
    pub countries: Vec<AnalyticsItem>,
    /// Traffic by device category (mobile, desktop, tablet).
    #[serde(default)]
    pub devices: Vec<AnalyticsItem>,
    /// Traffic by operating system.
    #[serde(default)]
    pub os: Vec<AnalyticsItem>,
    /// Traffic by browser.
    #[serde(default)]
    pub browsers: Vec<AnalyticsItem>,
    /// Traffic by HTTP protocol version (HTTP/1.1, HTTP/2, HTTP/3).
    #[serde(default)]
    pub protocols: Vec<AnalyticsItem>,
    /// Traffic by HTTP method (GET, POST, etc.).
    #[serde(default)]
    pub methods: Vec<AnalyticsItem>,
    /// Traffic by request path.
    #[serde(default)]
    pub paths: Vec<AnalyticsItem>,
    /// Traffic by HTTP referrer.
    #[serde(default)]
    pub referers: Vec<AnalyticsItem>,
    /// Traffic by network provider.
    #[serde(default)]
    pub providers: Vec<AnalyticsItem>,
}
