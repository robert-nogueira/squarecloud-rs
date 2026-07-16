use reqwest::{Client as ReqwestClient, Method, RequestBuilder};
use serde_json::Value;

/// A fully-resolved API request descriptor.
///
/// `Endpoint` holds the HTTP method, the complete URL path (with all
/// parameters substituted), and an optional JSON body. It is produced by
/// `EndpointBuilder::build` and consumed by
/// [`Client::request_endpoint`](crate::Client) or
/// [`Endpoint::request_builder`].
///
/// This type is a lower-level building block of this crate. End users should
/// not need to construct `Endpoint` values directly; prefer the methods on
/// [`Client`](crate::Client) and the resource handles instead.
#[derive(Clone)]
pub struct Endpoint {
    pub path: String,
    pub method: Method,
    pub json_body: Option<Value>,
}

/// A builder for constructing an [`Endpoint`].
///
/// Obtain an instance through [`Endpoint::builder`]. Call the fluent setter
/// methods to populate path parameters, query parameters, and the request
/// body, then call `build` to produce an
/// [`Endpoint`].
///
/// This type is a lower-level building block of this crate.
pub struct EndpointBuilder {
    pub path_template: String,
    pub method: Method,
    pub params: Vec<(String, String)>,
    pub queries: Vec<(String, String)>,
    pub json_body: Option<Value>,
}

impl EndpointBuilder {
    pub(crate) fn new(path_template: &str, method: Method) -> Self {
        Self {
            path_template: path_template.to_string(),
            method,
            params: vec![],
            queries: vec![],
            json_body: None,
        }
    }

    /// Finalises the builder and returns an [`Endpoint`] with the path
    /// template resolved.
    pub(crate) fn build(self) -> Endpoint {
        let mut path = self.path_template.clone();
        for (k, v) in &self.params {
            path = path.replace(&format!("{{{}}}", k), v);
        }
        if !self.queries.is_empty() {
            path.push('?');
            let qs = self
                .queries
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            path.push_str(&qs);
        }
        Endpoint {
            method: self.method,
            json_body: self.json_body,
            path,
        }
    }

    /// Sets the JSON body to send with the request.
    pub(crate) fn json(mut self, body: Value) -> Self {
        self.json_body = Some(body);
        self
    }

    /// Appends a query parameter (`name=value`) to the URL.
    pub(crate) fn query(mut self, name: &str, value: &str) -> Self {
        self.queries.push((name.to_string(), value.to_string()));
        self
    }

    /// Adds a path parameter substitution, replacing `{name}` in the path
    /// template with `value`.
    pub(crate) fn param(mut self, name: &str, value: &str) -> Self {
        self.params.push((name.to_string(), value.to_string()));
        self
    }
}

/// A (method, path-template) pair used by contract tests to verify that an
/// implemented endpoint exists in the live OpenAPI spec.
///
/// Only available with the `test-utils` feature.
#[cfg(feature = "test-utils")]
pub struct EndpointSpec {
    pub method: &'static str,
    pub path: &'static str,
}

#[cfg(feature = "test-utils")]
inventory::collect!(EndpointSpec);

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use super::Endpoint;

    #[test]
    fn param_substitutes_placeholder() {
        let ep = Endpoint::builder("/apps/{app_id}/logs", Method::GET)
            .param("app_id", "abc123")
            .build();
        assert_eq!(ep.path, "/apps/abc123/logs");
    }

    #[test]
    fn multiple_params_all_substituted() {
        let ep =
            Endpoint::builder("/apps/{app_id}/files/{file_id}", Method::GET)
                .param("app_id", "app-1")
                .param("file_id", "index.js")
                .build();
        assert_eq!(ep.path, "/apps/app-1/files/index.js");
    }

    #[test]
    fn query_appended_after_path() {
        let ep = Endpoint::builder("/apps/{app_id}/errors", Method::GET)
            .param("app_id", "app-1")
            .query("include_4xx", "true")
            .build();
        assert_eq!(ep.path, "/apps/app-1/errors?include_4xx=true");
    }

    #[test]
    fn multiple_queries_joined_with_ampersand() {
        let ep = Endpoint::builder("/resource", Method::GET)
            .query("page", "1")
            .query("limit", "20")
            .build();
        assert_eq!(ep.path, "/resource?page=1&limit=20");
    }

    #[test]
    fn no_params_no_queries_path_unchanged() {
        let ep = Endpoint::builder("/apps/status", Method::GET).build();
        assert_eq!(ep.path, "/apps/status");
    }
}

impl Endpoint {
    /// Creates an `EndpointBuilder` for the given URL path template and HTTP
    /// method.
    ///
    /// Path parameters are marked with curly-brace placeholders such as
    /// `"{app_id}"` and are substituted by calling `EndpointBuilder::param`
    /// before `EndpointBuilder::build`.
    pub fn builder(path_template: &str, method: Method) -> EndpointBuilder {
        EndpointBuilder::new(path_template, method)
    }

    /// Converts this endpoint into a `reqwest` [`RequestBuilder`], attaching
    /// the stored JSON body if one was set.
    ///
    /// The returned builder can be further customised (e.g. with
    /// `.multipart()`) before calling `.build()` or `.send()`.
    pub fn request_builder(
        &self,
        http_client: &ReqwestClient,
        base_url: &str,
    ) -> RequestBuilder {
        let url = format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            self.path.trim_start_matches('/')
        );
        let mut request = http_client.request(self.method.clone(), url);
        if let Some(body) = &self.json_body {
            request = request.json(&body);
        }
        request
    }
}
