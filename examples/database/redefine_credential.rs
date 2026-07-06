use squarecloud_rs::{ApiClient, CredentialType};

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let db = client.database("database_id");

    let new_password = db
        .redefine_credential(CredentialType::Password)
        .await
        .unwrap();
    println!("new password: {new_password}");

    let new_cert = db
        .redefine_credential(CredentialType::Certificate)
        .await
        .unwrap();
    println!("new certificate: {new_cert}");
}
