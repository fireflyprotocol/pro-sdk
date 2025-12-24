//! Version reading and writing for multiple file formats.
//!
//! Supports YAML, JSON, and TOML configuration files.

use std::fs;
use std::io::Write;
use std::path::Path;

use semver::Version;

use super::{Error, Result};
use crate::Lang;

/// Format of a version file (determines how to parse/write it).
#[derive(Debug, Clone, Copy)]
pub enum Format {
    /// OpenAPI generator YAML config (additionalProperties.packageVersion)
    Yaml,
    /// OpenAPI generator JSON config (generator-cli.generators.v1.additionalProperties.npmVersion)
    Json,
    /// Rust Cargo.toml (package.version)
    Toml,
    /// NPM package.json (top-level version field)
    PackageJson,
    /// Python pyproject.toml (project.version)
    PyprojectToml,
}

/// A location where a version number needs to be updated.
#[derive(Debug, Clone)]
pub struct Location {
    pub path: &'static str,
    pub format: Format,
    pub lang: Lang,
}

// Version Reading

/// Reads the version from a YAML config file (packageVersion field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read_yaml(path: &Path) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&contents)?;

    let version_str = yaml
        .get("additionalProperties")
        .and_then(|ap| ap.get("packageVersion"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            Error::Bump(format!("packageVersion not found in {}", path.display()))
        })?;

    Ok(Version::parse(version_str)?)
}

/// Reads the version from a JSON config file (npmVersion field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read_json(path: &Path) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&contents)?;

    let version_str = json
        .get("generator-cli")
        .and_then(|gc| gc.get("generators"))
        .and_then(|g| g.get("v1"))
        .and_then(|v1| v1.get("additionalProperties"))
        .and_then(|ap| ap.get("npmVersion"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Bump(format!("npmVersion not found in {}", path.display())))?;

    Ok(Version::parse(version_str)?)
}

/// Reads the package version from a TOML file (package.version field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read_toml(path: &Path) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let toml: toml::Value = toml::from_str(&contents)?;

    let version_str = toml
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            Error::Bump(format!("package.version not found in {}", path.display()))
        })?;

    Ok(Version::parse(version_str)?)
}

/// Reads the version from a package.json file (top-level version field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read_package_json(path: &Path) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let json: serde_json::Value = serde_json::from_str(&contents)?;

    let version_str = json
        .get("version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Bump(format!("version not found in {}", path.display())))?;

    Ok(Version::parse(version_str)?)
}

/// Reads the version from a pyproject.toml file (project.version field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read_pyproject_toml(path: &Path) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let toml_value: toml::Value = toml::from_str(&contents)?;

    let version_str = toml_value
        .get("project")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            Error::Bump(format!("project.version not found in {}", path.display()))
        })?;

    Ok(Version::parse(version_str)?)
}

/// Reads a version from the specified location.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
pub fn read(loc: &Location) -> Result<Version> {
    let path = Path::new(loc.path);
    match loc.format {
        Format::Yaml => read_yaml(path),
        Format::Json => read_json(path),
        Format::Toml => read_toml(path),
        Format::PackageJson => read_package_json(path),
        Format::PyprojectToml => read_pyproject_toml(path),
    }
}

/// Reads the dependency version from a TOML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or the dependency is not found.
pub fn read_toml_dependency(path: &Path, dep_name: &str) -> Result<Version> {
    let contents = fs::read_to_string(path)?;
    let toml_value: toml::Value = toml::from_str(&contents)?;

    let version_str = toml_value
        .get("dependencies")
        .and_then(|d| d.get(dep_name))
        .and_then(|dep| {
            // Handle both { path = "...", version = "..." } and simple string
            dep.as_str()
                .or_else(|| dep.get("version").and_then(|v| v.as_str()))
        })
        .ok_or_else(|| {
            Error::Bump(format!(
                "dependencies.{}.version not found in {}",
                dep_name,
                path.display()
            ))
        })?;

    Ok(Version::parse(version_str)?)
}

// Version Writing

/// Writes a new version to a YAML config file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_yaml(path: &Path, new_version: &Version) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut yaml: serde_yaml::Value = serde_yaml::from_str(&contents)?;

    if let Some(ap) = yaml.get_mut("additionalProperties")
        && let Some(pv) = ap.get_mut("packageVersion")
    {
        *pv = serde_yaml::Value::String(new_version.to_string());
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let yaml_str = serde_yaml::to_string(&yaml)?;
    temp.write_all(yaml_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}

/// Writes a new version to a JSON config file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_json(path: &Path, new_version: &Version) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut json: serde_json::Value = serde_json::from_str(&contents)?;

    if let Some(gc) = json.get_mut("generator-cli")
        && let Some(g) = gc.get_mut("generators")
        && let Some(v1) = g.get_mut("v1")
        && let Some(ap) = v1.get_mut("additionalProperties")
        && let Some(nv) = ap.get_mut("npmVersion")
    {
        *nv = serde_json::Value::String(new_version.to_string());
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let json_str = serde_json::to_string_pretty(&json)?;
    temp.write_all(json_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}

/// Writes a new package version to a TOML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_toml(path: &Path, new_version: &Version) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut toml_value: toml::Value = toml::from_str(&contents)?;

    if let Some(package) = toml_value.get_mut("package")
        && let Some(v) = package.get_mut("version")
    {
        *v = toml::Value::String(new_version.to_string());
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let toml_str = toml::to_string_pretty(&toml_value)?;
    temp.write_all(toml_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}

/// Writes a new version to a package.json file (top-level version field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_package_json(path: &Path, new_version: &Version) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut json: serde_json::Value = serde_json::from_str(&contents)?;

    if let Some(v) = json.get_mut("version") {
        *v = serde_json::Value::String(new_version.to_string());
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let json_str = serde_json::to_string_pretty(&json)?;
    temp.write_all(json_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}

/// Writes a new version to a pyproject.toml file (project.version field).
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_pyproject_toml(path: &Path, new_version: &Version) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut toml_value: toml::Value = toml::from_str(&contents)?;

    if let Some(project) = toml_value.get_mut("project")
        && let Some(v) = project.get_mut("version")
    {
        *v = toml::Value::String(new_version.to_string());
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let toml_str = toml::to_string_pretty(&toml_value)?;
    temp.write_all(toml_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}

/// Writes a version to the specified location.
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write(loc: &Location, new_version: &Version) -> Result<()> {
    let path = Path::new(loc.path);
    match loc.format {
        Format::Yaml => write_yaml(path, new_version),
        Format::Json => write_json(path, new_version),
        Format::Toml => write_toml(path, new_version),
        Format::PackageJson => write_package_json(path, new_version),
        Format::PyprojectToml => write_pyproject_toml(path, new_version),
    }
}

/// Writes a dependency version to a TOML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or written.
pub fn write_toml_dependency(
    path: &Path,
    dep_name: &str,
    new_version: &Version,
) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut toml_value: toml::Value = toml::from_str(&contents)?;

    if let Some(deps) = toml_value.get_mut("dependencies")
        && let Some(dep) = deps.get_mut(dep_name)
    {
        // Handle table format: { path = "...", version = "..." }
        if let Some(v) = dep.get_mut("version") {
            *v = toml::Value::String(new_version.to_string());
        }
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let toml_str = toml::to_string_pretty(&toml_value)?;
    temp.write_all(toml_str.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;

    Ok(())
}
