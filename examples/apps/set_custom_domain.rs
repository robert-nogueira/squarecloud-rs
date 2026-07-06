use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let app_id = std::env::args().nth(1).expect(
        "usage: cargo run --example set_custom_domain -- <app_id> <domain>",
    );
    let domain = std::env::args().nth(2).expect(
        "usage: cargo run --example set_custom_domain -- <app_id> <domain>",
    );
    let client = ApiClient::new();
    client
        .app(&app_id)
        .set_custom_domain(&domain)
        .await
        .unwrap();
}
