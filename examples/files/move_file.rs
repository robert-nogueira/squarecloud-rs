use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .app("application_id")
        .file("/app/old_name.py")
        .move_to("/app/new_name.py")
        .await
        .unwrap();
}
