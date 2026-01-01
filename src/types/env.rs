use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Env {
    pub key: String,
    pub value: String,
}
