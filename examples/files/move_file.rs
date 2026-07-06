use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    client
        .app(&app_id)
        .file("/app/old_name.py")
        .move_to("/app/new_name.py")
        .await
        .unwrap();
}
