use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by workspace operations
/// ([`WorkspaceResource`](crate::resources::WorkspaceResource), creation,
/// listing and shared applications).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum WorkspaceErrorCode {
    /// The workspace does not exist or the caller is not the owner or a
    /// member.
    WorkspaceNotFound,
    /// The workspace could not be created.
    WorkspaceCreationFailed,
    /// The account already has the plan's maximum number of workspaces.
    WorkspaceLimitReached,
    /// The name fails the 1-32 character validation.
    InvalidName,
    /// `workspaceId` is missing.
    InvalidId,
    /// The caller is the workspace owner and must delete it instead of
    /// leaving.
    CannotLeaveOwner,
    /// The application is already shared into this workspace.
    AppAlreadyInWorkspace,
    /// The workspace already has 100 shared applications.
    ApplicationsLimitReached,
    /// The application does not exist or is not owned by the caller.
    AppNotFound,
    /// The endpoint requires a higher plan than the account currently has.
    UpgradeRequired,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(WorkspaceErrorCode);
