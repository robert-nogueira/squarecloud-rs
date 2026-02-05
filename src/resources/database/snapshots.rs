use crate::{Endpoint, http::errors::ApiError, types::Snapshot};

use super::DatabaseResource;

impl DatabaseResource {
    pub async fn list_snapshots(&self) -> Result<Snapshot, ApiError> {
        self.client
            .request_endpoint(Endpoint::list_database_snapshots(&self.id))
            .await?
            .into_result_t()
    }
}
