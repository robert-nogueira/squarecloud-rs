use std::borrow::Cow;

use async_stream::stream;
use futures_util::StreamExt;
use reqwest::multipart::{Form, Part};

use crate::{
    Endpoint,
    http::{
        ApiResponse, Client, NoData,
        errors::{ApiError, AppErrorCode, CommitError, ServiceErrorCode},
    },
    resources::FileResource,
    types::{
        AppInfo, AppLogs, AppMetrics, LogStream, RealtimeEvent, RuntimeStats,
        StatusFrame, StatusMerger,
    },
};

/// The stream returned by [`AppResource::realtime`].
pub type RealtimeStream = futures_util::stream::BoxStream<
    'static,
    Result<RealtimeEvent, ApiError<AppErrorCode>>,
>;

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
/// | `snapshots.rs` | snapshot management |
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
    ///         other => println!("[other] {other:?}"),
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
    ///
    /// If the API refuses the connection, the stream yields a single
    /// [`ApiError::Service`] (for example
    /// [`AppErrorCode::RealtimeMaxConnections`] or
    /// [`AppErrorCode::RealtimeMaxConnectionsApp`] when too many streams are
    /// open) and ends; a refusal without an API envelope yields
    /// [`ApiError::Transport`] carrying the HTTP status. An event the client
    /// does not know, or a status frame that does not decode, arrives as
    /// [`RealtimeEvent::Unrecognized`] and the stream keeps running.
    pub fn realtime(&self) -> RealtimeStream {
        let client = self.client.clone();
        let id = self.id.clone();
        let endpoint = Endpoint::sse_realtime_app_logs(&id);

        Box::pin(stream! {
            let response = client
                .http_client
                .request(endpoint.method, client.url(&endpoint.path))
                .send()
                .await
                .map_err(ApiError::Transport)?;

            // A refused connection answers with a JSON error envelope, not
            // an event stream, sometimes even with a 2xx status.
            let is_json = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    let value = value.to_ascii_lowercase();
                    value.starts_with("application/")
                        && value.split(';').next().is_some_and(|mime| {
                            mime.trim_end().ends_with("json")
                        })
                });
            if !response.status().is_success() || is_json {
                // Keeps the HTTP status for a refusal without an envelope.
                let status_error = response.error_for_status_ref().err();
                match response.json::<ApiResponse<NoData>>().await {
                    Ok(envelope) => {
                        if let Err(e) = envelope.into_result_t() {
                            yield Err(e);
                        } else if let Some(e) = status_error {
                            yield Err(ApiError::Transport(e));
                        }
                    }
                    Err(decode) => {
                        yield Err(ApiError::Transport(
                            status_error.unwrap_or(decode),
                        ));
                    }
                }
                return;
            }

            let mut bytes = response.bytes_stream();

            let mut buffer = Vec::<u8>::new();
            let mut event_type = String::new();
            let mut data = String::new();
            let mut has_data = false;
            let mut status_merger = StatusMerger::default();
            let mut ended = false;

            while !ended {
                match bytes.next().await {
                    Some(chunk) => {
                        buffer.extend_from_slice(
                            &chunk.map_err(ApiError::Transport)?,
                        );
                    }
                    // Flush an event the server closed without its blank
                    // line.
                    None => {
                        buffer.extend_from_slice(b"\n\n");
                        ended = true;
                    }
                }

                while let Some(pos) =
                    buffer.iter().position(|&b| b == b'\n')
                {
                    let line =
                        String::from_utf8_lossy(&buffer[..pos]).into_owned();
                    buffer.drain(..=pos);
                    let line = line.trim_end_matches('\r');

                    if line.is_empty() {
                        if !has_data {
                            event_type.clear();
                            continue;
                        }
                        has_data = false;
                        let event = std::mem::take(&mut event_type);
                        let payload = std::mem::take(&mut data);
                        match event.as_str() {
                            "logs" => {
                                let stream = match payload.chars().next() {
                                    Some('\u{2}') => LogStream::Stderr,
                                    _ => LogStream::Stdout,
                                };
                                let line = payload
                                    .strip_prefix(['\u{1}', '\u{2}'])
                                    .unwrap_or(&payload)
                                    .to_string();
                                yield Ok(RealtimeEvent::Log { stream, line });
                            }
                            "status" => {
                                match serde_json::from_str::<StatusFrame>(&payload) {
                                    Ok(frame) => {
                                        yield Ok(RealtimeEvent::Status(
                                            status_merger.merge(frame),
                                        ));
                                    }
                                    Err(_) => {
                                        yield Ok(RealtimeEvent::Unrecognized {
                                            event,
                                            data: payload,
                                        });
                                    }
                                }
                            }
                            "error" => {
                                yield Err(ApiError::Service {
                                    code: AppErrorCode::from_wire(payload),
                                });
                                return;
                            }
                            "" | "system" => {
                                yield Ok(RealtimeEvent::System(payload));
                            }
                            _ => {
                                yield Ok(RealtimeEvent::Unrecognized {
                                    event,
                                    data: payload,
                                });
                            }
                        }
                    } else if let Some(val) = line.strip_prefix("event:") {
                        event_type = val.trim().to_string();
                    } else if let Some(val) = line.strip_prefix("data:") {
                        // SSE strips only the one space after the colon.
                        if has_data {
                            data.push('\n');
                        }
                        data.push_str(val.strip_prefix(' ').unwrap_or(val));
                        has_data = true;
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    ///
    /// Safe to retry, checked with [`ApiError::is_retryable`]:
    /// [`AppErrorCode::RestoreInProgress`] (a snapshot restore is running,
    /// and the lock can outlast `restore_status` reporting success),
    /// [`AppErrorCode::KeepCalm`], [`AppErrorCode::RateLimited`] and
    /// [`AppErrorCode::DatabaseUnavailable`]. [`AppErrorCode::ActionFailed`]
    /// and the `Container*` codes describe the container's state and are
    /// not classified as retryable.
    pub async fn start(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<NoData>(Endpoint::app_start(&self.id))
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    ///
    /// Safe to retry, checked with [`ApiError::is_retryable`]:
    /// [`AppErrorCode::RestoreInProgress`] (a snapshot restore is running,
    /// and the lock can outlast `restore_status` reporting success),
    /// [`AppErrorCode::KeepCalm`], [`AppErrorCode::RateLimited`] and
    /// [`AppErrorCode::DatabaseUnavailable`]. [`AppErrorCode::ActionFailed`]
    /// and the `Container*` codes describe the container's state and are
    /// not classified as retryable.
    pub async fn restart(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<NoData>(Endpoint::app_restart(&self.id))
            .await?
            .into_bool_result()
    }

    /// Stops the application.
    ///
    /// Returns `Ok(true)` when the stop command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    ///
    /// Safe to retry, checked with [`ApiError::is_retryable`]:
    /// [`AppErrorCode::RestoreInProgress`] (a snapshot restore is running,
    /// and the lock can outlast `restore_status` reporting success),
    /// [`AppErrorCode::KeepCalm`], [`AppErrorCode::RateLimited`] and
    /// [`AppErrorCode::DatabaseUnavailable`]. [`AppErrorCode::ActionFailed`]
    /// and the `Container*` codes describe the container's state and are
    /// not classified as retryable.
    pub async fn stop(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<NoData>(Endpoint::app_stop(&self.id))
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
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
    /// Returns [`CommitError::Api`] wrapping [`ApiError::Transport`] if the
    /// request cannot be built, fails or the reply does not decode, or
    /// [`CommitError::Api`] wrapping [`ApiError::Service`] if the archive is
    /// rejected by the API.
    ///
    /// Inside [`CommitError::Api`], the codes safe to retry (checked with
    /// [`ApiError::is_retryable`] on the wrapped error) are the
    /// [`UploadErrorCode`](crate::errors::UploadErrorCode) variants
    /// `UploadBusy`, `KeepCalm`, `RateLimited` and `DatabaseUnavailable`.
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
    /// Returns [`CommitError::Api`] wrapping [`ApiError::Transport`] if the
    /// request cannot be built, fails or the reply does not decode, or
    /// [`CommitError::Api`] wrapping [`ApiError::Service`] with
    /// [`UploadErrorCode::InvalidPath`](crate::errors::UploadErrorCode) if
    /// `path` contains traversal or shell metacharacters, or another code if
    /// the archive is rejected by the API.
    ///
    /// Inside [`CommitError::Api`], the codes safe to retry (checked with
    /// [`ApiError::is_retryable`] on the wrapped error) are the
    /// [`UploadErrorCode`](crate::errors::UploadErrorCode) variants
    /// `UploadBusy`, `KeepCalm`, `RateLimited` and `DatabaseUnavailable`.
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
            .execute_request::<NoData>(request)
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
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    ///
    /// Safe to retry, checked with [`ApiError::is_retryable`]:
    /// [`AppErrorCode::RestoreInProgress`] (a snapshot restore is running,
    /// and the lock can outlast `restore_status` reporting success),
    /// [`AppErrorCode::KeepCalm`], [`AppErrorCode::RateLimited`] and
    /// [`AppErrorCode::DatabaseUnavailable`]. [`AppErrorCode::DeleteFailed`]
    /// is not classified as retryable.
    pub async fn delete(&self) -> Result<bool, ApiError<AppErrorCode>> {
        self.client
            .request_endpoint::<NoData>(Endpoint::app_delete(&self.id))
            .await?
            .into_bool_result()
    }
}
