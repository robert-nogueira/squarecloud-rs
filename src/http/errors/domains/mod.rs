//! Domain-scoped error code enums, one file per domain.
//!
//! Each enum lists only the codes the corresponding group of routes is
//! known to return, plus an `Unknown` fallback that preserves any
//! unrecognised wire string as a raw
//! [`ErrorCode`](crate::http::errors::ErrorCode). See
//! [`ServiceErrorCode`](crate::http::errors::ServiceErrorCode).

/// Implements the sealed [`ServiceErrorCode`](crate::http::errors::ServiceErrorCode)
/// machinery plus the `enum == ErrorCode` comparison for a domain enum.
macro_rules! impl_service_error_code {
    ($ty:ty) => {
        impl crate::http::errors::sealed::Sealed for $ty {}
        impl crate::http::errors::ServiceErrorCode for $ty {
            fn is_unknown(&self) -> bool {
                matches!(self, Self::Unknown(_))
            }
        }
        impl PartialEq<crate::http::errors::ErrorCode> for $ty {
            fn eq(&self, other: &crate::http::errors::ErrorCode) -> bool {
                other == self
            }
        }
    };
}

mod account;
mod app;
mod blob;
mod database;
mod deploy;
mod env;
mod file;
mod member;
mod network;
mod service;
mod snapshot;
mod upload;
mod workspace;

pub use account::AccountErrorCode;
pub use app::AppErrorCode;
pub use blob::BlobErrorCode;
pub use database::DatabaseErrorCode;
pub use deploy::DeployErrorCode;
pub use env::EnvErrorCode;
pub use file::FileErrorCode;
pub use member::MemberErrorCode;
pub use network::NetworkErrorCode;
pub use service::ServiceStatusErrorCode;
pub use snapshot::SnapshotErrorCode;
pub use upload::UploadErrorCode;
pub use workspace::WorkspaceErrorCode;

#[cfg(test)]
mod tests {
    use crate::http::errors::{
        BlobErrorCode, EnvErrorCode, ErrorCode, ServiceErrorCode,
        SnapshotErrorCode,
    };

    #[test]
    fn from_wire_parses_known_code() {
        let got = SnapshotErrorCode::from_wire(
            "DAILY_SNAPSHOTS_LIMIT_REACHED".to_owned(),
        );
        assert_eq!(got, SnapshotErrorCode::DailySnapshotsLimitReached);
    }

    #[test]
    fn from_wire_falls_back_to_unknown() {
        let got = EnvErrorCode::from_wire("APPLICATION_STOPPING".to_owned());
        assert_eq!(got, EnvErrorCode::Unknown("APPLICATION_STOPPING".into()));
    }

    #[test]
    fn erase_returns_wire_string() {
        assert_eq!(
            BlobErrorCode::ObjectNotFound.erase(),
            ErrorCode::from("OBJECT_NOT_FOUND")
        );
    }

    #[test]
    fn erase_preserves_unknown_raw_string() {
        let code = BlobErrorCode::Unknown("APPLICATION_STOPPING".into());
        assert_eq!(code.erase(), ErrorCode::from("APPLICATION_STOPPING"));
    }

    #[test]
    fn domain_code_roundtrips_through_serde() {
        let json = serde_json::to_string(&EnvErrorCode::TooManyEnvVars)
            .expect("serializes");
        assert_eq!(json, r#""TOO_MANY_ENV_VARS""#);
        let back: EnvErrorCode =
            serde_json::from_str(&json).expect("deserializes");
        assert_eq!(back, EnvErrorCode::TooManyEnvVars);
    }

    #[test]
    fn domain_enum_compares_with_error_code_symmetrically() {
        let erased = ErrorCode::from("TOO_MANY_ENV_VARS");
        assert!(EnvErrorCode::TooManyEnvVars == erased);
        assert!(erased == EnvErrorCode::TooManyEnvVars);
    }
}
