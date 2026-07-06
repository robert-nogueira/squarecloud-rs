use squarecloud_rs::ApiClient;

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
    let client = ApiClient::new();
    client
        .workspace(&ws_id)
        .invite_member(&invite_code, &member_id)
        .await
        .unwrap();
}
