use serde_json::{Value, json};

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
    types::Deploy,
};

use super::AppResource;

impl AppResource {
    /// Returns metadata about the most recently completed deployment.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn current_deploy(&self) -> Result<Deploy, ApiError> {
        self.client
            .request_endpoint(Endpoint::get_current_app_deploy(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns the full deployment history for the application, ordered from
    /// most recent to oldest.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn list_deploys(&self) -> Result<Vec<Deploy>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_deploys(&self.id))
            .await?
            .into_result_t()
    }

    /// Registers a webhook integration for deployment events.
    ///
    /// `access_token` is an opaque token issued by the target webhook
    /// provider (e.g. GitHub deployments). On success, returns the canonical
    /// webhook URL that SquareCloud will call on each deploy.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with
    /// [`ApiErrorCode::InvalidAccessToken`](crate::ApiErrorCode::InvalidAccessToken)
    /// if the token is rejected, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn set_webhook_integration(
        &self,
        access_token: String,
    ) -> Result<String, ApiError> {
        let endpoint = Endpoint::set_webhook_integration(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"access_token": access_token}))
            .build()?;
        let response: ApiResponse<Value> =
            self.client.execute_request(request).await?;
        let value = response.into_result_t()?;
        let webhook = value.get("webhook").and_then(Value::as_str).unwrap();
        Ok(webhook.to_string())
    }
}
