use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Error counts broken down by HTTP class (4xx / 5xx).
///
/// Part of [`NetworkErrorSummary`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrorByClass {
    /// Number of 4xx client errors. Present only when `include_4xx` was
    /// set on the request.
    #[serde(rename = "4xx")]
    pub client_errors: Option<u64>,
    /// Number of 5xx server errors.
    #[serde(rename = "5xx")]
    pub server_errors: u64,
}

/// High-level totals for edge errors in the analysis window.
///
/// Part of [`NetworkErrors`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrorSummary {
    /// Total error count across all classes.
    pub total: u64,
    /// Breakdown by HTTP error class.
    pub by_class: NetworkErrorByClass,
}

/// Error count for a single HTTP status code.
///
/// Used in [`NetworkErrors::by_status`] and
/// [`NetworkErrorPath::by_status`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrorByStatus {
    /// The HTTP status code.
    pub status: u16,
    /// Number of requests that received this status code.
    pub total: u64,
}

/// Error counts within a single time bucket.
///
/// Part of [`NetworkErrors::timeseries`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrorTimeseries {
    /// The UTC timestamp that opens this time bucket.
    pub date: DateTime<Utc>,
    /// Error count per status code within this bucket, keyed by the status
    /// code as a string (e.g. `"502"`).
    pub buckets: HashMap<String, u64>,
    /// Total errors across all status codes in this bucket.
    pub total: u64,
}

/// Error statistics for a single request path.
///
/// Part of [`NetworkErrors::top_paths`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrorPath {
    /// The request URI path.
    pub path: String,
    /// The HTTP method used (e.g. `"GET"`).
    pub method: String,
    /// Total errors on this path.
    pub total: u64,
    /// Breakdown by individual status code.
    pub by_status: Vec<NetworkErrorByStatus>,
}

/// Edge-network error analytics for an application.
///
/// Returned by
/// [`AppResource::network_errors`](crate::resources::AppResource::network_errors).
///
/// Requires a Pro or Enterprise plan.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkErrors {
    /// Aggregate totals for the analysis window.
    pub summary: NetworkErrorSummary,
    /// Error count per individual HTTP status code.
    pub by_status: Vec<NetworkErrorByStatus>,
    /// Time-bucketed error counts across the analysis window.
    pub timeseries: Vec<NetworkErrorTimeseries>,
    /// Most error-prone request paths.
    pub top_paths: Vec<NetworkErrorPath>,
    /// Error totals aggregated by HTTP method. Structure varies by API
    /// version; use [`Value`] for forward compatibility.
    pub by_method: Value,
}

/// Edge and origin latency percentiles in milliseconds.
///
/// Used in [`NetworkPerformanceSummary`] and
/// [`NetworkPerformanceTimeseries`].
#[derive(Debug, Serialize, Deserialize)]
pub struct LatencyPercentiles {
    /// Median (50th percentile) latency in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// 99th percentile latency in milliseconds.
    pub p99: u32,
}

/// Aggregate latency summary for the analysis window.
///
/// Part of [`NetworkPerformance`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformanceSummary {
    /// Edge (CDN) latency percentiles.
    pub edge: LatencyPercentiles,
    /// Origin (application server) latency percentiles.
    pub origin: LatencyPercentiles,
    /// Total request count in the analysis window.
    pub requests: u64,
}

/// Latency data for a single time bucket.
///
/// Part of [`NetworkPerformance::timeseries`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformanceTimeseries {
    /// The UTC timestamp that opens this time bucket.
    pub date: DateTime<Utc>,
    /// Number of requests in this bucket.
    pub requests: u64,
    /// Edge latency percentiles for this bucket.
    pub edge: LatencyPercentiles,
    /// Origin latency percentiles for this bucket.
    pub origin: LatencyPercentiles,
}

/// Per-country latency statistics.
///
/// Part of [`NetworkPerformance::countries`]. The `country_code` field
/// holds the ISO 3166-1 alpha-2 country code (JSON key `"type"`).
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformanceCountry {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "type")]
    pub country_code: String,
    /// Median latency for requests from this country in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// Number of requests from this country.
    pub requests: u64,
}

/// Per-datacenter (colo) latency statistics.
///
/// Part of [`NetworkPerformance::colos`]. The `colo_id` field holds the
/// Cloudflare datacenter identifier (JSON key `"type"`).
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformanceColo {
    /// Cloudflare datacenter identifier (e.g. `"GRU"`).
    #[serde(rename = "type")]
    pub colo_id: String,
    /// City where the datacenter is located.
    pub city: String,
    /// Country where the datacenter is located.
    pub country: String,
    /// Median latency for requests handled by this colo in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// Number of requests handled by this colo.
    pub requests: u64,
}

/// Latency statistics for a single high-latency request path.
///
/// Part of [`NetworkPerformance::slowest_paths`].
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformancePath {
    /// The request URI path.
    pub path: String,
    /// 95th percentile latency for this path in milliseconds.
    pub p95: u32,
    /// 99th percentile latency for this path in milliseconds.
    pub p99: u32,
    /// Total number of requests to this path.
    pub requests: u64,
}

/// Edge-network latency performance analytics for an application.
///
/// Returned by
/// [`AppResource::network_performance`](crate::resources::AppResource::network_performance).
///
/// Requires a Pro or Enterprise plan. Rate-limited to 10 requests per
/// 60 seconds per owner for cache misses.
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPerformance {
    /// Aggregate latency summary for the analysis window.
    pub summary: NetworkPerformanceSummary,
    /// Time-bucketed latency data across the analysis window.
    pub timeseries: Vec<NetworkPerformanceTimeseries>,
    /// Per-country latency breakdown.
    pub countries: Vec<NetworkPerformanceCountry>,
    /// Per-datacenter latency breakdown.
    pub colos: Vec<NetworkPerformanceColo>,
    /// Highest-latency request paths.
    pub slowest_paths: Vec<NetworkPerformancePath>,
}

/// A single edge-network request log entry.
///
/// Returned as part of a [`Vec`] by
/// [`AppResource::network_logs`](crate::resources::AppResource::network_logs).
///
/// Requires a Pro or Enterprise plan. The API retains logs for a maximum of
/// 7 days.
#[derive(Debug, Serialize, Deserialize)]
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

/// One custom domain and the applications serving it.
///
/// Part of [`LoadBalancers`]. A group with two or more applications is an
/// active load balancer: traffic is balanced across them at the edge,
/// with automatic failover when an application is offline.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalancer {
    /// The custom domain.
    pub hostname: String,
    /// Applications serving this domain.
    pub apps: Vec<LoadBalancerApp>,
}

/// One application inside a [`LoadBalancer`] group.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalancerApp {
    /// The application's unique identifier.
    pub id: String,
    /// The application's name.
    pub name: String,
    /// The cluster currently hosting the application.
    pub cluster: Option<String>,
}

/// The account's custom domains grouped by hostname.
///
/// Returned by [`Client::load_balancers`](crate::Client::load_balancers).
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalancers {
    /// Maximum applications allowed on one domain for the account's plan:
    /// 2 on Standard, 5 on Pro, 10 on Enterprise.
    pub limit: u8,
    /// Custom domains grouped by hostname. Empty when no application has
    /// a custom domain attached.
    pub balancers: Vec<LoadBalancer>,
}
