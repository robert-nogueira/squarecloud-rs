use serde::{Deserialize, Serialize};

/// A single data point within an [`Analytics`] breakdown.
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsItem {
    /// The dimension label for this item (e.g. a country code, browser name,
    /// or HTTP method).
    #[serde(rename = "type")]
    pub item_type: String,
    /// Total number of unique visits attributed to this item.
    pub visits: u32,
    /// Total number of HTTP requests attributed to this item.
    pub requests: u32,
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
    pub visits: Vec<AnalyticsItem>,
    /// Traffic by visitor country.
    pub countries: Vec<AnalyticsItem>,
    /// Traffic by device category (mobile, desktop, tablet).
    pub devices: Vec<AnalyticsItem>,
    /// Traffic by operating system.
    pub os: Vec<AnalyticsItem>,
    /// Traffic by browser.
    pub browsers: Vec<AnalyticsItem>,
    /// Traffic by HTTP protocol version (HTTP/1.1, HTTP/2, HTTP/3).
    pub protocols: Vec<AnalyticsItem>,
    /// Traffic by HTTP method (GET, POST, etc.).
    pub methods: Vec<AnalyticsItem>,
    /// Traffic by request path.
    pub paths: Vec<AnalyticsItem>,
    /// Traffic by HTTP referrer.
    pub referers: Vec<AnalyticsItem>,
    /// Traffic by network provider.
    pub providers: Vec<AnalyticsItem>,
}
