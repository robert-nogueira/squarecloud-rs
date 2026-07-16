use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by file manager operations
/// ([`FileResource`](crate::resources::FileResource)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum FileErrorCode {
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The requested resource was not found.
    NotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(FileErrorCode);
