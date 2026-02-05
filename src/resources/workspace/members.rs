use serde_json::Value;

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
}
