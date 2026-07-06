use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let code = client
        .workspace("workspace_id")
        .get_invite_code()
        .await
        .unwrap();
    println!("invite code: {code}");
}
