use serde_json::json;

use crate::{Endpoint, http::errors::ApiError};

use super::WorkspaceResource;

impl WorkspaceResource {
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
}
