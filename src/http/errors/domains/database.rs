use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by database operations
/// ([`DatabaseResource`](crate::resources::DatabaseResource), including
/// credentials, creation and status listing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DatabaseErrorCode {
    /// The database does not exist or is not owned by the caller.
    DatabaseNotFound,
    /// The database is stopped; start it before requesting credentials.
    DatabaseNotRunning,
    /// Provisioning failed, or the cluster response was incomplete.
    DatabaseCreationFailed,
    /// `type` is not one of `mongo`, `mysql`, `redis`, `postgres`.
    InvalidDatabaseType,
    /// `version` is not a supported version for the chosen `type`.
    InvalidDatabaseVersion,
    /// The name fails the 1-32 character validation.
    InvalidName,
    /// The memory value is not permitted for this resource or plan.
    InvalidMemory,
    /// The account's available memory quota is insufficient for this
    /// resource.
    InsufficientMemory,
    /// Neither `name` nor `ram` was provided in the request body.
    NoUpdateData,
    /// `reset` was not `"password"` or `"certificate"`.
    InvalidResetType,
    /// The credential rotation was rejected by the cluster.
    ResetFailed,
    /// The requested content could not be read from the cluster.
    ReadFailed,
    /// The delete operation was rejected or failed.
    DeleteFailed,
    /// A snapshot restore is currently running for this database;
    /// wait for it to finish.
    RestoreInProgress,
    /// No cluster was available to host the database.
    ClusterSelectionFailed,
    /// Provisioning is temporarily unavailable due to maintenance.
    ClusterMaintenanceTryLater,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// The `Authorization` header is missing, malformed, or the
    /// credentials are not valid.
    AccessDenied,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
    /// The request payload exceeds the accepted size.
    PayloadTooLarge,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(DatabaseErrorCode);
