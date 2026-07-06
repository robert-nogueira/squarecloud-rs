use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .workspace("workspace_id")
        .remove_member("member_id")
        .await
        .unwrap();
}
