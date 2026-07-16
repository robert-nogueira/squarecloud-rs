use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by account operations
/// ([`Client::me`](crate::Client::me)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AccountErrorCode {
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(AccountErrorCode);
