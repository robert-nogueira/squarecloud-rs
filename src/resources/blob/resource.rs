use std::borrow::Cow;

use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        Client, NoData,
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
    /// - `name` is the object name, without extension: 1 to 128 letters,
    ///   digits, `_`, `.` and `-`, starting with a letter, digit or `_`.
    /// - `file_name` is the file's real name with its extension (for example
    ///   `"logo.png"` or `"reads.fastq.gz"`). The API takes the stored
    ///   extension from it and derives the served `Content-Type` from that
    ///   extension. A path is reduced to its last component.
    /// - `bytes` is the raw file content.
    /// - `options` controls optional metadata such as prefix, expiry, and
    ///   security settings.
    ///
    /// Returns the [`BlobObject`] descriptor. Its `url` is `None` when the
    /// object is private.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error (e.g.
    /// [`BlobErrorCode::InvalidObjectName`] or
    /// [`BlobErrorCode::BlockedFileType`] for executables).
    pub async fn upload(
        &self,
        name: &str,
        file_name: &str,
        bytes: impl Into<Cow<'static, [u8]>>,
        options: UploadOptions,
    ) -> Result<BlobObject, ApiError<BlobErrorCode>> {
        let endpoint = Endpoint::blob_upload(name, &options);
        let form = Form::new().part(
            "file",
            Part::bytes(bytes).file_name(base_name(file_name).to_string()),
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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
    /// Returns [`ApiError::Service`] with [`BlobErrorCode::ObjectNotFound`] if
    /// the key does not exist, or [`ApiError::Transport`] if the request fails
    /// or the reply does not decode.
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
            .execute_request::<NoData>(request)
            .await?
            .into_bool_result()
    }

    /// Returns usage, plan, and billing statistics for the blob storage
    /// account.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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

/// The last component of `file_name`, so a path such as
/// `"./images/logo.png"` is sent as `"logo.png"`.
fn base_name(file_name: &str) -> &str {
    std::path::Path::new(file_name)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(file_name)
}

#[cfg(test)]
mod tests {
    use super::base_name;

    #[test]
    fn base_name_strips_directories() {
        assert_eq!(base_name("./images/logo.png"), "logo.png");
        assert_eq!(base_name("logo.png"), "logo.png");
        assert_eq!(base_name("reads.fastq.gz"), "reads.fastq.gz");
    }

    #[test]
    fn base_name_keeps_input_without_a_final_component() {
        assert_eq!(base_name(".."), "..");
    }
}
