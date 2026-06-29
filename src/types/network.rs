use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Error counts broken down by HTTP class (4xx / 5xx).
///
/// Part of [`NetworkErrorSummary`].
#[derive(Serialize, Deserialize)]
pub struct NetworkErrorByClass {
    /// Number of 4xx client errors.
    #[serde(rename = "4xx")]
    pub client_errors: u64,
    /// Number of 5xx server errors.
    #[serde(rename = "5xx")]
    pub server_errors: u64,
}

/// High-level totals for edge errors in the analysis window.
///
/// Part of [`NetworkErrors`].
#[derive(Serialize, Deserialize)]
pub struct NetworkErrorSummary {
    /// Total error count across all classes.
    pub total: u64,
    /// Breakdown by HTTP error class.
    pub by_class: NetworkErrorByClass,
}

/// Error count for a single HTTP status code.
///
/// Part of [`NetworkErrors::by_status`].
#[derive(Serialize, Deserialize)]
pub struct NetworkErrorByStatus {
    /// The HTTP status code.
    pub status: u16,
    /// Number of requests that received this status code.
    pub requests: u64,
}

/// Error counts within a single time bucket.
///
/// Part of [`NetworkErrors::timeseries`].
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct NetworkErrorPath {
    /// The request URI path.
    pub path: String,
    /// The HTTP method used (e.g. `"GET"`), or `None` if unknown.
    pub method: Option<String>,
    /// Total errors on this path.
    pub total: u64,
    /// Error count per status code, keyed by the status code as a string.
    pub by_status: HashMap<String, u64>,
}

/// Error counts aggregated by HTTP method.
///
/// Part of [`NetworkErrors::by_method`].
#[derive(Serialize, Deserialize)]
pub struct NetworkErrorsByMethod {
    /// The HTTP method (e.g. `"GET"`), or `None` if unknown.
    pub method: Option<String>,
    /// Total errors for this method.
    pub total: u64,
    /// Error count per status code, keyed by the status code as a string.
    pub by_status: HashMap<String, u64>,
}

/// Edge-network error analytics for an application.
///
/// Returned by
/// [`AppResource::network_errors`](crate::resources::AppResource::network_errors).
///
/// Requires a Pro or Enterprise plan.
#[derive(Serialize, Deserialize)]
pub struct NetworkErrors {
    /// Aggregate totals for the analysis window.
    pub summary: NetworkErrorSummary,
    /// Error count per individual HTTP status code.
    pub by_status: Vec<NetworkErrorByStatus>,
    /// Time-bucketed error counts across the analysis window.
    pub timeseries: Vec<NetworkErrorTimeseries>,
    /// Most error-prone request paths.
    pub top_paths: Vec<NetworkErrorPath>,
    /// Error totals aggregated by HTTP method.
    pub by_method: Vec<NetworkErrorsByMethod>,
}

/// Edge and origin latency percentiles in milliseconds.
///
/// Used in [`NetworkPerformanceSummary`] and
/// [`NetworkPerformanceTimeseries`].
#[derive(Serialize, Deserialize)]
pub struct NetworkLatencyPercentiles {
    /// 50th percentile latency in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// 99th percentile latency in milliseconds.
    pub p99: u32,
}

/// Aggregate latency summary for the analysis window.
///
/// Part of [`NetworkPerformance`].
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformanceSummary {
    /// Edge (CDN) latency percentiles.
    pub edge: NetworkLatencyPercentiles,
    /// Origin (application server) latency percentiles.
    pub origin: NetworkLatencyPercentiles,
    /// Total number of requests in the analysis window.
    pub requests: u64,
}

/// Latency percentiles for a single time bucket.
///
/// Part of [`NetworkPerformance::timeseries`].
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformanceTimeseries {
    /// The UTC timestamp that opens this time bucket.
    pub date: DateTime<Utc>,
    /// Number of requests in this bucket.
    pub requests: u64,
    /// Edge latency percentiles for this bucket.
    pub edge: NetworkLatencyPercentiles,
    /// Origin latency percentiles for this bucket.
    pub origin: NetworkLatencyPercentiles,
}

/// Per-country latency breakdown.
///
/// Part of [`NetworkPerformance::countries`].
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformanceCountry {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "type")]
    pub country_code: String,
    /// Median latency in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// Number of requests from this country.
    pub requests: u64,
}

/// Per-data-centre (colo) latency breakdown.
///
/// Part of [`NetworkPerformance::colos`].
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformanceColo {
    /// Data-centre identifier.
    #[serde(rename = "type")]
    pub colo_id: String,
    /// City where the data centre is located.
    pub city: String,
    /// Country where the data centre is located.
    pub country: String,
    /// Median latency in milliseconds.
    pub p50: u32,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// Number of requests served by this colo.
    pub requests: u64,
}

/// Latency statistics for a single slow request path.
///
/// Part of [`NetworkPerformance::slowest_paths`].
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformancePath {
    /// The request URI path.
    pub path: String,
    /// 95th percentile latency in milliseconds.
    pub p95: u32,
    /// 99th percentile latency in milliseconds.
    pub p99: u32,
    /// Number of requests to this path.
    pub requests: u64,
}

/// Edge-network latency analytics for an application.
///
/// Returned by
/// [`AppResource::network_performance`](crate::resources::AppResource::network_performance).
///
/// Requires a Pro or Enterprise plan.
#[derive(Serialize, Deserialize)]
pub struct NetworkPerformance {
    /// Aggregate latency summary for the analysis window.
    pub summary: NetworkPerformanceSummary,
    /// Time-bucketed latency data across the analysis window.
    pub timeseries: Vec<NetworkPerformanceTimeseries>,
    /// Per-country latency breakdown.
    pub countries: Vec<NetworkPerformanceCountry>,
    /// Per-data-centre latency breakdown.
    pub colos: Vec<NetworkPerformanceColo>,
    /// Latency statistics for the slowest request paths.
    pub slowest_paths: Vec<NetworkPerformancePath>,
}

/// Client information for an edge-network log entry.
///
/// Part of [`NetworkLogEntry`].
#[derive(Serialize, Deserialize)]
pub struct NetworkLogClient {
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
}

/// Request details for an edge-network log entry.
///
/// Part of [`NetworkLogEntry`].
#[derive(Serialize, Deserialize)]
pub struct NetworkLogRequest {
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
}

/// Response details for an edge-network log entry.
///
/// Part of [`NetworkLogEntry`].
#[derive(Serialize, Deserialize)]
pub struct NetworkLogResponse {
    /// The HTTP response status code.
    pub status: u16,
    /// The `Content-Type` of the response, if present.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// The edge cache result (e.g. `"HIT"` or `"MISS"`), if applicable.
    pub cache: Option<String>,
}

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
    /// Client identification and geolocation data.
    pub client: NetworkLogClient,
    /// Request metadata.
    pub request: NetworkLogRequest,
    /// Response metadata.
    pub response: NetworkLogResponse,
}
