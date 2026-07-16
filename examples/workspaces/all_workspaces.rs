use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let workspaces = client.all_workspaces().await.unwrap();
    println!("{workspaces:#?}");
}
