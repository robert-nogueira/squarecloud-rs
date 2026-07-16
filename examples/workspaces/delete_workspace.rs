use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let ws_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <workspace_id>");
    client.workspace(&ws_id).delete().await.unwrap();
}
