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
    /// The provided object name is invalid. Must adhere to the a to z, A to Z,
    /// 0 to 9, and _ pattern.
    InvalidObjectName,
    /// The provided object prefix is invalid. Must adhere to the a to z, A to
    /// Z, 0 to 9, and _ pattern.
    InvalidObjectPrefix,
    /// The expiration value is invalid; it must be a number of days
    /// from 1 to 365.
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
    /// The provided file type is invalid.
    InvalidFileType,
    /// The request body has an unsupported content type.
    InvalidContentType,
    /// The file size is too small (< 1kb).
    FileTooSmall,
    /// The file exceeds the 100 MB per-file limit.
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
    /// The caller lacks permission for this resource (restricted file or plan
    /// gate).
    PermissionDenied,
    /// The API key is missing or invalid.
    AccessDenied,
    /// Per-route rate limit of the Blob API was exceeded.
    RateLimited,
    /// Global rate limit of the authentication layer.
    RateLimit,
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
