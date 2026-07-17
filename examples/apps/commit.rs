use squarecloud::Client;

#[tokio::main]
async fn main() {
    let app_id = std::env::args().nth(1).expect(
        "usage: cargo run --example commit -- <app_id> <zip_path> [dest_path]",
    );
    let zip_path = std::env::args().nth(2).expect(
        "usage: cargo run --example commit -- <app_id> <zip_path> [dest_path]",
    );
    let dest_path = std::env::args().nth(3);

    let client = Client::new();
    let bytes = std::fs::read(zip_path).unwrap();
    let app = client.app(&app_id);

    match dest_path {
        Some(dest) => app.commit_to(bytes, Some(&dest)).await.unwrap(),
        None => app.commit(bytes).await.unwrap(),
    };
}
