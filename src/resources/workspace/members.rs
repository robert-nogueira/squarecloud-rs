use serde::Deserialize;
use serde_json::json;

use crate::{
    Endpoint,
    http::{
        ApiResponse, NoData,
        errors::{ApiError, MemberErrorCode},
    },
};

use super::WorkspaceResource;

impl WorkspaceResource {
    /// Retrieves the current invitation code for this workspace.
    ///
    /// The code can be shared with potential members, who join by passing it
    /// to [`invite_member`](WorkspaceResource::invite_member).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn get_invite_code(
        &self,
    ) -> Result<String, ApiError<MemberErrorCode>> {
        #[derive(Deserialize)]
        struct InviteCode {
            code: String,
        }
        let response: ApiResponse<InviteCode> = self
            .client
            .request_endpoint(Endpoint::get_workspace_invite())
            .await?;
        Ok(response.into_result_t()?.code)
    }

    /// Accepts an invitation and joins this workspace under the given
    /// permission group.
    ///
    /// `code` is the invite code obtained from the workspace owner via
    /// [`get_invite_code`](WorkspaceResource::get_invite_code). `group` is the
    /// permission level assigned to the new member (e.g. `"member"`,
    /// `"admin"`). Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] if the code is invalid or has
    /// expired.
    pub async fn invite_member(
        &self,
        code: &str,
        group: &str,
    ) -> Result<bool, ApiError<MemberErrorCode>> {
        let endpoint = Endpoint::workspace_invite_member();
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({
		"workspaceId": self.id,
		"code": code,
		"group": group}))
            .build()?;
        self.client
            .execute_request::<NoData>(request)
            .await?
            .into_bool_result()
    }

    /// Removes the member identified by `member_id` from this workspace.
    ///
    /// Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] if the member does not exist in
    /// this workspace.
    pub async fn remove_member(
        &self,
        member_id: &str,
    ) -> Result<bool, ApiError<MemberErrorCode>> {
        let endpoint = Endpoint::remove_workspace_member();
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({
		"workspaceId": self.id,
		"memberId": member_id}))
            .build()?;
        self.client
            .execute_request::<NoData>(request)
            .await?
            .into_bool_result()
    }

    /// Changes the permission group of an existing workspace member.
    ///
    /// `code` is the invite code that identifies the member and `group` is the
    /// new permission level (e.g. `"member"`, `"admin"`). Returns `Ok(true)`
    /// on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] if the member or group is invalid.
    pub async fn change_member_permissions(
        &self,
        code: &str,
        group: &str,
    ) -> Result<bool, ApiError<MemberErrorCode>> {
        let endpoint = Endpoint::workspace_change_member_permissions();
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({
		"workspaceId": self.id,
		"code": code,
		"group": group}))
            .build()?;
        self.client
            .execute_request::<NoData>(request)
            .await?
            .into_bool_result()
    }
}
