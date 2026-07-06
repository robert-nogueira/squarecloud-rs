use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let content = client
        .app("application_id")
        .file("/app/main.py")
        .read("/app/main.py")
        .await
        .unwrap();
    println!("{}", String::from_utf8_lossy(&content.data));
}
