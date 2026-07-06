use serde::{Deserialize, Serialize};

/// Machine-readable error code returned by the SquareCloud API.
///
/// The wire format uses `SCREAMING_SNAKE_CASE` (e.g. `"FEW_MEMORY"`).
/// Match on this enum after receiving an [`ApiError::Api`] to act on the
/// specific cause of the failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiErrorCode {
    /// The account's memory quota is insufficient for the requested
    /// allocation.
    FewMemory,
    /// The requested memory value is not a valid allocation size.
    BadMemory,
    /// The uploaded archive does not contain a `squarecloud.app`
    /// configuration file.
    MissingConfig,
    /// A dependency listed in the configuration is not a supported
    /// package.
    InvalidDependency,
    /// No entry-point file was detected in the uploaded archive.
    MissingMain,
    /// The entry-point file specified in the configuration does not
    /// exist in the archive.
    InvalidMain,
    /// The `display_name` field contains disallowed characters.
    InvalidDisplayName,
    /// The `display_name` field is absent from the configuration.
    MissingDisplayName,
    /// The memory value is outside the range permitted by the plan.
    InvalidMemory,
    /// The `memory` field is absent from the configuration.
    MissingMemory,
    /// The language or runtime version is not supported.
    InvalidVersion,
    /// The `version` field is absent from the configuration.
    MissingVersion,
    /// The API token in the `Authorization` header is invalid or revoked.
    InvalidAccessToken,
    /// A field value did not pass server-side regex validation.
    RegexValidation,
    /// The `start` command in the configuration is not a valid invocation.
    InvalidStart,
    /// The requested subdomain is malformed or already taken.
    InvalidSubdomain,
    /// The request was rejected by the rate limiter.
    RateLimit,
    /// The requested resource was not found.
    NotFound,
    /// An application with this name already exists in the account.
    AppNotFound,
    /// The uploaded file is malformed or has an unsupported format.
    InvalidFile,
    /// Upload rate limit: too many uploads in a short window. Try again later.
    KeepCalm,
    /// Start was rejected because the container is already running.
    ContainerAlreadyStarted,
    /// The requested time range has no data or is invalid.
    InvalidTimeRange,
    /// The application has no custom domain configured.
    NoCustomDomain,
    /// The snapshot version ID is not valid or does not exist.
    InvalidVersionId,
    /// The database type value is not recognised by the API.
    DatabaseTypeInvalid,
    /// The database version string is not valid for the chosen engine.
    DatabaseVersionInvalid,
    /// A snapshot restore is already in progress for this resource.
    RestoreInProgress,
    /// The daily snapshot creation limit for this resource has been reached.
    DailySnapshotsLimitReached,
    /// The `scope` query parameter value is not recognised by the API.
    InvalidScope,
    /// A code returned by the API that this client does not recognise.
    /// The inner string contains the raw value from the API response.
    Unknown(String),
}

impl ApiErrorCode {
    fn as_str(&self) -> &str {
        match self {
            Self::FewMemory => "FEW_MEMORY",
            Self::BadMemory => "BAD_MEMORY",
            Self::MissingConfig => "MISSING_CONFIG",
            Self::InvalidDependency => "INVALID_DEPENDENCY",
            Self::MissingMain => "MISSING_MAIN",
            Self::InvalidMain => "INVALID_MAIN",
            Self::InvalidDisplayName => "INVALID_DISPLAY_NAME",
            Self::MissingDisplayName => "MISSING_DISPLAY_NAME",
            Self::InvalidMemory => "INVALID_MEMORY",
            Self::MissingMemory => "MISSING_MEMORY",
            Self::InvalidVersion => "INVALID_VERSION",
            Self::MissingVersion => "MISSING_VERSION",
            Self::InvalidAccessToken => "INVALID_ACCESS_TOKEN",
            Self::RegexValidation => "REGEX_VALIDATION",
            Self::InvalidStart => "INVALID_START",
            Self::InvalidSubdomain => "INVALID_SUBDOMAIN",
            Self::RateLimit => "RATE_LIMIT",
            Self::NotFound => "NOT_FOUND",
            Self::AppNotFound => "APP_NOT_FOUND",
            Self::InvalidFile => "INVALID_FILE",
            Self::KeepCalm => "KEEP_CALM",
            Self::ContainerAlreadyStarted => "CONTAINER_ALREADY_STARTED",
            Self::InvalidTimeRange => "INVALID_TIME_RANGE",
            Self::NoCustomDomain => "NO_CUSTOM_DOMAIN",
            Self::InvalidVersionId => "INVALID_VERSION_ID",
            Self::DatabaseTypeInvalid => "DATABASE_TYPE_INVALID",
            Self::DatabaseVersionInvalid => "DATABASE_VERSION_INVALID",
            Self::RestoreInProgress => "RESTORE_IN_PROGRESS",
            Self::DailySnapshotsLimitReached => {
                "DAILY_SNAPSHOTS_LIMIT_REACHED"
            }
            Self::InvalidScope => "INVALID_SCOPE",
            Self::Unknown(s) => s.as_str(),
        }
    }
}

