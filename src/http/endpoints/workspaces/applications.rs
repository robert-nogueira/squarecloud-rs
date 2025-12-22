use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn workspace_add_app() -> Endpoint {
        Self::build("/workspaces/applications", Method::POST, &[])
    }

    pub fn workspace_remove_app() -> Endpoint {
        Self::build("/workspaces/applications", Method::DELETE, &[])
    }
}
