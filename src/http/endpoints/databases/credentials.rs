use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn redefine_database_credentials(database_id: &str) -> Endpoint {
        Self::build(
            "/databases/{database_id}/credentials/reset",
            Method::POST,
            &[("database_id", database_id)],
        )
    }
}
