use serde_json::{Value, json};

use crate::{
    Endpoint,
    http::{ApiResponse, errors::ApiError},
    types::CredentialType,
};

use super::DatabaseResource;

impl DatabaseResource {
    /// Returns the PEM-encoded TLS client certificate for the database.
    ///
    /// Use this certificate when establishing a TLS-authenticated connection
    /// to the database server.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn certificate(&self) -> Result<String, ApiError> {
        let endpoint = Endpoint::get_database_certificate(&self.id);
        let response: ApiResponse<Value> =
            self.client.request_endpoint(endpoint).await?;
        let value = response.into_result_t()?;
        let certificate =
            value.get("certificate").and_then(Value::as_str).unwrap();
        Ok(certificate.to_string())
    }

    /// Rotates the specified credential and returns the new value.
    ///
    /// Pass [`CredentialType::Password`] to generate a new password, or
    /// [`CredentialType::Certificate`] to regenerate the TLS client
    /// certificate. The returned string is the new password or PEM
    /// certificate, respectively.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn redefine_credential(
        &self,
        credential_type: CredentialType,
    ) -> Result<String, ApiError> {
        let endpoint = Endpoint::redefine_database_credentials(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"reset": credential_type.as_str()}))
            .build()?;
        let response: ApiResponse<Value> =
            self.client.execute_request(request).await?;
        let value = response.into_result_t()?;
        if let Some(password) = value.get("password").and_then(Value::as_str) {
            return Ok(password.to_string());
        }
        let certificate =
            value.get("certificate").and_then(Value::as_str).unwrap();
        Ok(certificate.to_string())
    }
}
