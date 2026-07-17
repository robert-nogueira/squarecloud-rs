use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by environment variable operations
/// ([`AppResource`](crate::resources::AppResource) env methods).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum EnvErrorCode {
    /// `envs` is missing, not an object, or an array.
    InvalidEnvContent,
    /// A variable key exceeds 1024 characters.
    EnvNameTooLong,
    /// A variable value exceeds 4096 characters.
    EnvContentTooLong,
    /// The merged set would exceed 256 environment variables.
    TooManyEnvVars,
    /// The application is static; static apps don't support environment
    /// variables.
    StaticAppEnvNotSupported,
    /// The requested content could not be read from the cluster.
    ReadFailed,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// The `Authorization` header is missing, malformed, or the
    /// credentials are not valid.
    AccessDenied,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
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

impl_service_error_code!(EnvErrorCode);
