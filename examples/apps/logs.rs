use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let logs = client.app(&app_id).logs().await.unwrap();
    println!("{logs}");
}
