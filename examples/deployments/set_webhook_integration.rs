use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let app_id = std::env::args().nth(1).expect(
        "usage: cargo run --example set_webhook_integration -- <app_id>",
    );
    let client = ApiClient::new();
    let access_token = std::env::var("GITHUB_TOKEN").unwrap();
    let webhook_url = client
        .app(&app_id)
        .set_webhook_integration(access_token)
        .await
        .unwrap();
    println!("{webhook_url}");
}
