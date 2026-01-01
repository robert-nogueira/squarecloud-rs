use super::plan::Plan;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppFromUser {
    pub name: String,
    pub id: String,
    pub desc: String,
    pub ram: u32,
    pub lang: String,
    pub domain: Option<String>,
    pub custom: Option<String>,
    pub cluster: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub plan: Plan,
    pub applications: Vec<AppFromUser>,
}
