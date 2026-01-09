use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn read_app_file(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files/content", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn list_app_files(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn put_app_file(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::PUT)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn move_app_file(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::PATCH)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn delete_app_file(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::DELETE)
            .param("app_id", app_id)
            .build()
    }
}
