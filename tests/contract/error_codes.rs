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

/// Walks every path/method/status of the spec and collects the error
/// codes documented in `example` (single) and `examples` (named) blocks.
fn documented_error_codes(
    paths: &HashMap<String, Value>,
) -> Vec<DocumentedCode> {
    let mut found = Vec::new();
    for (path, item) in paths {
        for method in HTTP_METHODS {
            let responses = &item[method]["responses"];
            let Some(responses) = responses.as_object() else {
                continue;
            };
            for (status, response) in responses {
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
    let paths = crate::fetch_spec().await;

    let implemented: HashMap<(String, String), &'static EndpointSpec> =
        inventory::iter::<EndpointSpec>()
            .map(|spec| {
                (route_key(spec.method, &format!("/v2{}", spec.path)), spec)
            })
            .collect();

    let mut violations = BTreeSet::new();
    let mut unimplemented_routes = BTreeSet::new();

    for entry in documented_error_codes(&paths) {
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
