use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let me = client.me().await.unwrap();
    println!("{me:#?}");
}
