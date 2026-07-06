use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let workspaces = client.all_workspaces().await.unwrap();
    println!("{workspaces:#?}");
}
