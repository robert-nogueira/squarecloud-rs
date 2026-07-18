use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let records = client.app(&app_id).dns_records().await.unwrap();
    if records.is_empty() {
        println!(
            "no DNS records yet; the custom hostname is not registered on the edge"
        );
    }
    for record in records {
        println!("{record:#?}");
    }
}
