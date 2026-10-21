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
    /// The account or API key request limit was reached, or the IP is
    /// temporarily blocked after repeated invalid keys.
    RateLimited,
    /// The API key lacks the scope this route requires.
    MissingScope,
    /// The API key is restricted to resources that do not cover this
    /// request.
    ResourceNotAllowed,
    /// SquareCloud's own backing service is briefly unavailable (not a
    /// user database); retry in a few seconds.
    DatabaseUnavailable,
    /// The `Authorization` header is missing, malformed, or the
    /// credentials are not valid.
    AccessDenied,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
    /// The request payload exceeds the accepted size.
    PayloadTooLarge,
    /// The caller lacks permission for this operation (workspace role
    /// or restricted resource).
    PermissionDenied,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(DeployErrorCode);
