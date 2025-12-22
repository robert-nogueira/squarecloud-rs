use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn me() -> Endpoint {
        Self::build("/users/me", Method::GET, &[])
    }

    pub fn list_all_snapshots() -> Endpoint {
        Self::build("/users/snapshots", Method::GET, &[])
    }
}
