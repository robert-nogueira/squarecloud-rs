use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by workspace operations
/// ([`WorkspaceResource`](crate::resources::WorkspaceResource), creation,
/// listing and shared applications).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum WorkspaceErrorCode {
    /// A field value did not pass server-side regex validation.
    RegexValidation,
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

impl_service_error_code!(WorkspaceErrorCode);
