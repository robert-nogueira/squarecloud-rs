use std::sync::Arc;

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::http::ApiClient;
use crate::resources::AppResource;

/// Static metadata for a SquareCloud application.
///
/// Returned by [`ApiClient::upload_app`](crate::ApiClient::upload_app) and
/// [`AppResource::info`](crate::resources::AppResource::info). To obtain an
/// [`AppResource`] handle from this value, call
/// [`into_resource`](AppInfo::into_resource).
#[derive(Serialize, Deserialize)]
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

/// Network throughput figures for a running application.
#[derive(Serialize, Deserialize)]
pub struct AppNetwork {
    /// Cumulative bytes transferred since the application started.
    pub total: String,
    /// Bytes transferred in the current measurement interval.
    pub now: String,
}

/// A domain entry associated with an application.
///
/// Returned as part of a [`Vec`] by
/// [`ApiClient::all_domains`](crate::ApiClient::all_domains).
#[derive(Serialize, Deserialize)]
pub struct AppDomain {
    /// The owning application's unique identifier.
    pub app_id: String,
    /// The fully-qualified domain name.
    pub hostname: String,
    /// Either `"subdomain"` (*.squareweb.app) or `"custom"` (attached
    /// domain).
    #[serde(rename = "type")]
    pub domain_type: String,
}

/// A single historical resource-usage sample for an application.
///
/// Returned as part of a [`Vec`] by
/// [`AppResource::metrics`](crate::resources::AppResource::metrics).
/// Up to 288 data points covering the last 24 hours are returned,
/// sampled every 5 minutes.
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct AppStatus {
    /// Current CPU usage as a percentage string (e.g. `"3.2%"`).
    pub cpu: String,
    /// Current RAM usage (e.g. `"128MB"`).
    pub ram: String,
    /// Current disk usage.
    pub storage: String,
    /// Network throughput statistics.
    pub network: AppNetwork,
    /// The UTC timestamp when the process last started.
    #[serde(with = "ts_milliseconds")]
    pub uptime: DateTime<Utc>,
}
