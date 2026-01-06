//! Semantic version management.
//!
//! This module supports:
//!
//! - Spec change detection via SHA-256 hashing
//! - Version reading/writing across YAML, JSON, and TOML formats
//! - Version bumping with minor increment and patch reset
//! - Rust version synchronization validation

mod bump;
mod error;
mod hash;
mod state;
pub mod version;

use std::{fs, path::Path};

use crate::Lang;

// Re-exports for public API
pub use bump::{bump_all_versions, bump_minor, validate_rust_versions};
pub use hash::{hash_file, hash_spec_bundle};
pub use state::{ApigenState, create_state, detect_changes, load_state, save_state};

pub use error::Error;

/// Result type for version bump operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Path to the state file that tracks spec hashes.
pub const STATE_FILE: &str = ".apigen-state";

/// Directory containing OpenAPI spec files.
const RESOURCES_DIR: &str = "resources";

/// Discovers all OpenAPI spec files in the resources directory.
///
/// Scans `resources/` for `*.yaml` files and returns them sorted alphabetically
/// to ensure deterministic hash calculations.
///
/// # Errors
///
/// Returns an error if the resources directory cannot be read.
pub fn discover_spec_files() -> Result<Vec<String>> {
    let resources_path = Path::new(RESOURCES_DIR);

    if !resources_path.exists() {
        return Err(Error::Bump(format!(
            "resources directory not found: {RESOURCES_DIR}"
        )));
    }

    let mut spec_files: Vec<String> = fs::read_dir(resources_path)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "yaml"))
        .filter_map(|entry| entry.path().to_str().map(ToString::to_string))
        .collect();

    spec_files.sort();
    Ok(spec_files)
}

/// All version locations that need to be updated when spec changes.
pub const VERSION_LOCATIONS: &[version::Location] = &[
    version::Location {
        path: Lang::Python.config(),
        format: version::Format::Yaml,
        lang: Lang::Python,
    },
    version::Location {
        path: Lang::Rust.config(),
        format: version::Format::Yaml,
        lang: Lang::Rust,
    },
    version::Location {
        path: Lang::TypeScript.config(),
        format: version::Format::Json,
        lang: Lang::TypeScript,
    },
    version::Location {
        path: Lang::Rust.package_manifest(),
        format: version::Format::Toml,
        lang: Lang::Rust,
    },
    version::Location {
        path: Lang::TypeScript.package_manifest(),
        format: version::Format::PackageJson,
        lang: Lang::TypeScript,
    },
    version::Location {
        path: Lang::Python.package_manifest(),
        format: version::Format::PyprojectToml,
        lang: Lang::Python,
    },
];

/// Prints baseline information when the version bump workflow runs for the first time.
///
/// Displays the list of spec files being tracked along with truncated hashes
/// for debugging purposes.
fn print_baseline_info(
    spec_files: &[String],
    file_hashes: &std::collections::BTreeMap<String, String>,
) {
    println!("First run detected; establishing baseline (no version bump)");
    println!("Spec files being tracked:");
    for spec_file in spec_files {
        if let Some(hash) = file_hashes.get(spec_file) {
            println!("  {}: {}...", spec_file, &hash[..16]);
        }
    }
}

/// Prints which spec files have changed since the last run.
///
/// Compares current file hashes against the previous state and prints
/// files that were modified, newly added, or deleted.
fn print_changed_files(
    file_hashes: &std::collections::BTreeMap<String, String>,
    previous_state: &ApigenState,
) {
    println!("Changed files:");
    for (path, current_hash) in file_hashes {
        if let Some(prev_hash) = previous_state.spec_files.get(path) {
            if current_hash != prev_hash {
                println!("  {path} (modified)");
            }
        } else {
            println!("  {path} (new)");
        }
    }
    for path in previous_state.spec_files.keys() {
        if !file_hashes.contains_key(path) {
            println!("  {path} (deleted)");
        }
    }
}

/// Bumps versions for the specified languages.
///
/// Builds version locations dynamically from `Lang::version_locations()` and
/// bumps their versions. If Rust is included, also validates that Rust versions
/// are in sync.
///
/// # Errors
///
/// Returns an error if version files cannot be read or written, or if Rust
/// version validation fails.
fn bump_versions_for_langs(langs: &[Lang]) -> Result<()> {
    let target_langs = if langs.is_empty() {
        vec![Lang::Python, Lang::Rust, Lang::TypeScript]
    } else {
        langs.to_vec()
    };

    let locations: Vec<version::Location> = target_langs
        .iter()
        .flat_map(|lang| {
            lang.version_locations()
                .into_iter()
                .map(|(path, format)| version::Location {
                    path,
                    format,
                    lang: *lang,
                })
        })
        .collect();

    if locations.is_empty() {
        println!("No version locations match selected languages");
        return Ok(());
    }

    let location_refs: Vec<&version::Location> = locations.iter().collect();
    bump_all_versions(&location_refs, Lang::Rust.package_manifest())?;

    if langs.is_empty() || langs.contains(&Lang::Rust) {
        validate_rust_versions()?;
    }

    Ok(())
}

/// Runs the complete version bump workflow:
///
/// - Detects OpenAPI spec changes
/// - Bumps SDK package versions for the specified `langs`, iff specs changed
///   and `skip_bump` is false
/// - Hashes spec contents, and saves the hashes to local disk on the first run
///   and whenever specs have changed
///
/// Returns true if spec changes were detected, and false otherwise.
///
/// # Errors
///
/// Returns an error if spec files cannot be hashed, state cannot be
/// read/written, or version files cannot be updated.
pub fn run(skip_bump: bool, langs: &[Lang]) -> Result<bool> {
    debug_assert!(!langs.is_empty());

    let spec_files = discover_spec_files()?;

    if spec_files.is_empty() {
        println!("No spec files found in {RESOURCES_DIR}/");
        return Ok(false);
    }

    let state_path = Path::new(STATE_FILE);
    let (file_hashes, combined_hash) = hash_spec_bundle(&spec_files)?;
    let previous_state = load_state(state_path)?;
    let has_changes = detect_changes(&combined_hash, previous_state.as_ref());

    match &previous_state {
        None => print_baseline_info(&spec_files, &file_hashes),
        Some(prev) if has_changes => {
            print_changed_files(&file_hashes, prev);
            if skip_bump {
                println!("Version bump skipped (--no-bump flag)");
            } else {
                bump_versions_for_langs(langs)?;
            }
        }
        Some(_) => {} // No changes, nothing to do
    }

    if previous_state.is_none() || has_changes {
        let new_state = create_state(file_hashes, combined_hash);
        save_state(state_path, &new_state)?;
        println!("State saved to {STATE_FILE}");
    }

    Ok(has_changes)
}
