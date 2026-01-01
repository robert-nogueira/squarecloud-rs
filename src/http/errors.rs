use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorCode {
    FewMemory,
    BadMemory,
    MissingConfig,
    InvalidDependency,
    MissingMain,
    InvalidMain,
    InvalidDisplayName,
    MissingDisplayName,
    InvalidMemory,
    MissingMemory,
    InvalidVersion,
    MissingVersion,
    InvalidAccessToken,
    RegexValidation,
    InvalidStart,
    InvalidSubdomain,
}

pub enum ApiError {
    Transport(reqwest::Error),
    Api { code: ApiErrorCode },
}

pub enum CommitError {
    Api(ApiError),
    Io(std::io::Error),
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
