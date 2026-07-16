use std::borrow::Cow;

use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        Client,
        errors::{ApiError, BlobErrorCode},
    },
    types::{BlobObject, BlobObjectList, BlobStats, UploadOptions},
};

/// A handle to the SquareCloud Blob Storage API.
///
/// Obtain a `BlobResource` by calling [`Client::blob`].
pub struct BlobResource {
    pub(crate) client: Client,
}

impl BlobResource {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Uploads an object to blob storage.
    ///
    /// - `name` is the logical storage key. Must match `[a-zA-Z0-9_]`, 3-32
    ///   characters (no dots, dashes, or slashes).
    /// - `mime_type` is the MIME type of the content (e.g. `"image/png"`).
    ///   The name cannot carry an extension (dots are not allowed), so this
    ///   must always be supplied explicitly.
    /// - `bytes` is the raw file content.
    /// - `options` controls optional metadata such as prefix, expiry, and
    ///   security settings.
    ///
    /// Returns the [`BlobObject`] descriptor including the public URL.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error (e.g. [`BlobErrorCode::InvalidObjectName`]).
    pub async fn upload(
        &self,
        name: &str,
        mime_type: &str,
        bytes: impl Into<Cow<'static, [u8]>>,
        options: UploadOptions,
    ) -> Result<BlobObject, ApiError<BlobErrorCode>> {
        let endpoint = Endpoint::blob_upload(name, &options);
        let form = Form::new().part(
            "file",
            Part::bytes(bytes)
                .file_name(name.to_string())
                .mime_str(mime_type)
                .unwrap(),
        );
        let request = endpoint
            .request_builder(
                &self.client.http_client,
                &self.client.blob_base_url,
            )
            .multipart(form)
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    /// Lists objects stored in blob storage.
    ///
    /// - `prefix` filters results to objects whose key starts with the given
    ///   string (e.g. `"images/"`).
    /// - `continuation_token` resumes pagination from where a previous call
    ///   left off.
    ///
    /// Returns a [`BlobObjectList`] containing the objects and an optional
    /// token for the next page.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn list(
        &self,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
    ) -> Result<BlobObjectList, ApiError<BlobErrorCode>> {
        let endpoint = Endpoint::blob_list(prefix, continuation_token);
        let request = endpoint
            .request_builder(
                &self.client.http_client,
                &self.client.blob_base_url,
            )
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }

    /// Deletes a stored object by its key or ID.
    ///
    /// Returns `Ok(true)` when the object was successfully deleted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Service`] with [`BlobErrorCode::ObjectNotFound`]
    /// if the key does not exist, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn delete(
        &self,
        object: &str,
    ) -> Result<bool, ApiError<BlobErrorCode>> {
        let endpoint = Endpoint::blob_delete(object);
        let request = endpoint
            .request_builder(
                &self.client.http_client,
                &self.client.blob_base_url,
            )
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    /// Returns usage, plan, and billing statistics for the blob storage
    /// account.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn stats(&self) -> Result<BlobStats, ApiError<BlobErrorCode>> {
        let endpoint = Endpoint::blob_stats();
        let request = endpoint
            .request_builder(
                &self.client.http_client,
                &self.client.blob_base_url,
            )
            .build()?;
        self.client.execute_request(request).await?.into_result_t()
    }
}
