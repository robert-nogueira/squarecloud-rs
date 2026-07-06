use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .workspace("workspace_id")
        .invite_member("invite_code", "member")
        .await
        .unwrap();
}
