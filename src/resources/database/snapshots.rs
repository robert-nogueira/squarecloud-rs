use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Snapshot, SnapshotReference},
};

use super::DatabaseResource;

impl DatabaseResource {
    pub async fn list_snapshots(&self) -> Result<Snapshot, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_database_snapshots(&self.id))
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
}
