use serde::Deserialize;
use serde_json::json;

use crate::{
    Endpoint,
    http::{
        ApiResponse, NoData,
        errors::{ApiError, DatabaseErrorCode},
    },
    types::CredentialType,
};

use super::DatabaseResource;

impl DatabaseResource {
    /// Returns the database's TLS file for client connections,
    /// base64-encoded: decoding it gives a PEM holding the private key
    /// followed by the certificate.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn certificate(
        &self,
    ) -> Result<String, ApiError<DatabaseErrorCode>> {
        let endpoint = Endpoint::get_database_certificate(&self.id);
        #[derive(Deserialize)]
        struct Certificate {
            certificate: String,
        }
        let response: ApiResponse<Certificate> =
            self.client.request_endpoint(endpoint).await?;
        Ok(response.into_result_t()?.certificate)
    }

    /// Rotates the specified credential and returns the new value.
    ///
    /// Pass [`CredentialType::Password`] to generate a new password, or
    /// [`CredentialType::Certificate`] to regenerate the TLS client
    /// certificate. The returned string is the new password, or the new TLS
    /// file in the same format as [`certificate`](DatabaseResource::certificate).
    ///
    /// The API does not return a regenerated certificate, so this method
    /// fetches it with [`certificate`](DatabaseResource::certificate) after
    /// the reset. If that second request fails, the reset already happened:
    /// call `certificate` instead of resetting again.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn redefine_credential(
        &self,
        credential_type: CredentialType,
    ) -> Result<String, ApiError<DatabaseErrorCode>> {
        let endpoint = Endpoint::redefine_database_credentials(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"reset": credential_type.as_str()}))
            .build()?;
        match credential_type {
            CredentialType::Password => {
                #[derive(Deserialize)]
                struct NewPassword {
                    password: String,
                }
                let response: ApiResponse<NewPassword> =
                    self.client.execute_request(request).await?;
                Ok(response.into_result_t()?.password)
            }
            CredentialType::Certificate => {
                self.client
                    .execute_request::<NoData>(request)
                    .await?
                    .into_bool_result()?;
                self.certificate().await
            }
        }
    }
}
