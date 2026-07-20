use std::fmt;

/// Runtime version to use, declared as `VERSION` in the config file.
///
/// See the version table for each language at
/// <https://docs.squarecloud.app/getting-started/config-file>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuntimeVersion {
    /// The version SquareCloud recommends for the application's language.
    #[default]
    Recommended,
    /// The newest available version for the application's language.
    Latest,
}

impl RuntimeVersion {
    fn as_wire(self) -> &'static str {
        match self {
            Self::Recommended => "recommended",
            Self::Latest => "latest",
        }
    }
}

/// A [`ConfigFile`] field violated one of SquareCloud's documented limits.
///
/// See <https://docs.squarecloud.app/getting-started/config-file> for the
/// authoritative list of limits.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConfigFileError {
    /// `display_name` is longer than 32 characters.
    DisplayNameTooLong {
        /// The maximum allowed length.
        limit: usize,
        /// The actual length that was provided.
        actual: usize,
    },
    /// `main` is longer than 32 characters.
    MainTooLong {
        /// The maximum allowed length.
        limit: usize,
        /// The actual length that was provided.
        actual: usize,
    },
    /// `description` is longer than 280 characters.
    DescriptionTooLong {
        /// The maximum allowed length.
        limit: usize,
        /// The actual length that was provided.
        actual: usize,
    },
    /// `start` is longer than 256 characters.
    StartTooLong {
        /// The maximum allowed length.
        limit: usize,
        /// The actual length that was provided.
        actual: usize,
    },
    /// `subdomain` is longer than 63 characters.
    SubdomainTooLong {
        /// The maximum allowed length.
        limit: usize,
        /// The actual length that was provided.
        actual: usize,
    },
    /// `memory` is below the minimum (256 MB, or 512 MB when `subdomain` is
    /// set).
    MemoryTooLow {
        /// The minimum required for this configuration.
        minimum: u32,
        /// The actual value that was provided.
        actual: u32,
    },
}

impl fmt::Display for ConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DisplayNameTooLong { limit, actual } => write!(
                f,
                "display_name is {actual} characters long, the limit is {limit}"
            ),
            Self::MainTooLong { limit, actual } => {
                write!(
                    f,
                    "main is {actual} characters long, the limit is {limit}"
                )
            }
            Self::DescriptionTooLong { limit, actual } => write!(
                f,
                "description is {actual} characters long, the limit is {limit}"
            ),
            Self::StartTooLong { limit, actual } => write!(
                f,
                "start is {actual} characters long, the limit is {limit}"
            ),
            Self::SubdomainTooLong { limit, actual } => write!(
                f,
                "subdomain is {actual} characters long, the limit is {limit}"
            ),
            Self::MemoryTooLow { minimum, actual } => {
                write!(f, "memory is {actual}MB, the minimum is {minimum}MB")
            }
        }
    }
}

impl std::error::Error for ConfigFileError {}

const DISPLAY_NAME_LIMIT: usize = 32;
const MAIN_LIMIT: usize = 32;
const DESCRIPTION_LIMIT: usize = 280;
const START_LIMIT: usize = 256;
const SUBDOMAIN_LIMIT: usize = 63;
const MIN_MEMORY: u32 = 256;
const MIN_MEMORY_WITH_SUBDOMAIN: u32 = 512;

/// Builds a [`ConfigFile`], SquareCloud's application deployment
/// configuration.
///
/// `display_name`, `main`, and `memory` are required; everything else has a
/// sensible default and is set through the chained methods below. Call
/// [`build`](Self::build) to validate the result and get a [`ConfigFile`].
///
/// ```
/// use squarecloud::ConfigFile;
///
/// let config = ConfigFile::builder("My Bot", "index.js", 256)
///     .description("A cool bot")
///     .auto_restart(true)
///     .build()
///     .unwrap();
/// assert!(config.content().contains("DISPLAY_NAME=My Bot"));
/// ```
#[derive(Debug, Clone)]
pub struct ConfigFileBuilder {
    display_name: String,
    main: String,
    memory: u32,
    version: RuntimeVersion,
    description: Option<String>,
    subdomain: Option<String>,
    start: Option<String>,
    auto_restart: bool,
}

