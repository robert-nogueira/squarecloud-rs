use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by deploy operations
/// ([`AppResource`](crate::resources::AppResource) deploy methods).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DeployErrorCode {
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(DeployErrorCode);
