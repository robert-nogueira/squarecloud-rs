use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Snapshot, SnapshotReference},
};

use super::DatabaseResource;

impl DatabaseResource {
    /// Returns all snapshots taken of this database.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn list_snapshots(&self) -> Result<Vec<Snapshot>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_database_snapshots(&self.id))
            .await?
            .into_result_t()
    }

    /// Takes a new snapshot of the database and returns a reference to it.
    ///
    /// Returns a [`SnapshotReference`] containing the download URL and storage
    /// key for the snapshot archive.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn create_snapshot(
        &self,
    ) -> Result<SnapshotReference, ApiError> {
        self.client
            .request_endpoint(Endpoint::create_database_snapshot(&self.id))
            .await?
            .into_result_t()
    }

    /// Restores the database to the state captured in the specified snapshot.
    ///
    /// Use [`Snapshot::name`](crate::types::Snapshot::name) as `snapshot_id`
    /// and [`Snapshot::version_id`](crate::types::Snapshot::version_id) as
    /// `version_id`, both from a value returned by
    /// [`list_snapshots`](DatabaseResource::list_snapshots). Returns `Ok(true)`
    /// when the restore has been initiated.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// if the snapshot or version is not found.
    pub async fn restore_snapshot(
        &self,
        snapshot_id: String,
        version_id: String,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::restore_database_snapshot(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"snapshotId": snapshot_id, "versionId": version_id}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
