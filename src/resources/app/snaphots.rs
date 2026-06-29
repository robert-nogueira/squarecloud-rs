use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Snapshot, SnapshotReference},
};

use super::AppResource;

impl AppResource {
    /// Returns all snapshots that have been taken of this application.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn list_snapshots(&self) -> Result<Vec<Snapshot>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_snapshots(&self.id))
            .await?
            .into_result_t()
    }

    /// Triggers a new snapshot of the application's current state.
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
            .request_endpoint(Endpoint::app_create_snapshot(&self.id))
            .await?
            .into_result_t()
    }

    /// Restores the application to a previously saved snapshot.
    ///
    /// Both `snapshot_id` and `version_id` can be obtained from the
    /// [`Snapshot::key`](crate::types::Snapshot::key) field of values returned
    /// by [`list_snapshots`](AppResource::list_snapshots). Returns `Ok(true)`
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
        let endpoint = Endpoint::restore_app_snapshot(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"snapshotId": snapshot_id, "versionId": version_id}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
