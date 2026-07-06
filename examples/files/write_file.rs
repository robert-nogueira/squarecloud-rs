use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    client
        .app(&app_id)
        .file("/app/config.json")
        .write(r#"{"workers": 4}"#)
        .await
        .unwrap();
}
