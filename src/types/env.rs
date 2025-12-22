use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Env {
    key: String,
    value: String,
}
