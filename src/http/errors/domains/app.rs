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
    /// The action was rejected because the container is already running.
    ContainerAlreadyStarted,
    /// The action was rejected because the container is already stopped.
    ContainerAlreadyStopped,
    /// The container is temporarily suspended by the platform.
    ContainerTemporarilySuspended,
    /// The node hosting the container has no disk space left.
    ContainerInsufficientDiskSpace,
    /// The container could not start because of a port conflict.
    ContainerNetworkConflict,
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
    /// The realtime stream's container disappeared mid-connection (e.g. the
    /// application was stopped or deleted while streaming).
    ContainerNotFound,
    /// The workspace does not exist or the caller is not the owner or a
    /// member.
    WorkspaceNotFound,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// The account or API key request limit was reached, or the IP is
    /// temporarily blocked after repeated invalid keys.
    RateLimited,
    /// The API key lacks the scope this route requires.
    MissingScope,
    /// The API key is restricted to resources that do not cover this
    /// request.
    ResourceNotAllowed,
    /// SquareCloud's own backing service is briefly unavailable (not a
    /// user database); retry in a few seconds.
    DatabaseUnavailable,
    /// The `Authorization` header is missing, malformed, or the
    /// credentials are not valid.
    AccessDenied,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// Short-lived rate limit; retry after a few seconds.
    KeepCalm,
    /// The caller lacks permission for this operation (workspace role
    /// or restricted resource).
    PermissionDenied,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(AppErrorCode);
