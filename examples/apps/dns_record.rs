use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let record = client.app(&app_id).dns_record().await.unwrap();
    println!("{record:#?}");
}
