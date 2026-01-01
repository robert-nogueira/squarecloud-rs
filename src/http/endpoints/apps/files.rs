use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn read_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files/content",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub fn list_app_files(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/files", Method::GET, &[("app_id", app_id)])
    }

    pub fn put_app_file(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/files", Method::PUT, &[("app_id", app_id)])
    }

    pub fn move_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files",
            Method::PATCH,
            &[("app_id", app_id)],
        )
    }

    pub fn delete_app_file(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/files",
            Method::DELETE,
            &[("app_id", app_id)],
        )
    }
}
