use std::{fmt, io};

/// Error type for version bump operations.
#[derive(Debug)]
pub enum Error {
    /// System interaction failed.
    Io(io::Error),
    /// Version bump operation failed.
    Bump(String),
    /// JSON parsing/serialization failed.
    Json(serde_json::Error),
    /// YAML parsing/serialization failed.
    Yaml(serde_yaml::Error),
    /// TOML parsing failed.
    TomlParse(toml::de::Error),
    /// TOML serialization failed.
    TomlSerialize(toml::ser::Error),
    /// SemVer parsing failed.
    SemVer(semver::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO: {e}"),
            Error::Bump(s) => write!(f, "version bump failed: {s}"),
            Error::Json(e) => write!(f, "JSON: {e}"),
            Error::Yaml(e) => write!(f, "YAML: {e}"),
            Error::TomlParse(e) => write!(f, "TOML parse: {e}"),
            Error::TomlSerialize(e) => write!(f, "TOML serialize: {e}"),
            Error::SemVer(e) => write!(f, "SemVer parse: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Error::Yaml(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::TomlParse(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::TomlSerialize(value)
    }
}

impl From<semver::Error> for Error {
    fn from(value: semver::Error) -> Self {
        Error::SemVer(value)
    }
}
