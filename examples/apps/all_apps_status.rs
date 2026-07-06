use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let statuses = client.all_apps_status().await.unwrap();
    println!("{statuses:#?}");
}
