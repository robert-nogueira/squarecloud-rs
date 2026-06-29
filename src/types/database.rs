use std::sync::Arc;

use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

use crate::{http::ApiClient, resources::DatabaseResource};

/// Network throughput figures for a running database instance.
#[derive(Serialize, Deserialize)]
pub struct DatabaseNetwork {
    /// Cumulative bytes transferred since the database started.
    pub total: String,
    /// Bytes transferred in the current measurement interval.
    pub now: String,
}

/// Condensed runtime status for a database, as returned in list responses.
///
/// Returned as part of a [`Vec`] by
/// [`ApiClient::all_database_status`](crate::ApiClient::all_database_status).
#[derive(Serialize, Deserialize)]
pub struct DatabaseResumedStatus {
    /// The database's unique identifier.
    pub id: String,
    /// `true` if the database process is currently running.
    pub running: bool,
    /// Current CPU usage as a percentage string.
    pub cpu: String,
    /// Current RAM usage.
    pub ram: String,
}

/// Detailed runtime status for a single database instance.
///
/// Returned by
/// [`DatabaseResource::status`](crate::resources::DatabaseResource::status).
#[derive(Serialize, Deserialize)]
pub struct DatabaseStatus {
    /// Current CPU usage as a percentage string.
    pub cpu: String,
    /// Current RAM usage.
    pub ram: String,
    /// A human-readable status label (e.g. `"running"`, `"stopped"`).
    pub status: String,
    /// Network throughput statistics.
    pub network: DatabaseNetwork,
    /// The UTC timestamp when the database process last started.
    #[serde(with = "ts_milliseconds")]
    pub uptime: DateTime<Utc>,
}

/// A single historical resource-usage sample for a database.
///
/// Returned as part of a [`Vec`] by
/// [`DatabaseResource::metrics`](crate::resources::DatabaseResource::metrics).
#[derive(Serialize, Deserialize)]
pub struct DatabaseMetrics {
    /// The UTC timestamp this sample covers.
    pub date: DateTime<Utc>,
    /// CPU usage as a percentage at this point in time.
    pub cpu: f32,
    /// RAM usage as a percentage at this point in time.
    pub ram: f32,
    /// Network byte counts as `[bytes_in, bytes_out]`.
    pub net: [u32; 2],
}

/// The type of managed database engine to provision.
#[derive(Serialize, Deserialize)]
pub enum DatabaseType {
    /// Redis in-memory data store.
    Redis,
    /// PostgreSQL relational database.
    Postgres,
    /// MongoDB document database.
    Mongo,
}

/// Static metadata for a SquareCloud managed database.
///
/// Returned by
/// [`DatabaseResource::info`](crate::resources::DatabaseResource::info).
/// To obtain a [`DatabaseResource`] handle from an existing value, use
/// [`ApiClient::database`](crate::ApiClient::database) with the `id` field.
#[derive(Serialize, Deserialize)]
pub struct DatabaseInfo {
    /// The database's unique identifier.
    pub id: String,
    /// The user-defined name of the database.
    pub name: String,
    /// The owner account's unique identifier.
    pub owner: String,
    /// The data-centre cluster the database is hosted on.
    pub cluster: String,
    /// RAM allocation in megabytes.
    pub ram: u32,
    /// The database engine type (e.g. `"mongodb"`, `"postgresql"`).
    #[serde(rename = "type")]
    pub db_type: String,
    /// The network port the database listens on.
    pub port: u32,
    /// The UTC timestamp when the database was created.
    pub created_at: DateTime<Utc>,
}

/// Full details for a provisioned database.
///
/// Returned by
/// [`ApiClient::create_database`](crate::ApiClient::create_database). To
/// obtain a [`DatabaseResource`] handle from this value, call
/// [`into_resource`](Database::into_resource).
#[derive(Serialize, Deserialize)]
pub struct Database {
    /// The database's unique identifier.
    pub id: String,
    /// The human-readable name given at creation time.
    pub name: String,
    /// RAM allocation in megabytes.
    pub memory: u32,
    /// CPU allocation in abstract units.
    pub cpu: u8,
    /// The database engine type (e.g. `"redis"`, `"postgres"`).
    #[serde(rename = "type")]
    pub db_type: String,
    /// The current database password.
    pub password: String,
    /// The PEM-encoded TLS client certificate.
    pub certificate: String,
    /// The full connection URL including credentials.
    pub connection_url: String,
}

/// Selects which credential to regenerate when calling
/// [`DatabaseResource::redefine_credential`](crate::resources::DatabaseResource::redefine_credential).
pub enum CredentialType {
    /// Regenerate the TLS client certificate.
    Certificate,
    /// Generate a new password.
    Password,
}

impl CredentialType {
    pub fn as_str(&self) -> &str {
        match self {
            CredentialType::Certificate => "certificate",
            CredentialType::Password => "password",
        }
    }
}

/// A resolved credential value.
pub struct Credential {
    /// The type of credential that was rotated.
    pub credential_type: CredentialType,
    /// The new credential value (password string or PEM certificate).
    pub value: String,
}

impl Database {
    /// Converts this value into a [`DatabaseResource`] handle bound to `api`.
    pub fn into_resource(&self, api: Arc<ApiClient>) -> DatabaseResource {
        DatabaseResource::new(api, &self.id)
    }
}
