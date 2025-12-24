//! Version bump logic and validation.
//!
//! Handles minor version bumping and cross-file version synchronization.

use std::path::Path;

use semver::Version;

use super::{version, Error, Result};
use crate::Lang;

/// Bumps the minor version and resets patch to 0.
#[must_use]
pub fn bump_minor(ver: &Version) -> Version {
    Version::new(ver.major, ver.minor + 1, 0)
}

/// Bumps versions for the given locations and returns the new version.
///
/// The base version is always read from rust/Cargo.toml (the canonical source).
/// Only the locations provided will be updated.
///
/// # Errors
///
/// Returns an error if reading or writing any version file fails.
pub fn bump_all_versions(
    version_locations: &[&version::Location],
    rust_cargo_path: &str,
) -> Result<Version> {
    // Read the base version from Rust Cargo.toml (canonical source)
    let base_version = version::read_toml(Path::new(Lang::Rust.package_manifest()))?;
    let new_version = bump_minor(&base_version);

    println!("Bumping versions...");

    // Write all versions atomically (best effort - each file write is atomic)
    for loc in version_locations {
        version::write(loc, &new_version)?;
        println!("  {}: {} → {}", loc.path, base_version, new_version);
    }

    // Update the dependency version too (only if Rust is being bumped)
    let updating_rust = version_locations.iter().any(|loc| loc.lang == Lang::Rust);
    if updating_rust {
        let dep_path = Path::new(rust_cargo_path);
        version::write_toml_dependency(dep_path, "bluefin_api", &new_version)?;
        println!("  {rust_cargo_path} (bluefin_api dep): {base_version} → {new_version}");
    }

    Ok(new_version)
}

/// Validates that all Rust-related versions match after a bump.
///
/// # Errors
///
/// Returns an error if versions are out of sync or files cannot be read.
pub fn validate_rust_versions() -> Result<()> {
    let cargo_path = Lang::Rust.package_manifest();
    let config_path = Lang::Rust.config();

    let rust_cargo = version::read_toml(Path::new(cargo_path))?;
    let rust_gen = version::read_yaml(Path::new(config_path))?;
    let rust_dep = version::read_toml_dependency(Path::new(cargo_path), "bluefin_api")?;

    if rust_cargo != rust_gen {
        return Err(Error::Bump(format!(
            "{cargo_path} ({rust_cargo}) != {config_path} ({rust_gen})"
        )));
    }

    if rust_cargo != rust_dep {
        return Err(Error::Bump(format!(
            "{cargo_path} ({rust_cargo}) != bluefin_api dependency ({rust_dep})"
        )));
    }

    Ok(())
}
