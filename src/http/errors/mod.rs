//! Error types returned by API operations.
//!
//! Every API method returns [`ApiError<C>`], where `C` is the
//! domain-scoped error code enum for that group of routes (one enum per
//! domain, all listed in this module). Codes a domain enum does not
//! recognise are preserved in its `Unknown` variant as a raw
//! [`ErrorCode`].

mod code;
mod domains;

pub use code::ErrorCode;
pub use domains::{
    AccountErrorCode, AppErrorCode, BlobErrorCode, DatabaseErrorCode,
    DeployErrorCode, EnvErrorCode, FileErrorCode, MemberErrorCode,
    NetworkErrorCode, SnapshotErrorCode, UploadErrorCode, WorkspaceErrorCode,
};

mod sealed {
    pub trait Sealed {}
}

/// Implemented by every domain-scoped error code enum.
///
/// This trait is sealed: it cannot be implemented outside this crate.
/// [`ErrorCode`] deliberately does **not** implement it — that exclusion
/// is what allows the crate to provide a blanket conversion from any
/// domain-scoped [`ApiError`] into `ApiError<ErrorCode>` without
/// overlapping the standard library's identity `From` impl. Treat the
/// absence of that impl as a semver commitment.
pub trait ServiceErrorCode:
    sealed::Sealed
    + std::fmt::Debug
    + Clone
    + PartialEq
    + Eq
    + serde::Serialize
    + serde::de::DeserializeOwned
{
    /// Parses a raw wire code (e.g. `"APP_NOT_FOUND"`) into this enum.
    ///
    /// Codes this enum does not recognise land in its `Unknown` variant
    /// with the raw string preserved.
    fn from_wire(code: String) -> Self {
        serde_json::from_value(serde_json::Value::String(code))
            .expect("error-code enums accept any string via Unknown fallback")
    }

    /// Returns this code's raw wire string as a type-erased [`ErrorCode`].
    fn erase(&self) -> ErrorCode {
        match serde_json::to_value(self)
            .expect("error codes serialize to JSON strings")
        {
            serde_json::Value::String(s) => ErrorCode(s),
            other => unreachable!("error code serialized as {other:?}"),
        }
    }
}

/// An error returned by any API operation.
///
/// Most methods on [`ApiClient`](crate::ApiClient) and on resource handles
/// return `Result<_, ApiError<C>>`, where `C` is the domain-scoped error
/// code enum for that group of routes (e.g. [`EnvErrorCode`] for
/// environment variable operations). Match on this enum to distinguish
/// between a transport-level failure and a structured error from the
/// SquareCloud API.
///
/// The default type parameter is the type-erased [`ErrorCode`], so code
/// that composes calls across domains can propagate everything as plain
/// `ApiError` via `?` (see the `From` conversion).
#[derive(Debug)]
pub enum ApiError<C = ErrorCode> {
    /// The HTTP request failed before the server could respond.
    Transport(reqwest::Error),
    /// The server responded with a structured API error code.
    Service {
        /// The machine-readable error code from the API response.
        code: C,
    },
}

/// An error that can occur during [`AppResource::commit`](crate::resources::AppResource::commit).
///
/// Committing a file requires building a multipart HTTP request, which may
/// fail for reasons beyond an API error. This enum wraps both cases.
#[derive(Debug)]
pub enum CommitError {
    /// An API-level or transport-level error occurred.
    Api(ApiError<UploadErrorCode>),
    /// An I/O error occurred while reading the bytes to be committed.
    Io(std::io::Error),
}

impl<C: std::fmt::Debug> std::fmt::Display for ApiError<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Transport(e) => write!(f, "transport error: {e}"),
            ApiError::Service { code } => write!(f, "service error: {code:?}"),
        }
    }
}

impl<C: std::fmt::Debug> std::error::Error for ApiError<C> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Transport(e) => Some(e),
            ApiError::Service { .. } => None,
        }
    }
}

impl<C> From<reqwest::Error> for ApiError<C> {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Transport(err)
    }
}

impl<C: ServiceErrorCode> From<ApiError<C>> for ApiError<ErrorCode> {
    fn from(err: ApiError<C>) -> Self {
        match err {
            ApiError::Transport(e) => ApiError::Transport(e),
            ApiError::Service { code } => {
                ApiError::Service { code: code.erase() }
            }
        }
    }
}

impl From<std::io::Error> for CommitError {
    fn from(err: std::io::Error) -> Self {
        CommitError::Io(err)
    }
}

impl From<reqwest::Error> for CommitError {
    fn from(err: reqwest::Error) -> Self {
        CommitError::Api(ApiError::from(err))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{
        ApiError, BlobErrorCode, CommitError, EnvErrorCode, ErrorCode,
    };

    #[tokio::test]
    async fn api_error_display_transport() {
        let reqwest_err = reqwest::get("http://0.0.0.0:1").await.unwrap_err();
        let err: ApiError = ApiError::Transport(reqwest_err);
        assert!(err.to_string().starts_with("transport error:"));
    }

    #[test]
    fn api_error_display_service() {
        let err: ApiError = ApiError::Service {
            code: ErrorCode::from("RATE_LIMIT"),
        };
        assert!(err.to_string().starts_with("service error:"));
    }

    #[tokio::test]
    async fn api_error_source_transport_is_some() {
        let reqwest_err = reqwest::get("http://0.0.0.0:1").await.unwrap_err();
        let err: ApiError = ApiError::Transport(reqwest_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn api_error_source_service_is_none() {
        let err: ApiError = ApiError::Service {
            code: ErrorCode::from("NOT_FOUND"),
        };
        assert!(err.source().is_none());
    }

    #[test]
    fn commit_error_from_io_error() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err = CommitError::from(io);
        assert!(matches!(err, CommitError::Io(_)));
    }

    #[test]
    fn question_mark_composes_across_domains() {
        fn blob_op() -> Result<(), ApiError<BlobErrorCode>> {
            Err(ApiError::Service {
                code: BlobErrorCode::ObjectNotFound,
            })
        }
        fn env_op() -> Result<(), ApiError<EnvErrorCode>> {
            Err(ApiError::Service {
                code: EnvErrorCode::Unknown("ENV_NAME_TOO_LONG".into()),
            })
        }
        fn composed(pick_blob: bool) -> Result<(), ApiError> {
            if pick_blob {
                blob_op()?
            } else {
                env_op()?
            }
            Ok(())
        }
        match composed(true) {
            Err(ApiError::Service { code }) => {
                assert!(code == BlobErrorCode::ObjectNotFound);
                assert!(code == "OBJECT_NOT_FOUND");
            }
            other => panic!("expected service error, got {other:?}"),
        }
        match composed(false) {
            Err(ApiError::Service { code }) => {
                assert!(code == "ENV_NAME_TOO_LONG");
            }
            other => panic!("expected service error, got {other:?}"),
        }
    }
}
