use serde_json::Value;

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
};

use super::DatabaseResource;

impl DatabaseResource {
    pub async fn certificate(&self) -> Result<String, ApiError> {
        let endpoint = Endpoint::get_database_certificate(&self.id);
        let response: ApiResponse<Value> =
            self.client.request_endpoint(endpoint).await?;
        let value = response.into_result_t()?;
        let certificate =
            value.get("certificate").and_then(Value::as_str).unwrap();
        Ok(certificate.to_string())
    }
}
