use serde::{Deserialize, Serialize};

use super::ServiceErrorCode;

/// A raw, type-erased error code: the wire string exactly as the API sent
/// it.
///
/// `ErrorCode` appears in two places:
///
/// - As the payload of every domain enum's `Unknown` variant, holding a
///   code that enum does not (yet) recognise.
/// - As the default type parameter of [`ApiError`](super::ApiError), so
///   code that composes calls across domains can propagate everything as
///   plain `ApiError` via `?`.
///
/// It can be compared directly against any domain-scoped code enum or
/// against a string:
///
/// ```
/// use squarecloud::{ErrorCode, errors::BlobErrorCode};
///
/// let code = ErrorCode::from("OBJECT_NOT_FOUND");
/// assert!(code == BlobErrorCode::ObjectNotFound);
/// assert!(code == "OBJECT_NOT_FOUND");
/// ```
///
/// `ErrorCode` deliberately does **not** implement
/// [`ServiceErrorCode`]; see the trait docs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorCode(pub(crate) String);

impl ErrorCode {
    /// Returns the raw wire string (e.g. `"APP_NOT_FOUND"`).
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` for the short-term rate-limit codes, `RATE_LIMITED`
    /// and `KEEP_CALM`. Wait before sending the request again.
    ///
    /// Quota codes such as `DAILY_SNAPSHOTS_LIMIT_REACHED` are not
    /// included: they only clear when the quota resets.
    pub fn is_rate_limited(&self) -> bool {
        matches!(self.as_str(), "RATE_LIMITED" | "KEEP_CALM")
    }

    /// Returns `true` when the same request can succeed if sent again
    /// later, unchanged: short-term rate limits, and the codes the API
    /// uses for a temporarily busy or unavailable service.
    ///
    /// Generic failures such as `INTERNAL_SERVER_ERROR` or the `*_FAILED`
    /// codes are not included, since a retry may repeat a side effect.
    ///
    /// `RATE_LIMITED` is also what an IP gets after repeated invalid API
    /// keys; retrying with the same key keeps it blocked.
    pub fn is_retryable(&self) -> bool {
        self.is_rate_limited()
            || matches!(
                self.as_str(),
                "DATABASE_UNAVAILABLE"
                    | "UPLOAD_BUSY"
                    | "ANALYTICS_BUSY"
                    | "ENV_BUSY"
                    | "RESTORE_IN_PROGRESS"
                    | "SNAPSHOT_IN_PROGRESS"
                    | "CLUSTER_MAINTENANCE_TRY_LATER"
                    | "CLUSTER_UNAVAILABLE"
                    | "CLUSTER_TIMEOUT"
                    | "VALIDATION_TIMEOUT"
                    | "TOO_MANY_CONCURRENT_UPLOADS"
                    | "PRIVATE_STORAGE_UNAVAILABLE"
                    | "PUBLIC_STORAGE_UNAVAILABLE"
            )
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ErrorCode {
    fn from(code: String) -> Self {
        ErrorCode(code)
    }
}

impl From<&str> for ErrorCode {
    fn from(code: &str) -> Self {
        ErrorCode(code.to_owned())
    }
}

impl PartialEq<str> for ErrorCode {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for ErrorCode {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl<C: ServiceErrorCode> PartialEq<C> for ErrorCode {
    fn eq(&self, other: &C) -> bool {
        self.0 == other.erase().0
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;
    use crate::http::errors::{
        AppErrorCode, BlobErrorCode, DatabaseErrorCode, EnvErrorCode,
        NetworkErrorCode, ServiceErrorCode, SnapshotErrorCode,
        UploadErrorCode,
    };

    #[test]
    fn compares_with_domain_enums_and_strings() {
        let code = ErrorCode::from("OBJECT_NOT_FOUND");
        assert!(code == BlobErrorCode::ObjectNotFound);
        assert!(code != BlobErrorCode::InvalidObject);
        assert!(code == "OBJECT_NOT_FOUND");
        assert!(code != EnvErrorCode::TooManyEnvVars);
    }

    #[test]
    fn rate_limit_codes_are_rate_limited_and_retryable() {
        for code in [
            AppErrorCode::RateLimited.erase(),
            AppErrorCode::KeepCalm.erase(),
        ] {
            assert!(code.is_rate_limited(), "{code}");
            assert!(code.is_retryable(), "{code}");
        }
    }

    #[test]
    fn busy_and_unavailable_codes_are_retryable_only() {
        // Built from the catalogue, so a misspelled entry in the list
        // fails here instead of passing silently.
        let catalogued = [
            AppErrorCode::DatabaseUnavailable.erase(),
            UploadErrorCode::UploadBusy.erase(),
            NetworkErrorCode::AnalyticsBusy.erase(),
            EnvErrorCode::EnvBusy.erase(),
            AppErrorCode::RestoreInProgress.erase(),
            SnapshotErrorCode::SnapshotInProgress.erase(),
            DatabaseErrorCode::ClusterMaintenanceTryLater.erase(),
            BlobErrorCode::TooManyConcurrentUploads.erase(),
            BlobErrorCode::PrivateStorageUnavailable.erase(),
            BlobErrorCode::PublicStorageUnavailable.erase(),
        ];
        // Not catalogued by any domain enum yet.
        let uncatalogued = [
            "CLUSTER_UNAVAILABLE",
            "CLUSTER_TIMEOUT",
            "VALIDATION_TIMEOUT",
        ]
        .map(ErrorCode::from);
        for code in catalogued.iter().chain(&uncatalogued) {
            assert!(code.is_retryable(), "{code}");
            assert!(!code.is_rate_limited(), "{code}");
        }
    }

    #[test]
    fn permanent_and_quota_codes_are_not_retryable() {
        for code in [
            AppErrorCode::AppNotFound.erase(),
            AppErrorCode::InternalServerError.erase(),
            AppErrorCode::DeleteFailed.erase(),
            SnapshotErrorCode::DailySnapshotsLimitReached.erase(),
        ] {
            assert!(!code.is_retryable(), "{code}");
            assert!(!code.is_rate_limited(), "{code}");
        }
    }

    #[test]
    fn compares_with_unknown_variants_by_wire_string() {
        let code = ErrorCode::from("APPLICATION_STOPPING");
        assert!(code == EnvErrorCode::Unknown("APPLICATION_STOPPING".into()));
    }

    #[test]
    fn display_prints_raw_code() {
        assert_eq!(ErrorCode::from("KEEP_CALM").to_string(), "KEEP_CALM");
    }

    #[test]
    fn as_str_returns_raw_wire_string() {
        assert_eq!(ErrorCode::from("APP_NOT_FOUND").as_str(), "APP_NOT_FOUND");
    }

    /// `from_wire` (the internal parsing path) always hands `ServiceErrorCode`
    /// an owned `String` decoded from JSON, never a `&str`; if `From<String>`
    /// diverged from `From<&str>` (e.g. one trimmed whitespace and the other
    /// did not), a code would compare differently depending on which
    /// constructor happened to run.
    #[test]
    fn from_owned_string_matches_from_str_slice() {
        let owned = ErrorCode::from(String::from("RATE_LIMIT"));
        let borrowed = ErrorCode::from("RATE_LIMIT");
        assert_eq!(owned, borrowed);
    }

    /// Exercises `PartialEq<str>` specifically (not `PartialEq<&str>`):
    /// comparing against a dereferenced owned `String`, the way code
    /// holding a `String` it read from elsewhere (not a `&'static str`
    /// literal) would compare it without an explicit `.as_str()`.
    #[test]
    fn compares_against_a_dereferenced_owned_string() {
        let code = ErrorCode::from("OBJECT_NOT_FOUND");
        let owned = String::from("OBJECT_NOT_FOUND");
        assert!(code == *owned);
        let different = String::from("INVALID_OBJECT");
        assert!(code != *different);
    }

    #[test]
    fn serde_is_transparent() {
        let code: ErrorCode =
            serde_json::from_str(r#""SOME_CODE""#).expect("deserializes");
        assert_eq!(code, "SOME_CODE");
        assert_eq!(
            serde_json::to_string(&code).expect("serializes"),
            r#""SOME_CODE""#
        );
    }
}
