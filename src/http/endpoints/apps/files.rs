use std::collections::HashMap;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn read_app_file(app_id: &str, path: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files/content", Method::GET)
            .param("app_id", app_id)
            .query("path", path)
            .build()
    }

    pub(crate) fn list_app_files(app_id: &str, path: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::GET)
            .param("app_id", app_id)
            .query("path", path)
            .build()
    }

    pub(crate) fn put_app_file(
        app_id: &str,
        path: &str,
        content: &str,
    ) -> Endpoint {
        let mut json_body = HashMap::with_capacity(2);
        json_body.extend([("path", path), ("content", content)]);
        Self::builder("/apps/{app_id}/files", Method::PUT)
            .param("app_id", app_id)
            .json(serde_json::to_value(json_body).unwrap())
            .build()
    }

    pub(crate) fn move_app_file(
        app_id: &str,
        source_path: &str,
        destination_path: &str,
    ) -> Endpoint {
        let mut json_body = HashMap::with_capacity(2);
        json_body.extend([("path", source_path), ("to", destination_path)]);

        Self::builder("/apps/{app_id}/files", Method::PATCH)
            .param("app_id", app_id)
            .json(serde_json::to_value(json_body).unwrap())
            .build()
    }

    pub(crate) fn delete_app_file(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/files", Method::DELETE)
            .param("app_id", app_id)
            .build()
    }
}
