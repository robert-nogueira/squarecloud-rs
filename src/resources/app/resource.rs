use std::borrow::Cow;

use async_stream::stream;
use futures_util::StreamExt;
use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        Client,
        errors::{ApiError, AppErrorCode, CommitError, ServiceErrorCode},
    },
    resources::FileResource,
    types::{
        AppInfo, AppLogs, AppMetrics, LogStream, RealtimeEvent, RuntimeStats,
        StatusFrame, StatusMerger,
    },
};

/// A handle to a specific SquareCloud application.
///
/// Obtain an `AppResource` by calling [`Client::app`] with the application
/// ID, or via [`AppInfo::into_resource`](crate::types::AppInfo::into_resource).
///
/// Methods are spread across multiple `impl` blocks in submodules:
///
/// | Source file | Methods |
/// |-------------|---------|
/// | `resource.rs` | lifecycle, status, logs, commit, delete |
/// | `deploy.rs` | deploy history, webhook integration |
/// | `env.rs` | environment variable management |
/// | `network.rs` | analytics, DNS, custom domain, cache purge |
/// | `snaphots.rs` | snapshot management |
pub struct AppResource {
    /// The application's unique identifier.
    pub id: String,
    pub(crate) client: Client,
}

impl AppResource {
    /// Creates a new `AppResource` bound to the given client and application
    /// ID.
    ///
    /// Prefer [`Client::app`] over calling this directly.
    pub fn new(http: Client, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    /// Opens a live SSE stream of log, status, and system events for this
    /// application.
    ///
    /// Yields [`RealtimeEvent::Log`] for application log lines (with the
    /// stdout/stderr origin already decoded), [`RealtimeEvent::System`] for
    /// connection lifecycle messages (e.g. `REALTIME_CONNECTING`,
    /// `REALTIME_CONNECTED`), and [`RealtimeEvent::Status`] for live
    /// container metrics (updated roughly once a second; prefer this over
    /// polling [`status`](AppResource::status), which can be up to about a
    /// minute stale while a realtime stream is open). The stream runs until
    /// the server closes the connection.
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    /// use squarecloud::{Client, RealtimeEvent};
    ///
    /// # #[tokio::main] async fn main() {
    /// let client = Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    /// let mut stream = client.app("your-app-id").realtime();
    /// while let Some(event) = stream.next().await {
    ///     match event.unwrap() {
    ///         RealtimeEvent::Log { stream, line } => {
    ///             println!("[log:{stream:?}] {line}")
    ///         }
    ///         RealtimeEvent::System(msg) => println!("[system] {msg}"),
    ///         RealtimeEvent::Status(status) => {
    ///             println!("[status] cpu={} ram={:?}", status.cpu, status.ram)
    ///         }
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Each item is `Result<RealtimeEvent, ApiError<AppErrorCode>>`. A
    /// transport failure mid-stream yields an `Err` and the stream
    /// terminates. The server can also send its own terminal error event
    /// (e.g. [`AppErrorCode::ContainerNotFound`] if the application stops or
    /// is deleted mid-stream), which surfaces the same way.
    pub fn realtime(
        &self,
    ) -> futures_util::stream::BoxStream<
        'static,
        Result<RealtimeEvent, ApiError<AppErrorCode>>,
    > {
        let client = self.client.clone();
        let id = self.id.clone();
        let endpoint = Endpoint::sse_realtime_app_logs(&id);

        Box::pin(stream! {
            let mut bytes = client
                .http_client
                .request(endpoint.method, client.url(&endpoint.path))
                .send()
                .await
                .map_err(ApiError::Transport)?
                .bytes_stream();

            let mut buffer = Vec::<u8>::new();
            let mut event_type = String::new();
            let mut data = String::new();
            let mut status_merger = StatusMerger::default();

            while let Some(chunk) = bytes.next().await {
                let chunk = chunk.map_err(ApiError::Transport)?;
                buffer.extend_from_slice(&chunk);

                while let Some(pos) =
                    buffer.iter().position(|&b| b == b'\n')
                {
                    let line =
                        String::from_utf8_lossy(&buffer[..pos]).into_owned();
                    buffer.drain(..=pos);
                    let line = line.trim_end_matches('\r');

                    if line.is_empty() {
                        if !data.is_empty() {
                            match event_type.as_str() {
                                "logs" => {
                                    let stream = match data.chars().next() {
                                        Some('\u{2}') => LogStream::Stderr,
                                        _ => LogStream::Stdout,
                                    };
                                    let text = data
                                        .strip_prefix(['\u{1}', '\u{2}'])
                                        .unwrap_or(&data)
                                        .to_string();
                                    yield Ok(RealtimeEvent::Log {
                                        stream,
                                        line: text,
                                    });
                                }
                                "status" => {
                                    if let Ok(frame) =
                                        serde_json::from_str::<StatusFrame>(&data)
                                    {
                                        yield Ok(RealtimeEvent::Status(
                                            status_merger.merge(frame),
                                        ));
                                    }
                                }
                                "error" => {
                                    yield Err(ApiError::Service {
                                        code: AppErrorCode::from_wire(
                                            data.clone(),
                                        ),
                                    });
                                    return;
                                }
                                _ => {
                                    yield Ok(RealtimeEvent::System(data.clone()));
                                }
                            }
                        }
                        event_type.clear();
                        data.clear();
                    } else if let Some(val) = line.strip_prefix("event:") {
                        event_type = val.trim().to_string();
                    } else if let Some(val) = line.strip_prefix("data:") {
                        if !data.is_empty() {
                            data.push('\n');
                        }
                        data.push_str(val.trim());
                    }
                }
            }
        })
    }

    /// Returns a [`FileResource`] scoped to the given path within this
    /// application's filesystem.
    pub fn file(&self, path: &str) -> FileResource {
        FileResource::new(self.client.clone(), path, &self.id)
    }

    /// Starts the application.
    ///
    /// Returns `Ok(true)` when the start command is accepted by the platform.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn start(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::app_start(&self.id))
            .await?
            .into_bool_result()
    }

