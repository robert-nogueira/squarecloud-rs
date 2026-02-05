use serde_json::{Value, json};

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
    types::CredentialType,
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

    pub async fn redefine_credential(
        &self,
        credential_type: CredentialType,
    ) -> Result<String, ApiError> {
        let endpoint = Endpoint::redefine_database_credentials(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"reset": credential_type.as_str()}))
            .build()?;
        let response: ApiResponse<Value> =
            self.client.execute_request(request).await?;
        let value = response.into_result_t()?;
        if let Some(password) = value.get("password") {
            return Ok(password.to_string());
        }
        let certificate =
            value.get("certificate").and_then(Value::as_str).unwrap();
        Ok(certificate.to_string())
    }
}
