use serde_json::json;

use crate::{Endpoint, http::errors::ApiError};

use super::WorkspaceResource;

impl WorkspaceResource {
    /// Adds the application identified by `app_id` to this workspace.
    ///
    /// Returns `Ok(true)` when the association is created.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// if the application is already in a workspace or does not exist.
    pub async fn add_app(&self, app_id: &str) -> Result<bool, ApiError> {
        let endpoint = Endpoint::workspace_add_app();
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({
		"workspaceId": self.id,
		"appId": app_id}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    /// Removes the application identified by `app_id` from this workspace.
    ///
    /// Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// if the application is not a member of this workspace.
    pub async fn remove_app(&self, app_id: &str) -> Result<bool, ApiError> {
        let endpoint = Endpoint::workspace_remove_app();
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({
		"workspaceId": self.id,
		"appId": app_id}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
