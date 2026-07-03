use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::http::ApiClient;
use crate::resources::AppResource;

/// Deserializes a field that the API returns as either a string or a number.
///
/// Used for `cpu`, `ram`, and `storage` in [`AppStatus`], which the API
/// returns as formatted strings by default (e.g. `"3.2%"`) but as raw
/// numbers when called with `?rawData=true`.
fn deserialize_as_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};

    struct V;

    impl<'de> Visitor<'de> for V {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a string or number")
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<String, E> {
            Ok(v.to_owned())
        }

        fn visit_string<E: Error>(self, v: String) -> Result<String, E> {
            Ok(v)
        }

        fn visit_f64<E: Error>(self, v: f64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<String, E> {
            Ok(v.to_string())
        }
    }

    d.deserialize_any(V)
}

/// Static metadata for a SquareCloud application.
///
/// Returned by [`ApiClient::upload_app`](crate::ApiClient::upload_app) and
/// [`AppResource::info`](crate::resources::AppResource::info). To obtain an
/// [`AppResource`] handle from this value, call
/// [`into_resource`](AppInfo::into_resource).
#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    /// The application's display name.
    pub name: String,
    /// The application's unique identifier.
    pub id: String,
    /// The owner account's unique identifier.
    pub owner: String,
    /// The data-centre cluster the application is deployed to.
    pub cluster: String,
    /// RAM allocation in megabytes.
    pub ram: u32,
    /// The programming language or runtime the application uses.
    pub language: String,
    /// The SquareCloud-assigned subdomain, if any.
    pub domain: Option<String>,
    /// A custom domain configured by the owner, if any.
    pub custom: Option<String>,
}

impl AppInfo {
    /// Converts this value into an [`AppResource`] handle bound to `api`.
    pub fn into_resource(&self, api: Arc<ApiClient>) -> AppResource {
        AppResource::new(api, &self.id)
    }
}

/// A network throughput counter that the API returns as either a formatted
/// string or a raw `[bytes_in, bytes_out]` array when called with
/// `?rawData=true`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkCounter {
    /// Human-readable summary (e.g. `"1 MB ↑ 500 KB ↓"`).
    Formatted(String),
    /// Raw byte counts as `[bytes_in, bytes_out]`.
    Raw(Vec<u64>),
}

/// Network throughput figures for a running application.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppNetwork {
    /// Cumulative bytes transferred since the application started.
    pub total: NetworkCounter,
    /// Bytes transferred in the current measurement interval.
    pub now: NetworkCounter,
}

/// A domain entry associated with an application.
///
/// Returned as part of a [`Vec`] by
/// [`ApiClient::all_domains`](crate::ApiClient::all_domains).
#[derive(Debug, Serialize, Deserialize)]
pub struct AppDomain {
    /// The owning application's unique identifier.
    pub app_id: String,
    /// The fully-qualified domain name.
    pub hostname: String,
    /// Either `"subdomain"` (*.squareweb.app) or `"custom"` (attached domain).
    #[serde(rename = "type")]
    pub domain_type: String,
}

/// A single historical resource-usage sample for an application.
///
/// Returned as part of a [`Vec`] by
/// [`AppResource::metrics`](crate::resources::AppResource::metrics).
/// Up to 288 data points covering the last 24 hours are returned,
/// sampled every 5 minutes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppMetrics {
    /// The UTC timestamp this sample covers.
    pub date: DateTime<Utc>,
    /// CPU usage as a percentage at this point in time.
    pub cpu: f32,
    /// RAM consumption in megabytes at this point in time.
    pub ram: f32,
    /// Network byte counts as `[bytes_in, bytes_out]`.
    pub net: [u32; 2],
}

/// Runtime status for a running application.
///
/// Returned by [`AppResource::status`](crate::resources::AppResource::status).
/// The `cpu`, `ram`, and `storage` fields accept both the formatted string
/// mode (default) and the raw numeric mode (`?rawData=true`).
#[derive(Debug, Serialize, Deserialize)]
pub struct AppStatus {
    /// CPU usage (e.g. `"3.2%"` or `3.2` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub cpu: String,
    /// RAM usage (e.g. `"128/512MB"` or `128` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub ram: String,
    /// Resource lifecycle state (e.g. `"running"`, `"exited"`).
    pub status: String,
    /// Whether the application process is currently running.
    pub running: bool,
    /// Disk usage (e.g. `"50MB"` or `52428800` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub storage: String,
    /// Network throughput statistics.
    pub network: AppNetwork,
    /// The UTC timestamp when the process last started. `null` when not running.
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub uptime: Option<DateTime<Utc>>,
}
