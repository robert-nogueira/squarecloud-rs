use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn me() -> Endpoint {
        Self::builder("/users/me", Method::GET).build()
    }

    pub(crate) fn list_all_snapshots() -> Endpoint {
        Self::builder("/users/snapshots", Method::GET).build()
    }
}
