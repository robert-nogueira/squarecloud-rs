use squarecloud::Client;

#[tokio::main]
async fn main() {
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example commit -- <app_id> <zip_path>");
    let zip_path = std::env::args()
        .nth(2)
        .expect("usage: cargo run --example commit -- <app_id> <zip_path>");
    let client = Client::new();
    let bytes = std::fs::read(zip_path).unwrap();
    client.app(&app_id).commit(bytes).await.unwrap();
}
