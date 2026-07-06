use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let status = client.service_status().await.unwrap();
    println!("{status:#?}");
}
