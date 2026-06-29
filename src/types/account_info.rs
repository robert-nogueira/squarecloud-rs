use super::plan::Plan;
use serde::{Deserialize, Serialize};

/// Metadata about a single application as seen from the account owner's view.
#[derive(Serialize, Deserialize)]
pub struct AppFromUser {
    /// The application's display name.
    pub name: String,
    /// The application's unique identifier.
    pub id: String,
    /// A short description of the application.
    pub desc: String,
    /// RAM allocation in megabytes.
    pub ram: u32,
    /// The programming language or runtime the application uses.
    pub lang: String,
    /// The primary domain assigned by SquareCloud, if any.
    pub domain: Option<String>,
    /// A custom domain configured by the owner, if any.
    pub custom: Option<String>,
    /// The data-centre cluster the application is hosted on.
    pub cluster: String,
}

/// Account information for the authenticated user.
///
/// Returned by [`ApiClient::me`](crate::ApiClient::me).
#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    /// The account's unique identifier.
    pub id: String,
    /// The account holder's display name.
    pub name: String,
    /// The account holder's email address.
    pub email: String,
    /// The active subscription plan and its resource limits.
    pub plan: Plan,
    /// All applications owned by this account.
    pub applications: Vec<AppFromUser>,
}
