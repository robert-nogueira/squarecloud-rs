use reqwest::{Client, Method, RequestBuilder};

#[derive(Clone)]
pub struct EndpointTemplate {
    pub path: &'static str,
    pub method: Method,
}

#[derive(Clone)]
pub struct Endpoint {
    pub path: String,
    pub method: Method,
}

impl Endpoint {
    pub(crate) fn build(
        path: &'static str,
        method: Method,
        params: &[(&str, &str)],
    ) -> Endpoint {
        let mut final_path = path.to_string();
        for (k, v) in params {
            final_path = final_path.replace(&format!("{{{}}}", k), v);
        }
        Endpoint {
            method,
            path: final_path,
        }
    }

    pub fn request_builder(&self, http_client: &Client) -> RequestBuilder {
        http_client.request(self.method.clone(), self.path.clone())
    }
}
