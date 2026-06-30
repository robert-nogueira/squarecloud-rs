use std::collections::HashMap;

use serde_json::json;

use crate::{Endpoint, http::errors::ApiError};

use super::AppResource;

impl AppResource {
    /// Returns all environment variables currently set on the application.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn list_envs(
        &self,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_envs(&self.id))
            .await?
            .into_result_t()
    }

    /// Adds or updates the given environment variables without affecting any
    /// keys that are not present in `envs`.
    ///
    /// Returns the complete map of environment variables after the upsert.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with
    /// [`ApiErrorCode::RegexValidation`](crate::ApiErrorCode::RegexValidation)
    /// if a key or value is rejected, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn upsert_envs(
        &self,
        envs: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::post_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"envs": envs}))
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    /// Replaces all environment variables with exactly the given map.
    ///
    /// Any variables that exist on the application but are absent from `envs`
    /// will be deleted. Returns the resulting environment map.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn overwrite_envs(
        &self,
        envs: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::overwrite_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"envs": envs}))
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    /// Deletes the environment variables whose keys are listed in `envs`.
    ///
    /// Keys that do not exist are silently ignored. Returns the environment
    /// map after the deletions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn delete_envs(
        &self,
        envs: &[String],
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::delete_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"envs": envs}))
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }
}
