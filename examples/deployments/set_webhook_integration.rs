use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let access_token = std::env::var("GITHUB_TOKEN").unwrap();
    let webhook_url = client
        .app("application_id")
        .set_webhook_integration(access_token)
        .await
        .unwrap();
    println!("{webhook_url}");
}
