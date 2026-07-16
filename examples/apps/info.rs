use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let info = client.app(&app_id).info().await.unwrap();
    println!("{info:#?}");
}
