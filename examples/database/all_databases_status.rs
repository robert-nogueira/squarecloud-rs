use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let statuses = client.all_database_status().await.unwrap();
    println!("{statuses:#?}");
}
