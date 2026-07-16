use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let statuses = client.all_database_status().await.unwrap();
    println!("{statuses:#?}");
}
