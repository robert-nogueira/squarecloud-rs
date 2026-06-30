use std::sync::Arc;

use serde_json::{Map, Value};

use crate::{
    Endpoint,
    http::{ApiClient, errors::ApiError},
    types::{DatabaseInfo, DatabaseMetrics, DatabaseStatus},
};

/// A handle to a specific SquareCloud managed database.
///
/// Obtain a `DatabaseResource` by calling [`ApiClient::database`] with the
/// database ID, or via
/// [`Database::into_resource`](crate::types::Database::into_resource).
///
/// Methods are spread across multiple `impl` blocks in submodules:
///
/// | Source file | Methods |
/// |-------------|---------|
/// | `resource.rs` | lifecycle, status, metrics, edit, delete |
/// | `credentials.rs` | TLS certificate, credential rotation |
/// | `snapshots.rs` | snapshot management |
pub struct DatabaseResource {
    /// Shared reference to the underlying HTTP client.
    pub client: Arc<ApiClient>,
    /// The database's unique identifier.
    pub id: String,
}

impl DatabaseResource {
    /// Creates a new `DatabaseResource` bound to the given client and database
    /// ID.
    ///
    /// Prefer [`ApiClient::database`] over calling this directly.
    pub fn new(http: Arc<ApiClient>, id: &str) -> Self {
        Self {
            client: http,
            id: id.to_string(),
        }
    }

    /// Returns static metadata about the database.
    ///
    /// The returned [`DatabaseInfo`] includes the display name, owner ID,
    /// cluster, RAM allocation, engine type, port, and creation timestamp.
    /// Unlike [`status`](DatabaseResource::status), this does not reflect
    /// whether the database is currently running.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn info(&self) -> Result<DatabaseInfo, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_info(&self.id))
            .await?
            .into_result_t()
    }

    /// Starts the database instance.
    ///
    /// Returns `Ok(true)` when the start command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn start(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::start_database(&self.id))
            .await?
            .into_bool_result()
    }

    /// Stops the database instance.
    ///
    /// Returns `Ok(true)` when the stop command is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn stop(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::stop_database(&self.id))
            .await?
            .into_bool_result()
    }

    /// Returns the current runtime status of the database, including CPU and
    /// RAM usage, network throughput, and uptime.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn status(&self) -> Result<DatabaseStatus, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_status(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns historical resource-usage metrics for the database.
    ///
    /// Each [`DatabaseMetrics`] entry covers a discrete time window and
    /// reports CPU percentage, RAM percentage, and network byte counts.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn metrics(&self) -> Result<Vec<DatabaseMetrics>, ApiError> {
        self.client
            .request_endpoint(Endpoint::database_metrics(&self.id))
            .await?
            .into_result_t()
    }

    /// Updates the database's display name and/or RAM allocation.
    ///
    /// At least one of `name` or `ram` must be `Some`; if both are `None`,
    /// the method returns `Ok(false)` immediately without making a request.
    /// Returns `Ok(true)` when the update is accepted.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with
    /// [`ApiErrorCode::InvalidMemory`](crate::ApiErrorCode::InvalidMemory) or
    /// [`ApiErrorCode::FewMemory`](crate::ApiErrorCode::FewMemory) if the new
    /// memory value is not permitted, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn edit(
        &self,
        name: Option<&str>,
        ram: Option<u32>,
    ) -> Result<bool, ApiError> {
        if name.is_none() && ram.is_none() {
            return Ok(false);
        }
        let mut map = Map::new();
        if name.is_some() {
            map.insert(
                "name".to_string(),
                Value::String(name.unwrap().to_string()),
            );
        }
        if ram.is_some() {
            map.insert("ram".to_string(), Value::Number(ram.unwrap().into()));
        }
        let payload = Value::Object(map);
        let endpoint = Endpoint::edit_database(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&payload)
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    /// Permanently deletes the database and all its data.
    ///
    /// This action cannot be undone. Returns `Ok(true)` on success.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn delete(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint(Endpoint::delete_database(&self.id))
            .await?
            .into_result_t()
    }
}
