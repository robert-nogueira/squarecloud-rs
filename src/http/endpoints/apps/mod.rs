pub mod deploys;
pub mod envs;
pub mod files;
pub mod network;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn upload_app() -> Endpoint {
        Self::build("/apps", Method::POST, &[])
    }

    pub fn all_apps_status() -> Endpoint {
        Self::build("/apps/status", Method::GET, &[])
    }

    pub fn app_info(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}", Method::GET, &[("app_id", app_id)])
    }

    pub fn app_status(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/status", Method::GET, &[("app_id", app_id)])
    }

    pub fn app_logs(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/logs", Method::GET, &[("app_id", app_id)])
    }

    pub fn sse_realtime_app_logs(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/realtime",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub fn app_start(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/start", Method::POST, &[("app_id", app_id)])
    }

    pub fn app_restart(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/restart",
            Method::POST,
            &[("app_id", app_id)],
        )
    }

    pub fn app_stop(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/stop", Method::POST, &[("app_id", app_id)])
    }

    pub fn app_commit(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/commit", Method::POST, &[("app_id", app_id)])
    }

    pub fn app_delete(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}", Method::DELETE, &[("app_id", app_id)])
    }
}
