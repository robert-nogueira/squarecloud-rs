pub mod deploys;
pub mod envs;
pub mod files;
pub mod network;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn upload_app() -> Endpoint {
        Self::builder("/apps", Method::POST).build()
    }

    pub(crate) fn all_apps_status() -> Endpoint {
        Self::builder("/apps/status", Method::GET).build()
    }

    pub(crate) fn app_info(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_status(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/status", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_logs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/logs", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn sse_realtime_app_logs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/realtime", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_start(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/start", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_restart(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/restart", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_stop(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/stop", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_commit(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/commit", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_delete(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}", Method::DELETE)
            .param("app_id", app_id)
            .build()
    }
}
