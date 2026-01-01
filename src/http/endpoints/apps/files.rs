use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn read_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files/content",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn list_app_files(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/files", Method::GET, &[("app_id", app_id)])
    }

    pub(crate) fn put_app_file(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/files", Method::PUT, &[("app_id", app_id)])
    }

    pub(crate) fn move_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files",
            Method::PATCH,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn delete_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files",
            Method::DELETE,
            &[("app_id", app_id)],
        )
    }
}
