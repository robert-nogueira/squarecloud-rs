use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let workspaces = client.all_workspaces().await.unwrap();
    println!("{workspaces:#?}");
}
