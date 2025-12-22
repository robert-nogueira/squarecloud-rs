use serde::{Deserialize, Serialize};

use super::plan::Plan;

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
    id: String,
    name: String,
    email: String,
    plan: Plan,
    applications: Vec<AppFromUser>,
}