    /// Restarts the application.
    ///
    /// The application is stopped and then started again. Returns `Ok(true)`
    /// when the restart command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn restart(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
    }

    /// Stops the application.
    ///
    /// Returns `Ok(true)` when the stop command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn stop(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::app_stop(&self.id))
            .await?
            .into_bool_result()
    }

    /// Returns the current runtime status of the application.
    ///
    /// The returned [`RuntimeStats`] includes CPU usage, RAM usage, storage,
    /// network throughput, and the time the process last started.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn status(
        &self,
    ) -> Result<RuntimeStats, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint(Endpoint::app_status(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns static metadata about the application.
    ///
    /// The returned [`AppInfo`] includes the display name, owner ID, cluster,
    /// memory allocation, and language runtime. Unlike
    /// [`status`](AppResource::status), this does not reflect whether the
    /// application is currently running.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn info(&self) -> Result<AppInfo, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint(Endpoint::app_info(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns historical resource-usage metrics for the application.
    ///
    /// Each [`AppMetrics`] entry covers a 5-minute window. Up to 288 data
    /// points (24 hours) are returned. Results are cached for 2.5 minutes
    /// per application.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn metrics(
        &self,
    ) -> Result<Vec<AppMetrics>, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint(Endpoint::app_metrics(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns the most recent log output from the application as a plain
    /// string.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn logs(&self) -> Result<String, ApiError<AppErrorCode>> {
        let r: AppLogs = self
            .client
            .request_endpoint(Endpoint::app_logs(&self.id))
            .await?
            .into_result_t()?;
        Ok(r.logs)
    }

    /// Commits a new version of the application by uploading a ZIP archive.
    ///
    /// The archive is sent as a `multipart/form-data` request and unpacked
    /// at the application root. The `bytes` parameter accepts anything that
    /// converts to a `Cow<'static, [u8]>`, such as a `Vec<u8>`.
    ///
    /// Unlike [`Client::upload_app`](crate::Client::upload_app),
    /// `commit` updates an existing application in place rather than creating
    /// a new one.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::Io`] if constructing the multipart request
    /// fails, [`CommitError::Api`] wrapping [`ApiError::Transport`] on network
    /// failure, or [`CommitError::Api`] wrapping [`ApiError::Service`] if the
    /// archive is rejected by the API.
    pub async fn commit(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
    ) -> Result<bool, CommitError> {
        self.commit_to(bytes, None).await
    }

    /// Commits a new version of the application, unpacking the ZIP archive
    /// into `path` instead of the application root.
    ///
    /// `path` is a destination directory inside the application (no
    /// traversal, no shell metacharacters). Equivalent to `commit` with
    /// `path` set to `None`.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::Io`] if constructing the multipart request
    /// fails, [`CommitError::Api`] wrapping [`ApiError::Transport`] on
    /// network failure, or [`CommitError::Api`] wrapping
    /// [`ApiError::Service`] with
    /// [`UploadErrorCode::InvalidPath`](crate::errors::UploadErrorCode) if
    /// `path` contains traversal or shell metacharacters, or another code
    /// if the archive is rejected by the API.
    pub async fn commit_to(
        &self,
        bytes: impl Into<Cow<'static, [u8]>>,
        path: Option<&str>,
    ) -> Result<bool, CommitError> {
        let endpoint = Endpoint::app_commit(&self.id, path);
        let form = Form::new().part(
            "file",
            Part::bytes(bytes)
                .file_name("app.zip")
                .mime_str("application/zip")
                .unwrap(),
        );

        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .multipart(form)
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
            .map_err(CommitError::Api)
    }

    /// Permanently deletes the application and all associated data.
    ///
    /// This action cannot be undone. Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn delete(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::app_delete(&self.id))
            .await?
            .into_bool_result()
    }
}
