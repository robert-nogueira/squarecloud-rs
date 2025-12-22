use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Database {
    id: String,
    name: String,
    memory: u32,
    cpu: u8,
    #[serde(rename = "type")]
    db_type: String,
    password: String,
    certificate: String,
    connection_url: String,
}
