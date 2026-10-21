use serde_json::json;

use crate::{
    Endpoint,
    http::{
        NoData,
        errors::{ApiError, SnapshotErrorCode},
    },
    types::{RestoreStatus, Snapshot, SnapshotReference},
};

use super::AppResource;

impl AppResource {
    /// Returns all snapshots that have been taken of this application.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn list_snapshots(
        &self,
    ) -> Result<Vec<Snapshot>, ApiError<SnapshotErrorCode>> {
        self.client
            .request_endpoint(Endpoint::list_app_snapshots(&self.id))
            .await?
            .into_result_t()
    }

    /// Triggers a new snapshot of the application's current state.
    ///
    /// Returns a [`SnapshotReference`] containing the download URL and signed
    /// key for the snapshot archive.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn create_snapshot(
        &self,
    ) -> Result<SnapshotReference, ApiError<SnapshotErrorCode>> {
        self.client
            .request_endpoint(Endpoint::app_create_snapshot(&self.id))
            .await?
            .into_result_t()
    }

    /// Restores the application to a previously saved snapshot.
    ///
    /// Use [`Snapshot::name`](crate::types::Snapshot::name) as `snapshot_id`
    /// and [`Snapshot::version_id`](field@crate::types::Snapshot::version_id) as
    /// `version_id`, both from a value returned by
    /// [`list_snapshots`](AppResource::list_snapshots). Returns `Ok(true)`
    /// when the restore has finished.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] if the snapshot or version is not
    /// found.
    ///
    /// A large snapshot can outlive the API's ~90 s response deadline. The
    /// restore then keeps running in the background and this method returns
    /// [`SnapshotErrorCode::SnapshotRestoreProcessing`], which is not a
    /// failure: follow it with [`restore_status`](AppResource::restore_status)
    /// instead of calling this again.
    pub async fn restore_snapshot(
        &self,
        snapshot_id: String,
        version_id: String,
    ) -> Result<bool, ApiError<SnapshotErrorCode>> {
        let endpoint = Endpoint::restore_app_snapshot(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"snapshotId": snapshot_id, "versionId": version_id}))
            .build()?;
        self.client
            .execute_request::<NoData>(request)
            .await?
            .into_bool_result()
    }

    /// Returns the state of the last snapshot restore on this resource.
    ///
    /// Use it to follow a restore that
    /// [`restore_snapshot`](AppResource::restore_snapshot) reported as
    /// [`SnapshotErrorCode::SnapshotRestoreProcessing`].
    ///
    /// A `success` state does not free the resource right away: for about
    /// two more minutes, actions such as delete keep answering
    /// `RESTORE_IN_PROGRESS`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] if the request fails or the reply does
    /// not decode, or [`ApiError::Service`] on an API-level error.
    pub async fn restore_status(
        &self,
    ) -> Result<RestoreStatus, ApiError<SnapshotErrorCode>> {
        self.client
            .request_endpoint(Endpoint::app_restore_status(&self.id))
            .await?
            .into_result_t()
    }
}
