use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let ws_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <workspace_id>");
    client.workspace(&ws_id).leave().await.unwrap();
}