impl ConfigFileBuilder {
    /// Starts a new builder with the required fields. `memory` is in
    /// megabytes. `version` defaults to [`RuntimeVersion::Recommended`] and
    /// `auto_restart` defaults to `false`.
    pub fn new(
        display_name: impl Into<String>,
        main: impl Into<String>,
        memory: u32,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            main: main.into(),
            memory,
            version: RuntimeVersion::default(),
            description: None,
            subdomain: None,
            start: None,
            auto_restart: false,
        }
    }

    /// Sets the runtime version. Defaults to [`RuntimeVersion::Recommended`].
    pub fn version(mut self, version: RuntimeVersion) -> Self {
        self.version = version;
        self
    }

    /// Sets a human-readable description, shown on the dashboard.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the subdomain, exposing the application as a website at
    /// `<subdomain>.squareweb.app`. Raises the minimum required `memory`
    /// from 256 MB to 512 MB.
    pub fn subdomain(mut self, subdomain: impl Into<String>) -> Self {
        self.subdomain = Some(subdomain.into());
        self
    }

    /// Sets a custom startup command, overriding the platform's default
    /// invocation of `main` for the application's language.
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// Sets whether the application should restart automatically if it
    /// crashes. Defaults to `false`.
    pub fn auto_restart(mut self, auto_restart: bool) -> Self {
        self.auto_restart = auto_restart;
        self
    }

    /// Validates every field against SquareCloud's documented limits and
    /// produces the final [`ConfigFile`].
    ///
    /// # Errors
    ///
    /// Returns the first [`ConfigFileError`] found, checked in the order
    /// the fields are documented at
    /// <https://docs.squarecloud.app/getting-started/config-file>.
    pub fn build(self) -> Result<ConfigFile, ConfigFileError> {
        let display_name_len = self.display_name.chars().count();
        if display_name_len > DISPLAY_NAME_LIMIT {
            return Err(ConfigFileError::DisplayNameTooLong {
                limit: DISPLAY_NAME_LIMIT,
                actual: display_name_len,
            });
        }

        let main_len = self.main.chars().count();
        if main_len > MAIN_LIMIT {
            return Err(ConfigFileError::MainTooLong {
                limit: MAIN_LIMIT,
                actual: main_len,
            });
        }

        if let Some(description) = &self.description {
            let len = description.chars().count();
            if len > DESCRIPTION_LIMIT {
                return Err(ConfigFileError::DescriptionTooLong {
                    limit: DESCRIPTION_LIMIT,
                    actual: len,
                });
            }
        }

        if let Some(start) = &self.start {
            let len = start.chars().count();
            if len > START_LIMIT {
                return Err(ConfigFileError::StartTooLong {
                    limit: START_LIMIT,
                    actual: len,
                });
            }
        }

        if let Some(subdomain) = &self.subdomain {
            let len = subdomain.chars().count();
            if len > SUBDOMAIN_LIMIT {
                return Err(ConfigFileError::SubdomainTooLong {
                    limit: SUBDOMAIN_LIMIT,
                    actual: len,
                });
            }
        }

        let min_memory = if self.subdomain.is_some() {
            MIN_MEMORY_WITH_SUBDOMAIN
        } else {
            MIN_MEMORY
        };
        if self.memory < min_memory {
            return Err(ConfigFileError::MemoryTooLow {
                minimum: min_memory,
                actual: self.memory,
            });
        }

        Ok(ConfigFile {
            display_name: self.display_name,
            main: self.main,
            memory: self.memory,
            version: self.version,
            description: self.description,
            subdomain: self.subdomain,
            start: self.start,
            auto_restart: self.auto_restart,
        })
    }
}

/// A validated SquareCloud application deployment configuration.
///
/// Renders to the `KEY=value` content of a `squarecloud.app` (or
/// `squarecloud.config`) file. Construct one with
/// [`ConfigFile::builder`]/[`ConfigFileBuilder`]; every `ConfigFile` you hold
/// has already passed SquareCloud's documented field limits, so
/// [`content`](Self::content) cannot fail.
///
/// This only generates the file's *content* — packaging it into a zip
/// alongside your application files, at the archive root, is up to you. See
/// [`Client::upload_app`](crate::Client::upload_app) and
/// [`AppResource::commit`](crate::resources::AppResource::commit), which
/// both accept the resulting archive as raw bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFile {
    display_name: String,
    main: String,
    memory: u32,
    version: RuntimeVersion,
    description: Option<String>,
    subdomain: Option<String>,
    start: Option<String>,
    auto_restart: bool,
}

impl ConfigFile {
    /// Starts a [`ConfigFileBuilder`] with the required fields. Shorthand
    /// for [`ConfigFileBuilder::new`].
    pub fn builder(
        display_name: impl Into<String>,
        main: impl Into<String>,
        memory: u32,
    ) -> ConfigFileBuilder {
        ConfigFileBuilder::new(display_name, main, memory)
    }

