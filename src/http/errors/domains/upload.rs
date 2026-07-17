use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned when uploading or committing application archives
/// ([`Client::upload_app`](crate::Client::upload_app),
/// [`AppResource::commit`](crate::resources::AppResource::commit)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UploadErrorCode {
    /// The provided file is missing or malformed.
    InvalidFile,
    /// The file name contains path separators, `..`, or control characters.
    InvalidFilename,
    /// The `path` value contains traversal or shell metacharacters.
    InvalidPath,
    /// The request body has an unsupported content type.
    InvalidContentType,
    /// The file exceeds the 100 MB per-file limit.
    FileTooLarge,
    /// The cluster did not return a usable response.
    EmptyResponse,
    /// An unexpected error occurred while processing the upload.
    UploadFailed,
    /// The client disconnected before the upload finished.
    UploadAborted,
    /// The file failed to upload to storage; retry later.
    StorageUploadFailed,
    /// The commit could not be applied.
    CommitFailed,
    /// The account's available memory quota is insufficient for this
    /// resource.
    InsufficientMemory,
    /// No cluster was available to host the application.
    ClusterSelectionFailed,
    /// Application provisioning is temporarily unavailable due to maintenance.
    ClusterMaintenanceTryLater,
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
    /// The caller lacks permission for this operation (workspace role
    /// or restricted resource).
    PermissionDenied,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(UploadErrorCode);
