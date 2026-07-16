use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let db_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <db_id>");
    let status = client.database(&db_id).status().await.unwrap();
    println!("{status:#?}");
}
