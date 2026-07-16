use crate::{
    Endpoint,
    http::{
        Client,
        errors::{ApiError, WorkspaceErrorCode},
    },
    types::WorkspaceInfo,
};

/// A handle to a specific SquareCloud workspace.
///
/// Obtain a `WorkspaceResource` by calling [`Client::workspace`] with the
/// workspace ID, or via
/// [`WorkspaceInfo::into_resource`](crate::types::WorkspaceInfo::into_resource).
///
/// Methods are spread across multiple `impl` blocks in submodules:
///
/// | Source file | Methods |
/// |-------------|---------|
/// | `resource.rs` | info, leave, delete |
/// | `applications.rs` | app membership |
/// | `members.rs` | invite and member management |
pub struct WorkspaceResource {
    /// The workspace's unique identifier.
    pub id: String,
    pub(crate) client: Client,
}

impl WorkspaceResource {
    /// Creates a new `WorkspaceResource` bound to the given client and
    /// workspace ID.
    ///
    /// Prefer [`Client::workspace`] over calling this directly.
    pub fn new(http: Client, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    /// Returns metadata about the workspace, including its member list and
    /// application list.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn info(
        &self,
    ) -> Result<WorkspaceInfo, ApiError<WorkspaceErrorCode>> {
        self.client
            .request_endpoint(Endpoint::get_workspace(&self.id))
            .await?
            .into_result_t()
    }

    /// Removes the authenticated account from the workspace.
    ///
    /// Returns `Ok(true)` on success. The owner of a workspace cannot leave
    /// it; use [`delete`](WorkspaceResource::delete) instead.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if the account is the owner or is not a member.
    pub async fn leave(&self) -> Result<bool, ApiError<WorkspaceErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::leave_workspace())
            .await?
            .into_bool_result()
    }

    /// Permanently deletes the workspace and all member and application
    /// associations.
    ///
    /// Only the workspace owner can delete the workspace. This action cannot
    /// be undone. Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if the authenticated account is not the owner.
    pub async fn delete(&self) -> Result<bool, ApiError<WorkspaceErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::delete_workspace())
            .await?
            .into_bool_result()
    }
}
