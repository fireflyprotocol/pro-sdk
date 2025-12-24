//! File hashing utilities for change detection.
//!
//! Provides SHA-256 hashing for individual files and spec bundles.

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::{Error, Result};

/// Computes the SHA-256 hash of a file's contents.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let hash = Sha256::digest(&contents);
    Ok(format!("{hash:x}"))
}

/// Hashes all spec files and returns (individual hashes, combined hash).
///
/// # Errors
///
/// Returns an error if any spec file is missing or cannot be read.
pub fn hash_spec_bundle(spec_files: &[String]) -> Result<(HashMap<String, String>, String)> {
    let mut file_hashes = HashMap::new();

    for spec_file in spec_files {
        let path = Path::new(spec_file);
        if !path.exists() {
            return Err(Error::Bump(format!("spec file not found: {spec_file}")));
        }
        let hash = hash_file(path)?;
        file_hashes.insert(spec_file.clone(), hash);
    }

    // Compute combined hash from sorted file hashes
    let mut sorted_paths: Vec<_> = file_hashes.keys().collect();
    sorted_paths.sort();

    let mut combined = String::new();
    for path in sorted_paths {
        combined.push_str(&file_hashes[path]);
    }

    let combined_hash = format!("{:x}", Sha256::digest(combined.as_bytes()));

    Ok((file_hashes, combined_hash))
}
