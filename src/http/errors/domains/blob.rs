use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by blob storage operations
/// ([`BlobResource`](crate::resources::BlobResource)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BlobErrorCode {
    /// The provided object key is invalid: it must start with your own account
    /// ID prefix and contain no path traversal.
    InvalidObject,
    /// `name` does not match the allowed pattern (1 to 128 characters:
    /// letters, digits, `_`, `.` and `-`).
    InvalidObjectName,
    /// `prefix` does not match the allowed pattern.
    InvalidObjectPrefix,
    /// `expire` is not a valid duration (1 hour to 1825 days).
    InvalidObjectExpire,
    /// The `security_hash` parameter is not a boolean.
    InvalidObjectSecurityHash,
    /// The `auto_download` parameter is not a boolean.
    InvalidStorageAutoDownload,
    /// The request body is missing or is not a valid JSON object.
    InvalidBody,
    /// The provided continuation token is invalid (must be a string up to 2048
    /// characters).
    InvalidContinuationToken,
    /// The provided file is missing or malformed.
    InvalidFile,
    /// The file extension is malformed or too long.
    InvalidFileType,
    /// Executables and installers are not accepted.
    BlockedFileType,
    /// A prefix rule does not allow this file extension.
    FileTypeNotAllowed,
    /// The request body has an unsupported content type.
    InvalidContentType,
    /// The file is smaller than 512 bytes.
    FileTooSmall,
    /// The file is over 100 MB, or over the size a prefix rule or the plan
    /// allows.
    FileTooLarge,
    /// The specified object does not exist.
    ObjectNotFound,
    /// The list operation failed; retrying may succeed.
    ListFailed,
    /// The delete operation was rejected or failed.
    DeleteFailed,
    /// An unexpected error occurred while processing the upload.
    UploadFailed,
    /// The account exceeded its storage quota; delete objects or
    /// upgrade the plan.
    StorageQuotaExceeded,
    /// Too many simultaneous uploads (max 4 in progress at a time per
    /// account).
    TooManyConcurrentUploads,
    /// Private storage is temporarily unavailable; retry.
    PrivateStorageUnavailable,
    /// Public storage is temporarily unavailable; retry.
    PublicStorageUnavailable,
    /// The account has no active plan, which this action needs.
    PermissionDenied,
    /// The API key is missing or invalid.
    AccessDenied,
    /// The API key lacks the scope this route requires (`blob:read` or
    /// `blob:write`).
    MissingScope,
    /// The API key is restricted to specific applications and cannot
    /// reach Blob Storage.
    ResourceNotAllowed,
    /// The account is blocked from storing files.
    AccountBlocked,
    /// The requested option is not part of the plan.
    UpgradeRequired,
    /// The account request limit was reached, or the IP is temporarily
    /// blocked after repeated invalid credentials.
    RateLimited,
    /// The requested route does not exist.
    NotFound,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(BlobErrorCode);
