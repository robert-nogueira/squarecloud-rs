use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by application lifecycle and info operations
/// (start, restart, stop, status, info, metrics, logs, delete, realtime).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AppErrorCode {
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The cluster rejected the action, for example the application is already
    /// mid-deploy.
    ActionFailed,
    /// Start was rejected because the container is already running. Possibly
    /// superseded by `ACTION_FAILED`; kept until verified against the live
    /// API.
    ContainerAlreadyStarted,
    /// A snapshot restore is currently running for this application;
    /// wait for it to finish.
    RestoreInProgress,
    /// The delete operation was rejected or failed.
    DeleteFailed,
    /// The application has less than 512 MB of RAM allocated and does not
    /// collect metrics.
    MetricsNotSupported,
    /// The caller already has 5 concurrent realtime connections open.
    RealtimeMaxConnections,
    /// The application already has 30 concurrent realtime connections open
    /// across all users.
    RealtimeMaxConnectionsApp,
    /// The workspace does not exist or the caller is not the owner or a
    /// member.
    WorkspaceNotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(AppErrorCode);
