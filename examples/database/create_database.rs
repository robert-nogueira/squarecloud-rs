use squarecloud::{Client, DatabaseType};

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let db = client
        .create_database(
            "my-db".to_string(),
            256,
            DatabaseType::Redis,
            "latest".to_string(),
        )
        .await
        .unwrap();
    println!("{db:#?}");
}
