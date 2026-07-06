use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let bytes = std::fs::read("app.zip").unwrap();
    let app = client.upload_app(bytes).await.unwrap();
    println!("{app:#?}");
}
