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
        AccountErrorCode, AppErrorCode, BlobErrorCode, DatabaseErrorCode,
        DeployErrorCode, EnvErrorCode, ErrorCode, FileErrorCode,
        MemberErrorCode, NetworkErrorCode, ServiceErrorCode,
        SnapshotErrorCode, UploadErrorCode, WorkspaceErrorCode,
    };

    fn rate_limited<C: ServiceErrorCode>() -> C {
        C::from_wire("RATE_LIMITED".to_owned())
    }

    #[test]
    fn rate_limited_is_catalogued_in_every_domain() {
        assert_eq!(
            rate_limited::<AccountErrorCode>(),
            AccountErrorCode::RateLimited
        );
        assert_eq!(rate_limited::<AppErrorCode>(), AppErrorCode::RateLimited);
        assert_eq!(
            rate_limited::<BlobErrorCode>(),
            BlobErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<DatabaseErrorCode>(),
            DatabaseErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<DeployErrorCode>(),
            DeployErrorCode::RateLimited
        );
        assert_eq!(rate_limited::<EnvErrorCode>(), EnvErrorCode::RateLimited);
        assert_eq!(
            rate_limited::<FileErrorCode>(),
            FileErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<MemberErrorCode>(),
            MemberErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<NetworkErrorCode>(),
            NetworkErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<SnapshotErrorCode>(),
            SnapshotErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<UploadErrorCode>(),
            UploadErrorCode::RateLimited
        );
        assert_eq!(
            rate_limited::<WorkspaceErrorCode>(),
            WorkspaceErrorCode::RateLimited
        );
    }

    #[test]
    fn retryable_codes_documented_per_method_are_classified() {
        // start, restart, stop and delete
        for code in [
            AppErrorCode::RestoreInProgress,
            AppErrorCode::KeepCalm,
            AppErrorCode::RateLimited,
            AppErrorCode::DatabaseUnavailable,
        ] {
            assert!(code.is_retryable(), "{code:?}");
        }
        // upload_app (the first four also apply to commit)
        for code in [
            UploadErrorCode::UploadBusy,
            UploadErrorCode::KeepCalm,
            UploadErrorCode::RateLimited,
            UploadErrorCode::DatabaseUnavailable,
            UploadErrorCode::ClusterMaintenanceTryLater,
        ] {
            assert!(code.is_retryable(), "{code:?}");
        }
        // upsert_envs, overwrite_envs and delete_envs
        for code in [
            EnvErrorCode::EnvBusy,
            EnvErrorCode::RateLimited,
            EnvErrorCode::DatabaseUnavailable,
        ] {
            assert!(code.is_retryable(), "{code:?}");
        }
    }

    #[test]
    fn state_and_failure_codes_documented_per_method_stay_definitive() {
        for code in [
            AppErrorCode::ActionFailed,
            AppErrorCode::ContainerAlreadyStarted,
            AppErrorCode::ContainerNotFound,
            AppErrorCode::DeleteFailed,
        ] {
            assert!(!code.is_retryable(), "{code:?}");
        }
        assert!(!UploadErrorCode::UploadFailed.is_retryable());
    }

    #[test]
    fn domain_enums_classify_through_the_trait() {
        assert!(DatabaseErrorCode::RateLimited.is_rate_limited());
        assert!(UploadErrorCode::UploadBusy.is_retryable());
        assert!(!UploadErrorCode::UploadBusy.is_rate_limited());
        assert!(!AppErrorCode::AppNotFound.is_retryable());
    }

    #[test]
    fn unknown_variant_is_classified_by_its_wire_string() {
        let code = EnvErrorCode::Unknown("KEEP_CALM".into());
        assert!(code.is_rate_limited());
        assert!(code.is_retryable());
    }

    #[test]
    fn retired_rate_limit_codes_fall_back_to_unknown() {
        for code in ["RATE_LIMIT", "RATE_LIMIT_EXCEEDED"] {
            assert!(AppErrorCode::from_wire(code.to_owned()).is_unknown());
            assert!(NetworkErrorCode::from_wire(code.to_owned()).is_unknown());
            assert!(BlobErrorCode::from_wire(code.to_owned()).is_unknown());
        }
    }

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
