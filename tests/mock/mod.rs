use std::sync::Once;

use squarecloud::ApiClient;
use wiremock::MockServer;

mod account;
mod app;
mod client;
mod database;
mod workspace;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        // Try .env.test for local dev; fall back to a dummy token so mock
        // tests run in CI without credentials.
        let _ = dotenvy::from_filename(".env.test");
        if std::env::var("API_TOKEN").is_err() {
            // Safe: Once guarantees this runs before any test thread reads
            // the var, so there are no concurrent readers.
            unsafe { std::env::set_var("API_TOKEN", "mock-token") }
        }
    });
}

pub async fn mock_client() -> (ApiClient, MockServer) {
    setup();
    let server = MockServer::start().await;
    let mut client = ApiClient::new();
    client.base_url = server.uri();
    (client, server)
}
