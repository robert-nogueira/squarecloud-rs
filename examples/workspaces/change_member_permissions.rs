use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let ws_id = std::env::args().nth(1).expect(
        "usage: cargo run --example change_member_permissions -- <workspace_id> <invite_code> <role>",
    );
    let invite_code = std::env::args().nth(2).expect(
        "usage: cargo run --example change_member_permissions -- <workspace_id> <invite_code> <role>",
    );
    let role = std::env::args().nth(3).expect(
        "usage: cargo run --example change_member_permissions -- <workspace_id> <invite_code> <role>",
    );
    let client = ApiClient::new();
    client
        .workspace(&ws_id)
        .change_member_permissions(&invite_code, &role)
        .await
        .unwrap();
}
