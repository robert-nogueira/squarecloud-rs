use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by database operations
/// ([`DatabaseResource`](crate::resources::DatabaseResource), including
/// credentials, creation and status listing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DatabaseErrorCode {
    /// The database type value is not recognised by the API.
    DatabaseTypeInvalid,
    /// The database version string is not valid for the chosen engine.
    DatabaseVersionInvalid,
    /// The memory value is outside the range permitted by the plan.
    InvalidMemory,
    /// The account's memory quota is insufficient for the requested
    /// allocation.
    FewMemory,
    /// The requested memory value is not a valid allocation size.
    BadMemory,
    /// Start was rejected because the container is already running.
    ContainerAlreadyStarted,
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

impl_service_error_code!(DatabaseErrorCode);
