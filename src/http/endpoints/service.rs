use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get", path: "/service/status" } }

impl Endpoint {
    pub(crate) fn service_status() -> Endpoint {
        Self::builder("/service/status", Method::GET).build()
    }
}
