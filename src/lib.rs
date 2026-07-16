#![warn(missing_docs)]
//! Async Rust client for the [SquareCloud](https://squarecloud.app) API.
//!
//! # Overview
//!
//! This crate provides typed, async access to every endpoint exposed by the
//! SquareCloud platform: deploying and managing applications, provisioning
//! databases, organising workspaces, and inspecting account information.
//!
//! The main entry point is [`Client`]. It reads the `API_TOKEN`
//! environment variable (or a `.env` file) on first use, so no explicit
//! configuration struct is needed.
//!
//! # Quick start
//!
//! ```no_run
//! use squarecloud::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!
//!     // Fetch your account information.
//!     let me = client.me().await?;
//!     println!("Logged in as {} ({})", me.user.name, me.user.email);
//!
//!     // Inspect a running application. Resource factories borrow the
//!     // client and clone it internally (cheap: the connection pool is
//!     // shared), so the client stays usable afterwards.
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
//! | [`Client`] | Root entry point; construct with [`Client::new`]. |
//! | [`resources`] | Resource handles returned by the factory methods on `Client`. |
//! | [`types`] | Plain data structs deserialised from API responses. |
//! | [`ApiError`] / [`errors`] | Errors returned by every API call; one error-code enum per domain. |
//! | [`CommitError`] | Error type specific to [`resources::AppResource::commit`]. |
//!
//! # Environment variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `API_TOKEN` | Your SquareCloud API key. |
//!
//! Read at first use via [`dotenvy`](https://docs.rs/dotenvy), so a `.env`
//! file in the working directory is supported automatically.

mod http;
/// Resource handles returned by the factory methods on [`Client`].
pub mod resources;
mod settings;
/// Plain data structs deserialised from API responses.
pub mod types;

pub(crate) use http::endpoints::Endpoint;
#[cfg(feature = "test-utils")]
pub use http::endpoints::EndpointSpec;
pub use http::errors;
pub use http::errors::{ApiError, CommitError, ErrorCode, ServiceErrorCode};
pub use http::http_client::Client;
pub use types::{CredentialType, DatabaseType, RealtimeEvent, SnapshotScope};

#[cfg(test)]
mod tests {
    // use super::*;
}
