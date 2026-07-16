use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let status = client.service_status().await.unwrap();
    println!("{status:#?}");
}
