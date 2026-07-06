use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let snapshots = client.app(&app_id).list_snapshots().await.unwrap();
    println!("{snapshots:#?}");
}
