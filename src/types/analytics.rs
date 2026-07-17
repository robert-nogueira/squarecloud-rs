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
    /// Traffic by client IP address.
    #[serde(default)]
    pub ips: Vec<AnalyticsItem>,
    /// Traffic by HTTP response status code.
    #[serde(default)]
    pub status_codes: Vec<AnalyticsItem>,
    /// Traffic by bot category (e.g. search engine crawlers).
    #[serde(default)]
    pub bots: Vec<AnalyticsItem>,
    /// Traffic by response content type.
    #[serde(default)]
    pub content_types: Vec<AnalyticsItem>,
}

/// Optional drill-down filters for
/// [`AppResource::analytics_filtered`](crate::resources::AppResource::analytics_filtered).
///
/// Every set filter is applied to all breakdowns at once. Build with
/// chained setters; unset filters are omitted from the request:
///
/// ```
/// use squarecloud::types::AnalyticsFilters;
///
/// let filters = AnalyticsFilters::new().country("BR").status("404");
/// ```
#[derive(Debug, Clone, Default)]
pub struct AnalyticsFilters {
    /// Two-letter country code (e.g. `"BR"`).
    pub country: Option<String>,
    /// Client IP address.
    pub ip: Option<String>,
    /// Request path (e.g. `"/api"`).
    pub path: Option<String>,
    /// HTTP status code as a string (e.g. `"404"`).
    pub status: Option<String>,
    /// Operating system name (e.g. `"Windows"`).
    pub os: Option<String>,
    /// Browser name (e.g. `"Chrome"`).
    pub browser: Option<String>,
    /// HTTP protocol version (e.g. `"HTTP/2"`).
    pub protocol: Option<String>,
    /// HTTP referrer (e.g. `"google.com"`).
    pub referer: Option<String>,
    /// Network provider (e.g. `"GOOGLE (15169)"`).
    pub provider: Option<String>,
    /// Response content type (e.g. `"json"`).
    pub content_type: Option<String>,
    /// Bot category (e.g. `"Search Engine Crawler"`).
    pub bot: Option<String>,
}

impl AnalyticsFilters {
    /// Creates an empty filter set (no drill-down applied).
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters every breakdown to this country.
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Filters every breakdown to this ip.
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Filters every breakdown to this path.
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Filters every breakdown to this status.
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Filters every breakdown to this os.
    pub fn os(mut self, value: impl Into<String>) -> Self {
        self.os = Some(value.into());
        self
    }

    /// Filters every breakdown to this browser.
    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    /// Filters every breakdown to this protocol.
    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    /// Filters every breakdown to this referer.
    pub fn referer(mut self, value: impl Into<String>) -> Self {
        self.referer = Some(value.into());
        self
    }

    /// Filters every breakdown to this provider.
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Filters every breakdown to this content type.
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    /// Filters every breakdown to this bot.
    pub fn bot(mut self, value: impl Into<String>) -> Self {
        self.bot = Some(value.into());
        self
    }

    pub(crate) fn entries(&self) -> [(&'static str, Option<&str>); 11] {
        [
            ("country", self.country.as_deref()),
            ("ip", self.ip.as_deref()),
            ("path", self.path.as_deref()),
            ("status", self.status.as_deref()),
            ("os", self.os.as_deref()),
            ("browser", self.browser.as_deref()),
            ("protocol", self.protocol.as_deref()),
            ("referer", self.referer.as_deref()),
            ("provider", self.provider.as_deref()),
            ("content_type", self.content_type.as_deref()),
            ("bot", self.bot.as_deref()),
        ]
    }
}
