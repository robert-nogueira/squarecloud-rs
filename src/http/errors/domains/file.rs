use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by file manager operations
/// ([`FileResource`](crate::resources::FileResource)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum FileErrorCode {
    /// The `path` value contains traversal or shell metacharacters.
    InvalidPath,
    /// The requested path is on the platform's blocked-paths list.
    BlockedPath,
    /// No file exists at the given path.
    FileNotFound,
    /// The file exceeds the 100 MB per-file limit.
    FileTooLarge,
    /// `content` is missing or empty.
    InvalidContent,
    /// The cluster could not rename the file.
    RenameFailed,
    /// The delete operation was rejected or failed.
    DeleteFailed,
    /// The parsed config update could not be persisted.
    SaveFailed,
    /// The `display_name` field parsed from a `squarecloud.app`/
    /// `squarecloud.config` write failed validation.
    InvalidDisplayName,
    /// The `description` field parsed from a `squarecloud.app`/
    /// `squarecloud.config` write failed validation.
    InvalidDescription,
    /// The memory value is not permitted for this resource or plan.
    InvalidMemory,
    /// The `autorestart` field parsed from a `squarecloud.app`/
    /// `squarecloud.config` write failed validation.
    InvalidAutorestart,
    /// The `subdomain` field parsed from a `squarecloud.app`/
    /// `squarecloud.config` write failed validation.
    InvalidSubdomain,
    /// The parsed config tried to set a subdomain the plan or account cannot
    /// use.
    CannotSetSubdomain,
    /// The caller lacks permission for this resource (restricted file or plan
    /// gate).
    PermissionDenied,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
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
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(FileErrorCode);
