use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Snapshot, SnapshotReference},
};

use super::AppResource;

impl AppResource {
    pub async fn list_snapshots(&self) -> Result<Vec<Snapshot>, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_app_snapshots(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn create_snapshot(
        &self,
    ) -> Result<SnapshotReference, ApiError> {
        self.client
            .request_endpoint(Endpoint::create_database_snapshot(&self.id))
            .await?
            .into_result_t()
    }

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
