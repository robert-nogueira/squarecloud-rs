use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let envs = client.app("application_id").list_envs().await.unwrap();
    println!("{envs:#?}");
}
