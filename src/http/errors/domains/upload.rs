use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned when uploading or committing application archives
/// ([`Client::upload_app`](crate::Client::upload_app),
/// [`AppResource::commit`](crate::resources::AppResource::commit)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum UploadErrorCode {
    /// The account's memory quota is insufficient for the requested
    /// allocation.
    FewMemory,
    /// The requested memory value is not a valid allocation size.
    BadMemory,
    /// The uploaded archive does not contain a `squarecloud.app`
    /// configuration file.
    MissingConfig,
    /// A dependency listed in the configuration is not a supported
    /// package.
    InvalidDependency,
    /// No entry-point file was detected in the uploaded archive.
    MissingMain,
    /// The entry-point file specified in the configuration does not
    /// exist in the archive.
    InvalidMain,
    /// The `display_name` field contains disallowed characters.
    InvalidDisplayName,
    /// The `display_name` field is absent from the configuration.
    MissingDisplayName,
    /// The memory value is outside the range permitted by the plan.
    InvalidMemory,
    /// The `memory` field is absent from the configuration.
    MissingMemory,
    /// The language or runtime version is not supported.
    InvalidVersion,
    /// The `version` field is absent from the configuration.
    MissingVersion,
    /// The `start` command in the configuration is not a valid invocation.
    InvalidStart,
    /// The uploaded file is malformed or has an unsupported format.
    InvalidFile,
    /// Upload rate limit: too many uploads in a short window. Try again later.
    KeepCalm,
    /// The target application does not exist or is not owned by the caller.
    AppNotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// The requested resource was not found.
    NotFound,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(UploadErrorCode);
