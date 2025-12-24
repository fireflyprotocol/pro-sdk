//! SDK language definitions.
//!
//! Defines the target languages for code generation and version management.

use std::str::FromStr;

#[cfg(feature = "version-bump")]
use crate::semver::version::Format;

/// The target language for code generation and version management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Python,
    Rust,
    TypeScript,
}

impl Lang {
    /// Returns a repo-relative path to the OpenAPI config for this language.
    #[must_use]
    pub const fn config(self) -> &'static str {
        match self {
            Lang::Python => "python/sdk/config.yaml",
            Lang::Rust => "rust/gen/config.yaml",
            Lang::TypeScript => "ts/sdk/openapitools.json",
        }
    }

    /// Returns an OpenAPI generator name.
    #[must_use]
    pub fn generator(self) -> &'static str {
        match self {
            Lang::Python => "python",
            Lang::Rust => "rust",
            Lang::TypeScript => "typescript-axios",
        }
    }

    /// Returns the target directory path where code should be generated.
    #[must_use]
    pub fn output(self) -> &'static str {
        match self {
            Lang::Python => "python/sdk/src",
            Lang::Rust => "rust/gen/bluefin_api",
            Lang::TypeScript => "ts/sdk/src",
        }
    }

    /// Returns the path to the package manifest for this language.
    #[must_use]
    pub const fn package_manifest(self) -> &'static str {
        match self {
            Lang::Python => "python/sdk/pyproject.toml",
            Lang::Rust => "rust/Cargo.toml",
            Lang::TypeScript => "ts/sdk/package.json",
        }
    }

    /// Returns the version format for the OpenAPI config file.
    #[cfg(feature = "version-bump")]
    #[must_use]
    pub fn config_format(self) -> Format {
        match self {
            Lang::Python | Lang::Rust => Format::Yaml,
            Lang::TypeScript => Format::Json,
        }
    }

    /// Returns the version format for the package manifest.
    #[cfg(feature = "version-bump")]
    #[must_use]
    pub fn manifest_format(self) -> Format {
        match self {
            Lang::Python => Format::PyprojectToml,
            Lang::Rust => Format::Toml,
            Lang::TypeScript => Format::PackageJson,
        }
    }

    /// Returns all version file locations for this language.
    ///
    /// Each language has two version files:
    /// - OpenAPI generator config (from `config()`)
    /// - Package manifest (from `package_manifest()`)
    #[cfg(feature = "version-bump")]
    #[must_use]
    pub fn version_locations(self) -> [(&'static str, Format); 2] {
        [
            (self.config(), self.config_format()),
            (self.package_manifest(), self.manifest_format()),
        ]
    }
}

/// Error type for Lang parsing.
#[derive(Debug)]
pub struct ParseLangError(pub String);

impl std::fmt::Display for ParseLangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a supported language", self.0)
    }
}

impl std::error::Error for ParseLangError {}

impl FromStr for Lang {
    type Err = ParseLangError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "py" | "python" => Ok(Lang::Python),
            "rs" | "rust" => Ok(Lang::Rust),
            "ts" | "typescript" | "typescript-axios" => Ok(Lang::TypeScript),
            _ => Err(ParseLangError(s.to_owned())),
        }
    }
}
