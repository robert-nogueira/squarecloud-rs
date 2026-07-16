use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by snapshot operations (application and database
/// snapshots, plus the account-wide snapshot listing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SnapshotErrorCode {
    /// The snapshot version ID is not valid or does not exist.
    InvalidVersionId,
    /// A snapshot restore is already in progress for this resource.
    RestoreInProgress,
    /// The daily snapshot creation limit for this resource has been reached.
    DailySnapshotsLimitReached,
    /// The `scope` query parameter value is not recognised by the API.
    InvalidScope,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The requested resource was not found.
    NotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(SnapshotErrorCode);
