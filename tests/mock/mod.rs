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
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
}

pub async fn mock_client() -> (ApiClient, MockServer) {
    setup();
    let server = MockServer::start().await;
    let mut client = ApiClient::new();
    client.base_url = server.uri();
    (client, server)
}
