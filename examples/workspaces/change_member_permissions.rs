use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .workspace("workspace_id")
        .change_member_permissions("invite_code", "admin")
        .await
        .unwrap();
}
