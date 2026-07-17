use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/service/status",
    domain: "ServiceStatusErrorCode",
    known_code:
        crate::errors::code_is_known::<crate::errors::ServiceStatusErrorCode>,
    }
}

impl Endpoint {
    pub(crate) fn service_status() -> Endpoint {
        Self::builder("/service/status", Method::GET).build()
    }
}
