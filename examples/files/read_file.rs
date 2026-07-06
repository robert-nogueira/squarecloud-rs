use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let content = client
        .app(&app_id)
        .file("/app/main.py")
        .read("/app/main.py")
        .await
        .unwrap();
    println!("{}", String::from_utf8_lossy(&content.data));
}
