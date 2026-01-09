use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn workspace_add_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::POST).build()
    }

    pub(crate) fn workspace_remove_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::DELETE).build()
    }
}
