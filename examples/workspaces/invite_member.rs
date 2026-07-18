use squarecloud::Client;

#[tokio::main]
async fn main() {
    let ws_id = std::env::args().nth(1).expect(
        "usage: cargo run --example invite_member -- <workspace_id> <invite_code> <member_id>",
    );
    let invite_code = std::env::args().nth(2).expect(
        "usage: cargo run --example invite_member -- <workspace_id> <invite_code> <member_id>",
    );
    let member_id = std::env::args().nth(3).expect(
        "usage: cargo run --example invite_member -- <workspace_id> <invite_code> <member_id>",
    );
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    client
        .workspace(&ws_id)
        .invite_member(&invite_code, &member_id)
        .await
        .unwrap();
}
