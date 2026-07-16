use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/databases/{database_id}/credentials/reset",
    domain: "DatabaseErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::DatabaseErrorCode>,
    }
}

impl Endpoint {
    pub(crate) fn redefine_database_credentials(
        database_id: &str,
    ) -> Endpoint {
        Self::builder(
            "/databases/{database_id}/credentials/reset",
            Method::POST,
        )
        .param("database_id", database_id)
        .build()
    }
}
