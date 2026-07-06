use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let record = client.app("application_id").dns_record().await.unwrap();
    println!("{record:#?}");
}
