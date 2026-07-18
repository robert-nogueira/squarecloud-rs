use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));

    let object_id = std::env::args()
        .nth(1)
        .expect("Usage: blob_delete <object-id>");
    let deleted = client.blob().delete(&object_id).await.unwrap();
    println!("Deleted: {deleted}");
}
