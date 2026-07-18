use squarecloud::Client;
use wiremock::MockServer;

mod account;
mod app;
mod blob;
mod client;
mod database;
mod workspace;

pub async fn mock_client() -> (Client, MockServer) {
    let server = MockServer::start().await;
    let mut client = Client::new("mock-token");
    client.base_url = server.uri();
    client.blob_base_url = server.uri();
    (client, server)
}
