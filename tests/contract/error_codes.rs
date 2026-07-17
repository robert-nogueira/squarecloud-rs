//! Verifies that every error code documented in the OpenAPI spec's
//! response examples is catalogued in the error-code enum of the route
//! that returns it.
//!
//! Each implemented route registers its domain enum through
//! [`EndpointSpec`]; codes documented for routes the crate does not
//! implement are reported but do not fail the test.

use std::collections::{BTreeSet, HashMap};

use serde_json::Value;
use squarecloud::EndpointSpec;

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];

/// An error code found in the spec, with the route and status it was
/// documented under.
struct DocumentedCode {
    method: String,
    path: String,
    status: String,
    code: String,
}

/// Key shared by spec routes and [`EndpointSpec`] registrations: the
/// lowercase method plus the normalized path. Crate paths lack the
/// `/v2` prefix, so callers on that side must prepend it.
fn route_key(method: &str, versioned_path: &str) -> (String, String) {
    (method.to_owned(), crate::normalize(versioned_path))
}

/// Returns the error code of one response example, or `None` when the
/// example is not an error envelope (`"status": "error"`).
fn error_code_of(example: &Value) -> Option<String> {
    if example["status"] != "error" {
        return None;
    }
    example["code"].as_str().map(str::to_owned)
}

/// Resolves a response that is a `$ref` into the shared component it
/// points to (`#/components/responses/Name`); returns the response
/// itself when it is inline.
fn resolve<'a>(spec: &'a Value, response: &'a Value) -> &'a Value {
    match response["$ref"].as_str() {
        Some(reference) => {
            let name =
                reference.rsplit('/').next().expect("$ref is never empty");
            &spec["components"]["responses"][name]
        }
        None => response,
    }
}

/// Walks every path/method/status of the spec and collects the error
/// codes documented in `example` (single) and `examples` (named) blocks,
/// resolving shared `$ref` responses to their components.
fn documented_error_codes(spec: &Value) -> Vec<DocumentedCode> {
    let mut found = Vec::new();
    let Some(paths) = spec["paths"].as_object() else {
        return found;
    };
    for (path, item) in paths {
        for method in HTTP_METHODS {
            let responses = &item[method]["responses"];
            let Some(responses) = responses.as_object() else {
                continue;
            };
            for (status, response) in responses {
                let response = resolve(spec, response);
                let json = &response["content"]["application/json"];
                let single = json.get("example").into_iter();
                let named = json["examples"]
                    .as_object()
                    .into_iter()
                    .flat_map(|m| m.values())
                    .map(|entry| &entry["value"]);
                for code in single.chain(named).filter_map(error_code_of) {
                    found.push(DocumentedCode {
                        method: method.to_string(),
                        path: path.clone(),
                        status: status.clone(),
                        code,
                    });
                }
            }
        }
    }
    found
}

#[tokio::test]
async fn all_spec_error_codes_are_catalogued() {
    let spec = crate::fetch_full_spec().await;

    let implemented: HashMap<(String, String), &'static EndpointSpec> =
        inventory::iter::<EndpointSpec>()
            .map(|spec| {
                (route_key(spec.method, &format!("/v2{}", spec.path)), spec)
            })
            .collect();

    let mut violations = BTreeSet::new();
    let mut unimplemented_routes = BTreeSet::new();

    for entry in documented_error_codes(&spec) {
        let route = format!("{} {}", entry.method.to_uppercase(), entry.path);
        match implemented.get(&route_key(&entry.method, &entry.path)) {
            None => {
                unimplemented_routes.insert(route);
            }
            Some(spec) if !(spec.known_code)(&entry.code) => {
                violations.insert(format!(
                    "{route} ({}): {} not catalogued in {}",
                    entry.status, entry.code, spec.domain
                ));
            }
            Some(_) => {}
        }
    }

    if !unimplemented_routes.is_empty() {
        eprintln!(
            "note: spec documents error codes for {} route(s) the crate \
             does not implement:\n  {}",
            unimplemented_routes.len(),
            unimplemented_routes
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join("\n  ")
        );
    }

    assert!(
        violations.is_empty(),
        "{} uncatalogued error code(s):\n  {}",
        violations.len(),
        violations.iter().cloned().collect::<Vec<_>>().join("\n  ")
    );
}

/// `POST`/`DELETE /apps/{appId}/deploy/github-app` are the only two
/// documented main-API routes the crate does not implement. Both are
/// gated behind a session token (JWT): the API rejects a plain API key
/// with `ACCESS_DENIED`, and there is no documented, scriptable way to
/// obtain a session token outside an interactive dashboard login (the
/// JS and Go SDK references carry the same warning verbatim, and the Go
/// SDK exposes it as a separate `rest.WithToken` credential, not
/// something derived from an API key).
///
/// Implementing these methods would add surface area that no consumer
/// of this crate can ever call successfully with its only supported
/// auth mode. This test exists purely as a marker: it shows up ignored
/// in `cargo test -- --list --ignored`, so the omission reads as a
/// deliberate decision instead of a gap, and its message is the record
/// of why. It intentionally never runs.
#[test]
#[ignore = "github-app deploy link/unlink require a session token (JWT); \
            plain API keys are rejected with ACCESS_DENIED and there is \
            no documented way to obtain a session token, so the crate \
            does not implement these two routes"]
fn github_app_deploy_routes_are_intentionally_unimplemented() {}
