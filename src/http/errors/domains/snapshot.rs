use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by snapshot operations (application and database
/// snapshots, plus the account-wide snapshot listing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SnapshotErrorCode {
    /// The snapshot request was accepted (HTTP 202) and is being
    /// processed asynchronously.
    SnapshotProcessing,
    /// The cluster could not create the snapshot.
    SnapshotFailed,
    /// No snapshot exists at that ID and version for this resource.
    SnapshotNotFound,
    /// The cluster could not dispatch the restore.
    SnapshotRestoreFailed,
    /// The snapshot's engine does not match this database's `type`.
    SnapshotDatabaseMismatch,
    /// `snapshotId` is not a valid UUID v4.
    InvalidSnapshotId,
    /// `versionId` does not match the expected S3 version-key format.
    InvalidVersionId,
    /// `snapshotId` or `versionId` is missing.
    MissingParameters,
    /// The `scope` query parameter is not `applications` or `databases`.
    InvalidScope,
    /// The plan's daily snapshot creation quota was reached (distinct from
    /// `KEEP_CALM`).
    DailySnapshotsLimitReached,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
    /// 429 em `GET .../network/analytics`, `.../network/errors`,
    /// `.../network/logs`, `.../network/performance`, `GET
    /// /v2/users/snapshots`
    RateLimitExceeded,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// The resource owner could not be resolved.
    UserNotFound,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The database does not exist or is not owned by the caller.
    DatabaseNotFound,
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

impl_service_error_code!(SnapshotErrorCode);
