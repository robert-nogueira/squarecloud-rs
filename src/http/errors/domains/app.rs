use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by application lifecycle and info operations
/// (start, restart, stop, status, info, metrics, logs, delete, realtime).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AppErrorCode {
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// Start was rejected because the container is already running.
    ContainerAlreadyStarted,
    /// A snapshot restore is already in progress for this application.
    RestoreInProgress,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// The requested resource was not found.
    NotFound,
    /// Rate limit: too many expensive operations in a short window.
    KeepCalm,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(AppErrorCode);
