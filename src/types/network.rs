use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single edge-network request log entry.
///
/// Returned as part of a [`Vec`] by
/// [`AppResource::network_logs`](crate::resources::AppResource::network_logs).
///
/// Requires a Pro or Enterprise plan. The API retains logs for a maximum of
/// 7 days.
#[derive(Serialize, Deserialize)]
pub struct NetworkLogEntry {
    /// The UTC timestamp of the request.
    pub timestamp: DateTime<Utc>,
    /// The client's IP address, if available.
    pub ip: Option<String>,
    /// The client's ISO 3166-1 alpha-2 country code, if known.
    pub country: Option<String>,
    /// An approximate geographic location string, if known.
    pub location: Option<String>,
    /// The client's Autonomous System Number.
    pub asn: String,
    /// The `User-Agent` header value, if present.
    pub agent: Option<String>,
    /// Classification of the client (e.g. `"bot"` or `"human"`).
    pub category: Option<String>,
    /// Whether the request was blocked or altered by edge protection.
    pub mitigated: bool,
    /// The HTTP method (e.g. `"GET"`, `"POST"`).
    pub method: String,
    /// The `Host` header value.
    pub host: String,
    /// The request URI path.
    pub path: String,
    /// The raw query string, if present.
    pub query: Option<String>,
    /// The HTTP protocol version (e.g. `"HTTP/2"`).
    pub protocol: String,
    /// The `Referer` header value, if present.
    pub referer: Option<String>,
    /// The HTTP response status code.
    pub status: u16,
    /// The `Content-Type` of the response, if present.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// The edge cache result (e.g. `"HIT"` or `"MISS"`), if applicable.
    pub cache: Option<String>,
}
