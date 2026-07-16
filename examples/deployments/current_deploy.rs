use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let deploy = client.app(&app_id).current_deploy().await.unwrap();
    println!("{deploy:#?}");
}
