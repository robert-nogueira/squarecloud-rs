use reqwest::Method;
use serde_json::json;

use crate::types::UploadOptions;

use super::Endpoint;

impl Endpoint {
    pub(crate) fn blob_upload(name: &str, opts: &UploadOptions) -> Self {
        let mut b =
            Self::builder("/objects", Method::POST).query("name", name);
        if let Some(p) = &opts.prefix {
            b = b.query("prefix", p);
        }
        if let Some(e) = opts.expire {
            b = b.query("expire", &e.to_string());
        }
        if let Some(h) = opts.security_hash {
            b = b.query("security_hash", &h.to_string());
        }
        if let Some(d) = opts.auto_download {
            b = b.query("auto_download", &d.to_string());
        }
        b.build()
    }

    pub(crate) fn blob_list(
        prefix: Option<&str>,
        continuation_token: Option<&str>,
    ) -> Self {
        let mut b = Self::builder("/objects", Method::GET);
        if let Some(p) = prefix {
            b = b.query("prefix", p);
        }
        if let Some(t) = continuation_token {
            b = b.query("continuationToken", t);
        }
        b.build()
    }

    pub(crate) fn blob_delete(object: &str) -> Self {
        Self::builder("/objects", Method::DELETE)
            .json(json!({ "object": object }))
            .build()
    }

    pub(crate) fn blob_stats() -> Self {
        Self::builder("/account/stats", Method::GET).build()
    }
}
