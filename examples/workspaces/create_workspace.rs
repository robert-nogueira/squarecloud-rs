use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let workspace = client
        .create_workspace("my-workspace".to_string())
        .await
        .unwrap();
    println!("{workspace:#?}");
}
