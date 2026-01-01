use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn list_database_snapshots(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/snapshots",
            Method::GET,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn create_database_snapshot(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/snapshots",
            Method::POST,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn restore_database_snapshot(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/snapshots/restore",
            Method::POST,
            &[("database_id", database_id)],
        )
    }
}
