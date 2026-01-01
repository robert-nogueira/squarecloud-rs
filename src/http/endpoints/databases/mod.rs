pub mod credentials;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn create_database() -> Endpoint {
        Self::build("/databases", Method::POST, &[])
    }

    pub(crate) fn start_database(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/start",
            Method::POST,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn stop_database(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/stop",
            Method::POST,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn all_database_status() -> Endpoint {
        Self::build("/databases/status", Method::GET, &[])
    }

    pub(crate) fn database_status(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/status",
            Method::GET,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn database_metrics(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/metrics",
            Method::GET,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn edit_database(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}",
            Method::PATCH,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn delete_database(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}",
            Method::DELETE,
            &[("database_id", database_id)],
        )
    }

    pub(crate) fn get_database_certificate(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/certificate",
            Method::GET,
            &[("database_id", database_id)],
        )
    }
}
