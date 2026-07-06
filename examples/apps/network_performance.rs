use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let perf = client
        .app("application_id")
        .network_performance()
        .await
        .unwrap();
    println!("{perf:#?}");
}