    /// Renders the `KEY=value` content of a `squarecloud.app` config file,
    /// in the field order SquareCloud documents them.
    pub fn content(&self) -> String {
        let mut lines = vec![
            format!("MAIN={}", self.main),
            format!("MEMORY={}", self.memory),
            format!("VERSION={}", self.version.as_wire()),
            format!("DISPLAY_NAME={}", self.display_name),
        ];
        if let Some(description) = &self.description {
            lines.push(format!("DESCRIPTION={description}"));
        }
        if self.auto_restart {
            lines.push("AUTORESTART=true".to_string());
        }
        if let Some(subdomain) = &self.subdomain {
            lines.push(format!("SUBDOMAIN={subdomain}"));
        }
        if let Some(start) = &self.start {
            lines.push(format!("START={start}"));
        }
        lines.join("\n")
    }
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.content())
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigFile, ConfigFileError, RuntimeVersion};

    #[test]
    fn minimal_config_renders_required_fields_only() {
        let config = ConfigFile::builder("Robin bot", "index.js", 256)
            .build()
            .unwrap();
        assert_eq!(
            config.content(),
            "MAIN=index.js\nMEMORY=256\nVERSION=recommended\nDISPLAY_NAME=Robin bot"
        );
    }

    #[test]
    fn full_config_renders_every_field_in_documented_order() {
        let config = ConfigFile::builder("My website!", "index.js", 512)
            .version(RuntimeVersion::Latest)
            .description("My website is very cool!")
            .auto_restart(true)
            .subdomain("mysite")
            .start("npm run build && npm run start")
            .build()
            .unwrap();
        assert_eq!(
            config.content(),
            "MAIN=index.js\n\
             MEMORY=512\n\
             VERSION=latest\n\
             DISPLAY_NAME=My website!\n\
             DESCRIPTION=My website is very cool!\n\
             AUTORESTART=true\n\
             SUBDOMAIN=mysite\n\
             START=npm run build && npm run start"
        );
    }

    #[test]
    fn auto_restart_false_is_omitted() {
        let config = ConfigFile::builder("Bot", "index.js", 256)
            .auto_restart(false)
            .build()
            .unwrap();
        assert!(!config.content().contains("AUTORESTART"));
    }

    #[test]
    fn display_impl_matches_content() {
        let config =
            ConfigFile::builder("Bot", "index.js", 256).build().unwrap();
        assert_eq!(config.to_string(), config.content());
    }

    #[test]
    fn display_name_over_32_chars_is_rejected() {
        let name = "a".repeat(33);
        let err = ConfigFile::builder(name, "index.js", 256)
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::DisplayNameTooLong {
                limit: 32,
                actual: 33
            }
        );
        assert_eq!(
            err.to_string(),
            "display_name is 33 characters long, the limit is 32"
        );
    }

    #[test]
    fn display_name_at_32_chars_is_accepted() {
        let name = "a".repeat(32);
        assert!(ConfigFile::builder(name, "index.js", 256).build().is_ok());
    }

    #[test]
    fn display_name_length_counts_unicode_chars_not_bytes() {
        // 32 emoji, 4 bytes each in UTF-8: would be well past any byte-based
        // 32-byte limit, but is exactly 32 *characters*.
        let name = "🚀".repeat(32);
        assert_eq!(name.chars().count(), 32);
        assert!(name.len() > 32);
        assert!(ConfigFile::builder(name, "index.js", 256).build().is_ok());
    }

    #[test]
    fn main_over_32_chars_is_rejected() {
        let main = "a".repeat(33);
        let err = ConfigFile::builder("Bot", main, 256).build().unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::MainTooLong {
                limit: 32,
                actual: 33
            }
        );
        assert_eq!(
            err.to_string(),
            "main is 33 characters long, the limit is 32"
        );
    }

    #[test]
    fn description_over_280_chars_is_rejected() {
        let description = "a".repeat(281);
        let err = ConfigFile::builder("Bot", "index.js", 256)
            .description(description)
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::DescriptionTooLong {
                limit: 280,
                actual: 281
            }
        );
        assert_eq!(
            err.to_string(),
            "description is 281 characters long, the limit is 280"
        );
    }

    #[test]
    fn start_over_256_chars_is_rejected() {
        let start = "a".repeat(257);
        let err = ConfigFile::builder("Bot", "index.js", 256)
            .start(start)
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::StartTooLong {
                limit: 256,
                actual: 257
            }
        );
        assert_eq!(
            err.to_string(),
            "start is 257 characters long, the limit is 256"
        );
    }

    #[test]
    fn subdomain_over_63_chars_is_rejected() {
        let subdomain = "a".repeat(64);
        let err = ConfigFile::builder("Bot", "index.js", 512)
            .subdomain(subdomain)
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::SubdomainTooLong {
                limit: 63,
                actual: 64
            }
        );
        assert_eq!(
            err.to_string(),
            "subdomain is 64 characters long, the limit is 63"
        );
    }

    #[test]
    fn memory_below_256_is_rejected() {
        let err = ConfigFile::builder("Bot", "index.js", 255)
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::MemoryTooLow {
                minimum: 256,
                actual: 255
            }
        );
    }

    #[test]
    fn memory_256_without_subdomain_is_accepted() {
        assert!(ConfigFile::builder("Bot", "index.js", 256).build().is_ok());
    }

    #[test]
    fn memory_256_with_subdomain_is_rejected() {
        let err = ConfigFile::builder("Site", "index.js", 256)
            .subdomain("mysite")
            .build()
            .unwrap_err();
        assert_eq!(
            err,
            ConfigFileError::MemoryTooLow {
                minimum: 512,
                actual: 256
            }
        );
    }

    #[test]
    fn memory_512_with_subdomain_is_accepted() {
        assert!(
            ConfigFile::builder("Site", "index.js", 512)
                .subdomain("mysite")
                .build()
                .is_ok()
        );
    }

    #[test]
    fn error_display_is_human_readable() {
        let err = ConfigFileError::MemoryTooLow {
            minimum: 512,
            actual: 256,
        };
        assert_eq!(err.to_string(), "memory is 256MB, the minimum is 512MB");
    }
}
