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
    use crate::http::errors::{BlobErrorCode, EnvErrorCode};

    #[test]
    fn compares_with_domain_enums_and_strings() {
        let code = ErrorCode::from("OBJECT_NOT_FOUND");
        assert!(code == BlobErrorCode::ObjectNotFound);
        assert!(code != BlobErrorCode::InvalidObject);
        assert!(code == "OBJECT_NOT_FOUND");
        assert!(code != EnvErrorCode::RegexValidation);
    }

    #[test]
    fn compares_with_unknown_variants_by_wire_string() {
        let code = ErrorCode::from("TOO_MANY_ENV_VARS");
        assert!(code == EnvErrorCode::Unknown("TOO_MANY_ENV_VARS".into()));
    }

    #[test]
    fn display_prints_raw_code() {
        assert_eq!(ErrorCode::from("KEEP_CALM").to_string(), "KEEP_CALM");
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
