use serde::{Deserialize, Serialize};

use crate::http::errors::ErrorCode;

/// Error codes returned by the platform status endpoint
/// ([`Client::service_status`](crate::Client::service_status)).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ServiceStatusErrorCode {
    /// Unexpected internal failure. Try again later.
    InternalServerError,
    /// A code returned by the API that this client does not recognise.
    /// The inner [`ErrorCode`] preserves the raw wire string.
    #[serde(untagged)]
    Unknown(ErrorCode),
}

impl_service_error_code!(ServiceStatusErrorCode);
