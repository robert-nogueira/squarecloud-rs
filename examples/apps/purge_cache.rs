use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client.app("application_id").purge_cache().await.unwrap();
}
