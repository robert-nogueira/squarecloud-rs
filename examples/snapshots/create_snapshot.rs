use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let snapshot = client.app(&app_id).create_snapshot().await.unwrap();
    println!("url: {}", snapshot.url);
}
