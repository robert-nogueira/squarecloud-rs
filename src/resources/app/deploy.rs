use serde_json::{Value, json};

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
    types::Deploy,
};

use super::AppResource;

impl AppResource {
    pub async fn current_deploy(&self) -> Result<Deploy, ApiError> {
        self.client
            .request_endpoint(Endpoint::get_current_app_deploy(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn list_deploys(&self) -> Result<Vec<Deploy>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_deploys(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn set_webhook_integration(
        &self,
        access_token: String,
    ) -> Result<String, ApiError> {
        let endpoint = Endpoint::set_webhook_integration(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"access_token": access_token}))
            .build()?;
        let response: ApiResponse<Value> =
            self.client.execute_request(request).await?;
        let value = response.into_result_t()?;
        let webhook = value.get("webhook").and_then(Value::as_str).unwrap();
        Ok(webhook.to_string())
    }
}
