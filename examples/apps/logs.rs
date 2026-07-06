use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let logs = client.app("application_id").logs().await.unwrap();
    println!("{logs}");
}
