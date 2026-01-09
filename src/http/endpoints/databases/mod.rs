pub mod credentials;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn create_database() -> Endpoint {
        Self::builder("/databases", Method::POST).build()
    }

    pub(crate) fn start_database(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/start", Method::POST)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn stop_database(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/stop", Method::POST)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn all_database_status() -> Endpoint {
        Self::builder("/databases/status", Method::GET).build()
    }

    pub(crate) fn database_status(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/status", Method::GET)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn database_metrics(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/metrics", Method::GET)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn edit_database(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}", Method::PATCH)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn delete_database(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}", Method::DELETE)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn get_database_certificate(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/certificate", Method::GET)
            .param("database_id", database_id)
            .build()
    }
}
