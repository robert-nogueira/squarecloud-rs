use std::collections::HashMap;

use serde_json::json;

use crate::{Endpoint, http::errors::ApiError};

use super::AppResource;

impl AppResource {
    pub async fn list_envs(
        &self,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_envs(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn upsert_envs(
        &self,
        envs: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::post_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"envs": envs}))
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    pub async fn overwrite_envs(
        &self,
        envs: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::overwrite_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"envs": envs}))
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    pub async fn delete_envs(
        &self,
        envs: &[String],
    ) -> Result<HashMap<String, String>, ApiError> {
        let endpoint = Endpoint::delete_app_envs(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(envs)
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }
}
