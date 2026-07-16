use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by workspace member operations (invite codes,
/// invitations, removal, permissions).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum MemberErrorCode {
    /// `memberId` is missing.
    MemberNotFound,
    /// The invited user is already a member of this workspace.
    MemberAlreadyAdded,
    /// The workspace already has the plan's maximum number of members.
    MembersLimitReached,
    /// The caller tried to change their own role.
    CannotEditOwner,
    /// The invite code belongs to the caller, who already owns the workspace.
    CannotInviteOwner,
    /// The invite code is missing, malformed, or has expired.
    InvalidCode,
    /// `group` is not one of `view`, `manager`, `maintain`, `admin`.
    InvalidGroup,
    /// `workspaceId` is missing.
    InvalidId,
    /// The workspace does not exist or the caller is not the owner or a
    /// member.
    WorkspaceNotFound,
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// Global rate limit of the authentication layer.
    RateLimit,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(MemberErrorCode);
