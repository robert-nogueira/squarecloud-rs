use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by network operations (analytics, DNS, custom
/// domain, logs, performance, cache purge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum NetworkErrorCode {
    /// The requested time range has no data or is invalid.
    InvalidTimeRange,
    /// The application has no custom domain configured.
    NoCustomDomain,
    /// The requested subdomain is malformed or already taken.
    InvalidSubdomain,
    /// Rate limit: too many expensive operations in a short window.
    KeepCalm,
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

impl_service_error_code!(NetworkErrorCode);
