use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let bytes = std::fs::read("app.zip").unwrap();
    client.app("application_id").commit(bytes).await.unwrap();
}
