pub mod credentials;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/databases",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/databases/status",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/databases/{database_id}",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "delete",
    path: "/databases/{database_id}",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "patch",
    path: "/databases/{database_id}",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/databases/{database_id}/status",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/databases/{database_id}/metrics",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/databases/{database_id}/start",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/databases/{database_id}/stop",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/databases/{database_id}/credentials/certificate",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

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
        Self::builder(
            "/databases/{database_id}/credentials/certificate",
            Method::GET,
        )
        .param("database_id", database_id)
        .build()
    }

    pub(crate) fn database_info(database_id: &str) -> Endpoint {
        Self::builder("/databases/{database_id}", Method::GET)
            .param("database_id", database_id)
            .build()
    }
}
