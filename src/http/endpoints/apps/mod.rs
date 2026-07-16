pub mod deploys;
pub mod envs;
pub mod files;
pub mod network;
pub mod snapshots;

use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps",
    domain: "UploadErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::UploadErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/status",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/domains",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/load-balancers",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "delete",
    path: "/apps/{app_id}",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/status",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/logs",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/metrics",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/realtime",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/start",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/restart",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/stop",
    domain: "AppErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::AppErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/commit",
    domain: "UploadErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::UploadErrorCode>,
    }
}

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

    pub(crate) fn app_domains() -> Endpoint {
        Self::builder("/apps/domains", Method::GET).build()
    }

    pub(crate) fn app_load_balancers() -> Endpoint {
        Self::builder("/apps/load-balancers", Method::GET).build()
    }

    pub(crate) fn app_metrics(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/metrics", Method::GET)
            .param("app_id", app_id)
            .build()
    }
}
