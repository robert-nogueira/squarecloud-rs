use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let ws_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <workspace_id>");
    let code = client.workspace(&ws_id).get_invite_code().await.unwrap();
    println!("invite code: {code}");
}
