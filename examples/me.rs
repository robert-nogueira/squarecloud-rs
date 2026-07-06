use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let me = client.me().await.unwrap();
    println!("{me:#?}");
}
