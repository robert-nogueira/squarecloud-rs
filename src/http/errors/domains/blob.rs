use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by blob storage operations
/// ([`BlobResource`](crate::resources::BlobResource)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BlobErrorCode {
    /// The blob object name or ID is malformed.
    InvalidObject,
    /// The blob deletion request failed; retrying may succeed.
    FailedDelete,
    /// The specified blob object does not exist.
    ObjectNotFound,
    /// The `prefix` query parameter for blob listing is malformed.
    InvalidObjectPrefix,
    /// The uploaded file's MIME type or content is not accepted by the blob API.
    InvalidFiletype,
    /// The uploaded file is below the minimum size accepted by the blob API.
    FileTooSmall,
    /// Upload rate limit: too many uploads in a short window. Try again later.
    KeepCalm,
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

impl_service_error_code!(BlobErrorCode);
