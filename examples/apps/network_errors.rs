use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let errors = client
        .app("application_id")
        .network_errors(false)
        .await
        .unwrap();
    println!("{errors:#?}");
}
