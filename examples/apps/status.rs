use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let status = client.app("application_id").status().await.unwrap();
    println!("{status:#?}");
}
