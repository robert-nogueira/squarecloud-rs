use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServiceStatus {
    status: String,
    message: String,
}
