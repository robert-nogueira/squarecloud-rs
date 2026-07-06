use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let analytics = client.app("application_id").analytics().await.unwrap();
    println!("{analytics:#?}");
}
