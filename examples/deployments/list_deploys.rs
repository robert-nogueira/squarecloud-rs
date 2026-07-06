use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let deploys = client.app("application_id").list_deploys().await.unwrap();
    println!("{deploys:#?}");
}
