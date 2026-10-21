use serde::Deserialize;
use serde_json::json;

use crate::{
    Endpoint,
    http::{
        ApiResponse, NoData,
        errors::{ApiError, DeployErrorCode},
    },
    types::Deploy,
};

use super::AppResource;

impl AppResource {
    /// Returns metadata about the most recently completed deployment.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn current_deploy(
        &self,
    ) -> Result<Deploy, ApiError<DeployErrorCode>> {
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn list_deploys(
        &self,
    ) -> Result<Vec<Deploy>, ApiError<DeployErrorCode>> {
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
    /// Passing `"@"` removes the integration instead and returns an empty
    /// string; prefer
    /// [`remove_webhook_integration`](AppResource::remove_webhook_integration).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Service`] with
    /// [`DeployErrorCode::InvalidAccessToken`] if the token is rejected, or
    /// [`ApiError::Transport`] if the request fails or the reply does not
    /// decode.
    pub async fn set_webhook_integration(
        &self,
        access_token: String,
    ) -> Result<String, ApiError<DeployErrorCode>> {
        let removing = access_token == "@";
        let endpoint = Endpoint::set_webhook_integration(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"access_token": access_token}))
            .build()?;
        #[derive(Deserialize)]
        struct Webhook {
            webhook: String,
        }
        if removing {
            self.client
                .execute_request::<NoData>(request)
                .await?
                .into_bool_result()?;
            return Ok(String::new());
        }
        let response: ApiResponse<Webhook> =
            self.client.execute_request(request).await?;
        Ok(response.into_result_t()?.webhook)
    }

    /// Removes the application's webhook integration. Returns `Ok(true)` on
    /// success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn remove_webhook_integration(
        &self,
    ) -> Result<bool, ApiError<DeployErrorCode>> {
        self.set_webhook_integration("@".to_owned())
            .await
            .map(|_| true)
    }
}
