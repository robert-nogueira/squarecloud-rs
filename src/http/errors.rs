use serde::{Deserialize, Serialize};

/// Machine-readable error code returned by the SquareCloud API.
///
/// The wire format uses `SCREAMING_SNAKE_CASE` (e.g. `"FEW_MEMORY"`).
/// Match on this enum after receiving an [`ApiError::Api`] to act on the
/// specific cause of the failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    /// A code returned by the API that this client does not recognise.
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::ApiErrorCode;

    #[test]
    fn known_codes_deserialize_from_screaming_snake_case() {
        let cases = [
            (r#""RATE_LIMIT""#, ApiErrorCode::RateLimit),
            (r#""NOT_FOUND""#, ApiErrorCode::NotFound),
            (r#""KEEP_CALM""#, ApiErrorCode::KeepCalm),
            (r#""INVALID_FILE""#, ApiErrorCode::InvalidFile),
            (r#""CONTAINER_ALREADY_STARTED""#, ApiErrorCode::ContainerAlreadyStarted),
            (r#""INVALID_TIME_RANGE""#, ApiErrorCode::InvalidTimeRange),
            (r#""NO_CUSTOM_DOMAIN""#, ApiErrorCode::NoCustomDomain),
            (r#""INVALID_ACCESS_TOKEN""#, ApiErrorCode::InvalidAccessToken),
        ];
        for (input, expected) in cases {
            let got: ApiErrorCode = serde_json::from_str(input).unwrap();
            assert_eq!(got, expected, "failed for {input}");
        }
    }

    #[test]
    fn known_codes_serialize_to_screaming_snake_case() {
        let cases = [
            (ApiErrorCode::RateLimit, "RATE_LIMIT"),
            (ApiErrorCode::NotFound, "NOT_FOUND"),
            (ApiErrorCode::KeepCalm, "KEEP_CALM"),
            (ApiErrorCode::ContainerAlreadyStarted, "CONTAINER_ALREADY_STARTED"),
        ];
        for (code, expected) in cases {
            let got = serde_json::to_string(&code).unwrap();
            assert_eq!(got, format!(r#""{expected}""#));
        }
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
