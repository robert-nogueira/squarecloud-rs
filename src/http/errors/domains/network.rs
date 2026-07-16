use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by network operations (analytics, DNS, custom
/// domain, logs, performance, cache purge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum NetworkErrorCode {
    /// The domain is not a valid fully qualified domain name.
    InvalidDomain,
    /// Square Cloud domains cannot be used as a custom domain.
    ReservedDomain,
    /// The domain is already attached to an application owned by another
    /// account.
    DomainAlreadyExists,
    /// The domain attachment failed at the edge provider.
    DnsFailed,
    /// The plan's limit of applications on this domain was reached.
    LoadBalancerLimitReached,
    /// The application does not have a custom domain attached.
    NoCustomDomain,
    /// `start`/`end` are missing, malformed, or invert the time range.
    InvalidTimeRange,
    /// A drill-down filter does not match its expected format.
    InvalidFilter,
    /// Failed to fetch analytics from the edge provider.
    UnableToFetchAnalytics,
    /// Failed to fetch error data from the edge provider.
    UnableToFetchErrors,
    /// Failed to fetch performance data from the edge provider.
    UnableToFetchPerformance,
    /// 429 em `GET .../network/analytics`, `.../network/errors`,
    /// `.../network/logs`, `.../network/performance`, `GET
    /// /v2/users/snapshots`
    RateLimitExceeded,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(NetworkErrorCode);
