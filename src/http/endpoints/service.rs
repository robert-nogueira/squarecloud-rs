use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn service_status() -> Endpoint {
        Self::builder("/service/status", Method::GET).build()
    }
}