impl Serialize for ApiErrorCode {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ApiErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(
        d: D,
    ) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(match s.as_str() {
            "FEW_MEMORY" => Self::FewMemory,
            "BAD_MEMORY" => Self::BadMemory,
            "MISSING_CONFIG" => Self::MissingConfig,
            "INVALID_DEPENDENCY" => Self::InvalidDependency,
            "MISSING_MAIN" => Self::MissingMain,
            "INVALID_MAIN" => Self::InvalidMain,
            "INVALID_DISPLAY_NAME" => Self::InvalidDisplayName,
            "MISSING_DISPLAY_NAME" => Self::MissingDisplayName,
            "INVALID_MEMORY" => Self::InvalidMemory,
            "MISSING_MEMORY" => Self::MissingMemory,
            "INVALID_VERSION" => Self::InvalidVersion,
            "MISSING_VERSION" => Self::MissingVersion,
            "INVALID_ACCESS_TOKEN" => Self::InvalidAccessToken,
            "REGEX_VALIDATION" => Self::RegexValidation,
            "INVALID_START" => Self::InvalidStart,
            "INVALID_SUBDOMAIN" => Self::InvalidSubdomain,
            "RATE_LIMIT" => Self::RateLimit,
            "NOT_FOUND" => Self::NotFound,
            "APP_NOT_FOUND" => Self::AppNotFound,
            "INVALID_FILE" => Self::InvalidFile,
            "KEEP_CALM" => Self::KeepCalm,
            "CONTAINER_ALREADY_STARTED" => Self::ContainerAlreadyStarted,
            "INVALID_TIME_RANGE" => Self::InvalidTimeRange,
            "NO_CUSTOM_DOMAIN" => Self::NoCustomDomain,
            "INVALID_VERSION_ID" => Self::InvalidVersionId,
            "DATABASE_TYPE_INVALID" => Self::DatabaseTypeInvalid,
            "DATABASE_VERSION_INVALID" => Self::DatabaseVersionInvalid,
            "RESTORE_IN_PROGRESS" => Self::RestoreInProgress,
            "DAILY_SNAPSHOTS_LIMIT_REACHED" => {
                Self::DailySnapshotsLimitReached
            }
            "INVALID_SCOPE" => Self::InvalidScope,
            _ => Self::Unknown(s),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{ApiError, ApiErrorCode, CommitError};

    const ALL_CODES: &[(&str, ApiErrorCode)] = &[
        ("FEW_MEMORY", ApiErrorCode::FewMemory),
        ("BAD_MEMORY", ApiErrorCode::BadMemory),
        ("MISSING_CONFIG", ApiErrorCode::MissingConfig),
        ("INVALID_DEPENDENCY", ApiErrorCode::InvalidDependency),
        ("MISSING_MAIN", ApiErrorCode::MissingMain),
        ("INVALID_MAIN", ApiErrorCode::InvalidMain),
        ("INVALID_DISPLAY_NAME", ApiErrorCode::InvalidDisplayName),
        ("MISSING_DISPLAY_NAME", ApiErrorCode::MissingDisplayName),
        ("INVALID_MEMORY", ApiErrorCode::InvalidMemory),
        ("MISSING_MEMORY", ApiErrorCode::MissingMemory),
        ("INVALID_VERSION", ApiErrorCode::InvalidVersion),
        ("MISSING_VERSION", ApiErrorCode::MissingVersion),
        ("INVALID_ACCESS_TOKEN", ApiErrorCode::InvalidAccessToken),
        ("REGEX_VALIDATION", ApiErrorCode::RegexValidation),
        ("INVALID_START", ApiErrorCode::InvalidStart),
        ("INVALID_SUBDOMAIN", ApiErrorCode::InvalidSubdomain),
        ("RATE_LIMIT", ApiErrorCode::RateLimit),
        ("NOT_FOUND", ApiErrorCode::NotFound),
        ("APP_NOT_FOUND", ApiErrorCode::AppNotFound),
        ("INVALID_FILE", ApiErrorCode::InvalidFile),
        ("KEEP_CALM", ApiErrorCode::KeepCalm),
        (
            "CONTAINER_ALREADY_STARTED",
            ApiErrorCode::ContainerAlreadyStarted,
        ),
        ("INVALID_TIME_RANGE", ApiErrorCode::InvalidTimeRange),
        ("NO_CUSTOM_DOMAIN", ApiErrorCode::NoCustomDomain),
        ("INVALID_VERSION_ID", ApiErrorCode::InvalidVersionId),
        ("DATABASE_TYPE_INVALID", ApiErrorCode::DatabaseTypeInvalid),
        (
            "DATABASE_VERSION_INVALID",
            ApiErrorCode::DatabaseVersionInvalid,
        ),
        ("RESTORE_IN_PROGRESS", ApiErrorCode::RestoreInProgress),
        (
            "DAILY_SNAPSHOTS_LIMIT_REACHED",
            ApiErrorCode::DailySnapshotsLimitReached,
        ),
        ("INVALID_SCOPE", ApiErrorCode::InvalidScope),
    ];

    #[test]
    fn all_known_codes_roundtrip() {
        for (wire, variant) in ALL_CODES {
            let json = format!(r#""{wire}""#);
            let got: ApiErrorCode = serde_json::from_str(&json).unwrap();
            assert_eq!(&got, variant, "deserialize failed for {wire}");
            let serialized = serde_json::to_string(variant).unwrap();
            assert_eq!(serialized, json, "serialize failed for {wire}");
        }
    }

    #[test]
    fn unknown_code_captures_raw_string() {
        let got: ApiErrorCode =
            serde_json::from_str(r#""APPLICATION_STOPPING""#).unwrap();
        assert_eq!(got, ApiErrorCode::Unknown("APPLICATION_STOPPING".into()));
        assert_eq!(
            serde_json::to_string(&got).unwrap(),
            r#""APPLICATION_STOPPING""#
        );
    }

    #[tokio::test]
    async fn api_error_display_transport() {
        let reqwest_err = reqwest::get("http://0.0.0.0:1").await.unwrap_err();
        let err = ApiError::Transport(reqwest_err);
        assert!(err.to_string().starts_with("transport error:"));
    }

    #[test]
    fn api_error_display_api() {
        let err = ApiError::Api {
            code: ApiErrorCode::RateLimit,
        };
        assert!(err.to_string().starts_with("api error:"));
    }

    #[tokio::test]
    async fn api_error_source_transport_is_some() {
        let reqwest_err = reqwest::get("http://0.0.0.0:1").await.unwrap_err();
        let err = ApiError::Transport(reqwest_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn api_error_source_api_is_none() {
        let err = ApiError::Api {
            code: ApiErrorCode::NotFound,
        };
        assert!(err.source().is_none());
    }

    #[test]
    fn commit_error_from_io_error() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err = CommitError::from(io);
        assert!(matches!(err, CommitError::Io(_)));
    }
}

/// An error returned by any API operation.
///
/// Most methods on [`ApiClient`](crate::ApiClient) and on resource handles
/// return `Result<_, ApiError>`. Match on this enum to distinguish between a
/// transport-level failure and a structured error from the SquareCloud API.
#[derive(Debug)]
pub enum ApiError {
    /// The HTTP request failed before the server could respond.
    Transport(reqwest::Error),
    /// The server responded with a structured API error code.
    Api { code: ApiErrorCode },
}

/// An error that can occur during [`AppResource::commit`](crate::resources::AppResource::commit).
///
/// Committing a file requires building a multipart HTTP request, which may
/// fail for reasons beyond an API error. This enum wraps both cases.
#[derive(Debug)]
pub enum CommitError {
    /// An API-level or transport-level error occurred.
    Api(ApiError),
    /// An I/O error occurred while reading the bytes to be committed.
    Io(std::io::Error),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Transport(e) => write!(f, "transport error: {e}"),
            ApiError::Api { code } => write!(f, "api error: {code:?}"),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Transport(e) => Some(e),
            ApiError::Api { .. } => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Transport(err)
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
