use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn me() -> Endpoint {
        Self::builder("/users/me", Method::GET).build()
    }

    pub(crate) fn list_all_snapshots(scope: Option<&str>) -> Endpoint {
        match scope {
            Some(s) => Self::builder(
                &format!("/users/snapshots?scope={s}"),
                Method::GET,
            )
            .build(),
            None => Self::builder("/users/snapshots", Method::GET).build(),
        }
    }
}
