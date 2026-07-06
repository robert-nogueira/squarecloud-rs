use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .app("application_id")
        .set_custom_domain("myapp.example.com")
        .await
        .unwrap();
}
