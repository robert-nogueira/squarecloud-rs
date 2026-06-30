//! Contract tests: verify that every endpoint this crate implements exists in
//! the live SquareCloud OpenAPI spec at the expected HTTP method and path.
//!
//! These tests fetch the spec at runtime so they catch upstream removals or
//! renames without requiring the fixture to be manually updated. They do not
//! check request/response schemas — only that the (method, path) pair is
//! present in the spec.

use std::collections::HashMap;

use serde_json::Value;

mod endpoints;

/// Collapse all `{param}` placeholders to `{p}` so paths with different
/// parameter names still compare equal (e.g. `{appId}` vs `{app_id}`).
fn normalize(path: &str) -> String {
    let mut out = String::new();
    let mut in_param = false;
    for ch in path.chars() {
        match ch {
            '{' => {
                in_param = true;
                out.push_str("{p}");
            }
            '}' => in_param = false,
            _ if in_param => {}
            c => out.push(c),
        }
    }
    out
}

/// Download the OpenAPI spec and return its `paths` object as a map.
///
/// Cloudflare blocks the default reqwest TLS fingerprint with 403, so we
/// identify as `curl` — a known-good UA that Cloudflare passes through.
pub async fn fetch_spec() -> HashMap<String, Value> {
    let client = reqwest::Client::builder()
        .user_agent("curl/8.0.0")
        .build()
        .expect("failed to build HTTP client");

    let spec: Value = client
        .get("https://api.squarecloud.app/v2/openapi.json")
        .send()
        .await
        .expect("failed to fetch OpenAPI spec")
        .json()
        .await
        .expect("failed to parse OpenAPI spec");

    spec["paths"]
        .as_object()
        .expect("OpenAPI spec missing 'paths'")
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

/// Check whether `(method, our_path)` exists in the spec.
///
/// Returns `None` when the pair is found, or a human-readable violation
/// string when the path or method is missing.
pub fn check(
    paths: &HashMap<String, Value>,
    method: &str,
    our_path: &str,
) -> Option<String> {
    let with_version = format!("/v2{our_path}");
    let our_normalized = normalize(&with_version);
    let spec_paths: HashMap<String, &str> = paths
        .keys()
        .map(|k| (normalize(k), k.as_str()))
        .collect();

    match spec_paths.get(&our_normalized) {
        None => Some(format!("path not in spec: {our_path}")),
        Some(&spec_path) => {
            if paths[spec_path].get(method).is_none() {
                let available = paths[spec_path]
                    .as_object()
                    .map(|o| {
                        o.keys().cloned().collect::<Vec<_>>().join(", ")
                    })
                    .unwrap_or_default();
                Some(format!(
                    "method not in spec: {method} {our_path} \
                     (spec has: {available})"
                ))
            } else {
                None
            }
        }
    }
}

#[test]
fn normalize_replaces_params() {
    assert_eq!(normalize("/v2/apps/{appId}"), "/v2/apps/{p}");
    assert_eq!(
        normalize("/v2/apps/{appId}/network/logs"),
        "/v2/apps/{p}/network/logs"
    );
    assert_eq!(normalize("/v2/apps/status"), "/v2/apps/status");
}
