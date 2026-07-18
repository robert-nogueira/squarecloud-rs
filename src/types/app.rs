use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::http::Client;
use crate::resources::AppResource;

/// Detected runtime language returned by the upload endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppLanguage {
    /// Runtime identifier (e.g. `"javascript"`, `"python"`, `"rust"`).
    pub name: String,
    /// Detected version string (e.g. `"22"`, `"3.12"`).
    pub version: String,
}

/// Response returned by [`Client::upload_app`](crate::Client::upload_app).
///
/// This differs from [`AppInfo`] in that `language` is a structured object and
/// `cpu` is included. To obtain an [`AppResource`] handle from this value, call
/// [`into_resource`](UploadedApp::into_resource).
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedApp {
    /// The application's unique identifier.
    pub id: String,
    /// The application's display name.
    pub name: String,
    /// Optional description from `squarecloud.app`.
    pub description: Option<String>,
    /// Optional SquareCloud-assigned subdomain.
    pub subdomain: Option<String>,
    /// RAM allocation in megabytes.
    pub ram: u32,
    /// CPU shares allocated.
    pub cpu: f32,
    /// Detected runtime language and version.
    pub language: AppLanguage,
}

impl UploadedApp {
    /// Converts this value into an [`AppResource`] handle bound to `api`.
    pub fn into_resource(&self, api: Client) -> AppResource {
        AppResource::new(api, &self.id)
    }
}

/// Static metadata for a SquareCloud application.
///
/// Returned by [`AppResource::info`](crate::resources::AppResource::info).
/// To obtain an [`AppResource`] handle from this value, call
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
    pub fn into_resource(&self, api: Client) -> AppResource {
        AppResource::new(api, &self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppInfo, AppLanguage, UploadedApp};
    use crate::http::Client;

    fn client() -> Client {
        Client::new("test")
    }

    #[test]
    fn uploaded_app_into_resource_binds_correct_id() {
        let app = UploadedApp {
            id: "app-abc".to_string(),
            name: "test".to_string(),
            description: None,
            subdomain: None,
            ram: 512,
            cpu: 0.5,
            language: AppLanguage {
                name: "rust".to_string(),
                version: "1.80".to_string(),
            },
        };
        let resource = app.into_resource(client());
        assert_eq!(resource.id, "app-abc");
    }

    #[test]
    fn app_info_into_resource_binds_correct_id() {
        let info = AppInfo {
            id: "app-xyz".to_string(),
            name: "test".to_string(),
            owner: "owner".to_string(),
            cluster: "us-east".to_string(),
            ram: 256,
            language: "rust".to_string(),
            domain: None,
            custom: None,
        };
        let resource = info.into_resource(client());
        assert_eq!(resource.id, "app-xyz");
    }
}

/// Wrapper for the logs response body.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppLogs {
    pub(crate) logs: String,
}

/// A domain entry associated with an application.
///
/// Returned as part of a [`Vec`] by
/// [`Client::all_domains`](crate::Client::all_domains).
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
    pub net: [u64; 2],
}
