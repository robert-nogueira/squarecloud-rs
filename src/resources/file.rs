use crate::{
    Endpoint,
    http::{
        ApiClient,
        errors::{ApiError, FileErrorCode},
    },
    types::{FileContent, FileInfo},
};

/// A handle to a file or directory in a SquareCloud application's filesystem.
///
/// Obtain a `FileResource` by calling [`AppResource::file`] with the desired
/// path. The path stored in [`FileResource::path`] is the default target for
/// [`write`](FileResource::write), [`delete`](FileResource::delete), and
/// [`move_to`](FileResource::move_to). The [`read`](FileResource::read) and
/// [`all_files`](FileResource::all_files) methods accept an explicit path
/// argument so that a single handle can be used for broader operations.
///
/// [`AppResource::file`]: crate::resources::AppResource::file
pub struct FileResource {
    /// The file path this handle was created with.
    pub path: String,
    /// The ID of the application this file belongs to.
    pub app_id: String,
    pub(crate) client: ApiClient,
}

impl FileResource {
    /// Creates a new `FileResource` scoped to `path` within the application
    /// identified by `app_id`.
    ///
    /// Prefer [`AppResource::file`](crate::resources::AppResource::file) over
    /// calling this directly.
    pub fn new(api: ApiClient, path: &str, app_id: &str) -> Self {
        Self {
            client: api,
            app_id: app_id.to_string(),
            path: path.to_string(),
        }
    }

    /// Writes `content` to the file at the path stored in this handle.
    ///
    /// If the file does not exist it is created; if it exists it is
    /// overwritten. Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn write(
        &self,
        content: &str,
    ) -> Result<bool, ApiError<FileErrorCode>> {
        let endpoint =
            Endpoint::put_app_file(&self.app_id, &self.path, content);
        self.client
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }

    /// Reads the file at `path` within the application's filesystem.
    ///
    /// `path` does not need to match [`FileResource::path`]; any path within
    /// the same application is valid. Returns a [`FileContent`] containing the
    /// raw bytes and MIME type.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if the file does not exist.
    pub async fn read(
        &self,
        path: &str,
    ) -> Result<FileContent, ApiError<FileErrorCode>> {
        self.client
            .request_endpoint(Endpoint::read_app_file(&self.app_id, path))
            .await?
            .into_result_t()
    }

    /// Deletes the file at the path stored in this handle.
    ///
    /// Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if the file does not exist.
    pub async fn delete(&self) -> Result<bool, ApiError<FileErrorCode>> {
        let endpoint = Endpoint::delete_app_file(&self.app_id, &self.path);
        self.client
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }

    /// Moves (renames) the file from the path stored in this handle to
    /// `destination_path`.
    ///
    /// Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if the source does not exist or the destination is invalid.
    pub async fn move_to(
        &self,
        destination_path: &str,
    ) -> Result<bool, ApiError<FileErrorCode>> {
        let endpoint = Endpoint::move_app_file(
            &self.app_id,
            &self.path,
            destination_path,
        );

        self.client
            .request_endpoint::<bool>(endpoint)
            .await?
            .into_bool_result()
    }

    /// Searches a slice of `FileResource` handles for one whose
    /// [`path`](FileResource::path) matches `path`.
    ///
    /// Returns `Some(&FileResource)` on the first match, or `None` if no
    /// handle has that path. This is a pure in-memory lookup; no network
    /// request is made.
    pub fn find_by_path<'a>(
        files: &'a [FileResource],
        path: &'a str,
    ) -> Option<&'a FileResource> {
        files.iter().find(|file| file.path == path)
    }

    /// Lists all files and directories under `path` in the application's
    /// filesystem.
    ///
    /// `path` does not need to match [`FileResource::path`]; any directory
    /// within the same application is valid.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// if `path` is not a directory or does not exist.
    pub async fn all_files(
        &self,
        path: &str,
    ) -> Result<Vec<FileInfo>, ApiError<FileErrorCode>> {
        let endpoint = Endpoint::list_app_files(&self.app_id, path);
        self.client
            .request_endpoint(endpoint)
            .await?
            .into_result_t()
    }
}

#[cfg(test)]
mod tests {
    use super::FileResource;
    use crate::http::ApiClient;

    fn make_file(path: &str) -> FileResource {
        unsafe { std::env::set_var("API_TOKEN", "test") };
        FileResource {
            path: path.to_string(),
            app_id: "app-123".to_string(),
            client: ApiClient::new(),
        }
    }

    #[test]
    fn find_by_path_returns_match() {
        let files =
            vec![make_file("/app/main.py"), make_file("/app/config.json")];
        let found = FileResource::find_by_path(&files, "/app/main.py");
        assert!(found.is_some());
        assert_eq!(found.unwrap().path, "/app/main.py");
    }

    #[test]
    fn find_by_path_returns_none_when_missing() {
        let files = vec![make_file("/app/main.py")];
        assert!(FileResource::find_by_path(&files, "/app/other.py").is_none());
    }
}
