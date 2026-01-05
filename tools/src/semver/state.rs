//! State management for apigen change detection.
//!
//! Handles persistence and comparison of spec file hashes to detect changes.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Error, Result};

/// State record persisted to .apigen-state for change detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApigenState {
    /// Schema version for forward compatibility.
    pub version: String,
    /// Hash algorithm used (always "sha256").
    pub algorithm: String,
    /// Timestamp of last successful generation.
    pub generated_at: DateTime<Utc>,
    /// Map of spec file path to SHA-256 hash.
    pub spec_files: HashMap<String, String>,
    /// Combined hash of all spec files.
    pub combined_hash: String,
}

/// Loads the state file if it exists.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read or parsed.
pub fn load_state(path: &Path) -> Result<Option<ApigenState>> {
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(path)?;
    let state: ApigenState = serde_json::from_str(&contents)?;
    Ok(Some(state))
}

/// Saves state to the state file atomically (temp file + rename).
///
/// # Errors
///
/// Returns an error if the file cannot be written.
pub fn save_state(path: &Path, state: &ApigenState) -> Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    let json = serde_json::to_string_pretty(state)?;
    temp.write_all(json.as_bytes())?;
    temp.persist(path).map_err(|e| Error::Io(e.error))?;
    Ok(())
}

/// Returns true if the spec has changed since the last recorded state. Returns
/// false if `previous.is_none()`, meaning there is no last recorded state, or
/// if the spec has not changed.
#[must_use]
pub fn detect_changes(current_hash: &str, previous: Option<&ApigenState>) -> bool {
    previous.is_some_and(|state| current_hash != state.combined_hash)
}

/// Creates a new `ApigenState` from the current spec hashes.
#[must_use]
#[expect(clippy::implicit_hasher)]
pub fn create_state(file_hashes: HashMap<String, String>, combined_hash: String) -> ApigenState {
    ApigenState {
        version: "1".to_string(),
        algorithm: "sha256".to_string(),
        generated_at: Utc::now(),
        spec_files: file_hashes,
        combined_hash,
    }
}
