use serde_json::Value;

use crate::{Endpoint, http::errors::ApiError, types::Env};

use super::AppResource;

impl AppResource {
    pub async fn list_env_vars(&self) -> Result<Env, ApiError> {
        let response: Value = self
            .api
            .request_endpoint(Endpoint::list_app_envs(&self.id))
            .await?
            .into_result_t()?;

        let map = response.as_object().unwrap();
        let (key, value) = map.iter().next().unwrap();
        let value_str = value
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| value.to_string());

        Ok(Env {
            key: key.clone(),
            value: value_str.to_string(),
        })
    }
}
