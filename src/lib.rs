//! Async Rust client for the [SquareCloud](https://squarecloud.app) API.
//!
//! # Overview
//!
//! This crate provides typed, async access to every endpoint exposed by the
//! SquareCloud platform: deploying and managing applications, provisioning
//! databases, organising workspaces, and inspecting account information.
//!
//! The main entry point is [`ApiClient`]. It reads the `API_TOKEN` and
//! `BASE_URL` environment variables (or a `.env` file) on first use, so no
//! explicit configuration struct is needed.
//!
//! # Quick start
//!
//! ```no_run
//! use squarecloud_rs::ApiClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ApiClient::new();
//!
//!     // Fetch your account information.
//!     let me = client.me().await?;
//!     println!("Logged in as {} ({})", me.user.name, me.user.email);
//!
//!     // Inspect a running application.
//!     // Note: `app()` consumes the client, so call account-level
//!     // methods first.
//!     let status = client.app("your-app-id").status().await?;
//!     println!("CPU: {}  RAM: {}", status.cpu, status.ram);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Crate layout
//!
//! | Item | Purpose |
//! |------|---------|
//! | [`ApiClient`] | Root entry point; construct with [`ApiClient::new`]. |
//! | [`resources`] | Resource handles returned by the factory methods on `ApiClient`. |
//! | [`types`] | Plain data structs deserialised from API responses. |
//! | [`ApiError`] / [`ApiErrorCode`] | Errors returned by every API call. |
//! | [`CommitError`] | Error type specific to [`resources::AppResource::commit`]. |
//!
//! # Environment variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `API_TOKEN` | Your SquareCloud API key. |
//! | `BASE_URL` | Base URL of the SquareCloud API (e.g. `https://api.squarecloud.app/v2`). |
//!
//! Both variables are read at first use via
//! [`dotenvy`](https://docs.rs/dotenvy), so a `.env` file in the working
//! directory is supported automatically.

mod http;
pub mod resources;
mod settings;
pub mod types;

pub use http::endpoints::Endpoint;
#[cfg(feature = "test-utils")]
pub use http::endpoints::EndpointSpec;
pub use http::errors::{ApiError, ApiErrorCode, CommitError};
pub use http::http_client::ApiClient;

#[cfg(test)]
mod tests {
    // use super::*;
}
