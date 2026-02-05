use serde_json::{Value, json};

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
};

use super::WorkspaceResource;

impl WorkspaceResource {
    pub async fn get_invite_code(&self) -> Result<String, ApiError> {
        let response: ApiResponse<Value> = self
            .client
            .request_endpoint(Endpoint::get_workspace_invite())
            .await?;
        let value = response.into_result_t()?;
        let code = value.get("code").and_then(Value::as_str).unwrap();
        Ok(code.to_string())
    }

    pub async fn invite_member(
        &self,
        code: &str,
        group: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::workspace_invite_member();
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({
		"workspaceId": self.id,
		"code": code,
		"group": group}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    pub async fn remove_member(
        &self,
        member_id: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::remove_workspace_member();
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({
		"workspaceId": self.id,
		"memberId": member_id}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    pub async fn change_member_permissions(
        &self,
        code: &str,
        group: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::workspace_change_member_permissions();
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({
		"workspaceId": self.id,
		"code": code,
		"group": group}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
