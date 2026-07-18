use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let workspace = client
        .create_workspace("my-workspace".to_string())
        .await
        .unwrap();
    println!("{workspace:#?}");
}
