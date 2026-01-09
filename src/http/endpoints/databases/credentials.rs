use super::Endpoint;
use reqwest::Method;

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
