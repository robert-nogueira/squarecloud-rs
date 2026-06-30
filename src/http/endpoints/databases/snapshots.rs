use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",  path: "/databases/{database_id}/snapshots" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post", path: "/databases/{database_id}/snapshots" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post", path: "/databases/{database_id}/snapshots/restore" } }

impl Endpoint {
    pub(crate) fn list_database_snapshots(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/snapshots", Method::GET)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn create_database_snapshot(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}/snapshots", Method::POST)
            .param("database_id", database_id)
            .build()
    }

    pub(crate) fn restore_database_snapshot(database_id: &str) -> Endpoint {
        Self::builder(
            "/databases/{database_id}/snapshots/restore",
            Method::POST,
        )
        .param("database_id", database_id)
        .build()
    }
}
