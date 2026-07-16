use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let db_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <db_id>");
    let cert = client.database(&db_id).certificate().await.unwrap();
    println!("{cert}");
}
