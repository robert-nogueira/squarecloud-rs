use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub memory: u32,
    pub cpu: u8,
    #[serde(rename = "type")]
    pub db_type: String,
    pub password: String,
    pub certificate: String,
    pub connection_url: String,
}
