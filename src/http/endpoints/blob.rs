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
        if let Some(expire) = opts.expire_query() {
            b = b.query("expire", &expire);
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

#[cfg(test)]
mod tests {
    use super::Endpoint;
    use crate::types::{BlobExpiry, UploadOptions};

    #[test]
    fn blob_list_with_neither_option_has_no_query_string() {
        let ep = Endpoint::blob_list(None, None);
        assert_eq!(ep.path, "/objects");
    }

    #[test]
    fn blob_list_with_prefix_only_omits_continuation_token() {
        let ep = Endpoint::blob_list(Some("images"), None);
        assert_eq!(ep.path, "/objects?prefix=images");
    }

    #[test]
    fn blob_list_with_token_only_omits_prefix() {
        let ep = Endpoint::blob_list(None, Some("abc123"));
        assert_eq!(ep.path, "/objects?continuationToken=abc123");
    }

    #[test]
    fn blob_list_with_both_options_joins_them() {
        let ep = Endpoint::blob_list(Some("images"), Some("abc123"));
        assert_eq!(ep.path, "/objects?prefix=images&continuationToken=abc123");
    }

    #[test]
    fn blob_upload_with_no_options_only_sends_name() {
        let ep = Endpoint::blob_upload("photo", &UploadOptions::default());
        assert_eq!(ep.path, "/objects?name=photo");
    }

    #[test]
    fn blob_upload_with_all_options_sends_every_query_param() {
        let opts = UploadOptions {
            prefix: Some("images".to_string()),
            expiry: Some(BlobExpiry::Hours(6)),
            security_hash: Some(true),
            auto_download: Some(false),
            ..Default::default()
        };
        let ep = Endpoint::blob_upload("photo", &opts);
        assert_eq!(
            ep.path,
            "/objects?name=photo&prefix=images&expire=6h\
             &security_hash=true&auto_download=false"
        );
    }

    #[test]
    fn blob_upload_sends_expiry_in_days() {
        let opts = UploadOptions {
            expiry: Some(BlobExpiry::Days(30)),
            ..Default::default()
        };
        let ep = Endpoint::blob_upload("photo", &opts);
        assert_eq!(ep.path, "/objects?name=photo&expire=30d");
    }

    #[test]
    #[allow(deprecated)]
    fn blob_upload_still_sends_deprecated_expire_in_days() {
        let opts = UploadOptions {
            expire: Some(30),
            ..Default::default()
        };
        let ep = Endpoint::blob_upload("photo", &opts);
        assert_eq!(ep.path, "/objects?name=photo&expire=30d");
    }

    #[test]
    #[allow(deprecated)]
    fn blob_upload_prefers_expiry_over_deprecated_expire() {
        let opts = UploadOptions {
            expire: Some(30),
            expiry: Some(BlobExpiry::Hours(12)),
            ..Default::default()
        };
        let ep = Endpoint::blob_upload("photo", &opts);
        assert_eq!(ep.path, "/objects?name=photo&expire=12h");
    }

    #[test]
    fn blob_delete_sends_object_name_as_json_body() {
        let ep = Endpoint::blob_delete("images/photo.png");
        assert_eq!(
            ep.json_body,
            Some(serde_json::json!({ "object": "images/photo.png" }))
        );
    }
}
