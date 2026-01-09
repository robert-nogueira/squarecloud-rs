use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value;

#[derive(Clone)]
pub struct EndpointTemplate {
    pub path: &'static str,
    pub method: Method,
}

#[derive(Clone)]
pub struct Endpoint {
    pub path: String,
    pub method: Method,
    pub json_body: Option<Value>,
}

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

    pub(crate) fn build(self) -> Endpoint {
        let mut path = String::new();
        for (k, v) in &self.params {
            path = self.path_template.replace(&format!("{{{}}}", k), v);
        }
        if !self.queries.is_empty() {
            path.push('&');
            for (k, v) in &self.queries {
                path.push_str(&format!("{k}={v}"));
            }
        }
        Endpoint {
            method: self.method,
            json_body: self.json_body,
            path,
        }
    }

    pub(crate) fn json(mut self, body: Value) -> Self {
        self.json_body = Some(body);
        self
    }

    pub(crate) fn query(mut self, name: &str, value: &str) -> Self {
        self.queries.push((name.to_string(), value.to_string()));
        self
    }

    pub(crate) fn param(mut self, name: &str, value: &str) -> Self {
        self.params.push((name.to_string(), value.to_string()));
        self
    }
}

impl Endpoint {
    pub fn builder(path_template: &str, method: Method) -> EndpointBuilder {
        EndpointBuilder::new(path_template, method)
    }
    pub fn request_builder(&self, http_client: &Client) -> RequestBuilder {
        let mut request =
            http_client.request(self.method.clone(), self.path.clone());
        if let Some(body) = &self.json_body {
            request = request.json(&body);
        }
        request
    }
}
