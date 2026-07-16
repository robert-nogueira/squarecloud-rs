use squarecloud::Client;

#[tokio::main]
async fn main() {
    let ws_id = std::env::args().nth(1).expect(
        "usage: cargo run --example remove_member -- <workspace_id> <member_id>",
    );
    let member_id = std::env::args().nth(2).expect(
        "usage: cargo run --example remove_member -- <workspace_id> <member_id>",
    );
    let client = Client::new();
    client
        .workspace(&ws_id)
        .remove_member(&member_id)
        .await
        .unwrap();
}
