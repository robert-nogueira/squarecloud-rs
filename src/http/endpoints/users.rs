use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get", path: "/users/me" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get", path: "/users/snapshots" } }

impl Endpoint {
    pub(crate) fn me() -> Endpoint {
        Self::builder("/users/me", Method::GET).build()
    }

    pub(crate) fn list_all_snapshots(
        scope: Option<crate::types::SnapshotScope>,
    ) -> Endpoint {
        match scope {
            Some(s) => Self::builder(
                &format!("/users/snapshots?scope={}", s.as_str()),
                Method::GET,
            )
            .build(),
            None => Self::builder("/users/snapshots", Method::GET).build(),
        }
    }
}
