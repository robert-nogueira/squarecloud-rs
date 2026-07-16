use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{database::DatabaseSummary, plan::Plan};

/// Condensed application metadata as returned inside the `me` response.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSummary {
    /// The application's unique identifier.
    pub id: String,
    /// The application's display name.
    pub name: String,
    /// A short description of the application, if set.
    pub desc: Option<String>,
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
    /// The UTC timestamp when the application was created.
    pub created_at: DateTime<Utc>,
}

/// The authenticated user's profile fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// The account's unique identifier.
    pub id: String,
    /// The account holder's display name.
    pub name: String,
    /// The account holder's email address.
    pub email: String,
    /// The active subscription plan and its resource limits.
    pub plan: Plan,
    /// The UTC timestamp when the account was created.
    pub created_at: DateTime<Utc>,
}

/// Full account information returned by `me`.
///
/// Returned by [`Client::me`](crate::Client::me).
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    /// The authenticated user's profile.
    pub user: UserInfo,
    /// All applications owned by this account.
    pub applications: Vec<AppSummary>,
    /// All managed databases owned by this account.
    pub databases: Vec<DatabaseSummary>,
}
