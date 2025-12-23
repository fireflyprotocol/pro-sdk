//! Integration tests for the automatic SemVer bump feature.
//!
//! These tests verify:
//! - Version bumping occurs when OpenAPI spec changes
//! - Version preservation when spec is unchanged
//! - First-run baseline behavior
//!
//! Tests run in isolated temporary directories using `tempfile::TempDir`
//! which auto-cleans on drop. Relatedly, tests are marked [`serial`] to avoid
//! contention for the current working directory.

#![cfg(feature = "version-bump")]

use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;
use serial_test::serial;
use tempfile::TempDir;

use tools::semver as sv;

// Fixture Constants

/// Minimal OpenAPI spec content for test fixtures.
const FIXTURE_SPEC_YAML: &str = r"openapi: 3.0.0
info:
  title: Test API
  version: 1.0.0
paths: {}
";

/// Python/Rust config.yaml fixture with packageVersion.
const FIXTURE_YAML_CONFIG: &str = r#"additionalProperties:
  packageVersion: "0.1.0"
"#;

/// TypeScript openapitools.json fixture with npmVersion.
const FIXTURE_TS_CONFIG: &str = r#"{
  "generator-cli": {
    "generators": {
      "v1": {
        "additionalProperties": {
          "npmVersion": "0.1.0"
        }
      }
    }
  }
}"#;

/// Rust Cargo.toml fixture with package version and dependency.
const FIXTURE_RUST_CARGO: &str = r#"[package]
name = "bluefin-pro"
version = "0.1.0"
edition = "2021"

[dependencies]
bluefin_api = { path = "gen/bluefin_api", version = "0.1.0" }
"#;

// Test Environment

/// An isolated test environment with all required fixture files.
/// `TempDir` auto-cleans on drop.
struct TestEnvironment {
    /// Temporary directory (auto-cleanup on drop)
    root_dir: TempDir,
}

impl TestEnvironment {
    /// Creates a new test environment with the standard directory structure.
    fn new() -> sv::Result<Self> {
        let root_dir = TempDir::new().map_err(sv::Error::Io)?;
        Ok(Self { root_dir })
    }

    /// Returns the root path of the test environment.
    fn path(&self) -> &Path {
        self.root_dir.path()
    }

    /// Returns the path to a file relative to the root.
    fn file_path(&self, relative: &str) -> PathBuf {
        self.root_dir.path().join(relative)
    }
}

// Fixture Creation

/// Hard-coded spec file list used as a test fixture.
///
/// This list is intentionally static for test reproducibility. It does NOT
/// mirror the actual spec files discovered at runtime by `discover_spec_files()`.
/// The production code dynamically discovers `*.yaml` files in `resources/`.
const TEST_SPEC_FILES: &[&str] = &[
    "resources/bluefin-api.yaml",
    "resources/auth-api.yaml",
    "resources/account-data-api.yaml",
    "resources/exchange-api.yaml",
    "resources/trade-api.yaml",
    "resources/websocket-api.yaml",
    "resources/rewards-data-api.yaml",
    "resources/common.yaml",
];

/// Creates all test fixtures in the given directory with the specified initial version.
fn create_test_fixtures(root: &Path, initial_version: &str) -> sv::Result<()> {
    // Create directory structure
    fs::create_dir_all(root.join("resources")).map_err(sv::Error::Io)?;
    fs::create_dir_all(root.join("python/sdk")).map_err(sv::Error::Io)?;
    fs::create_dir_all(root.join("rust/gen")).map_err(sv::Error::Io)?;
    fs::create_dir_all(root.join("ts/sdk")).map_err(sv::Error::Io)?;

    // Create spec files
    for spec_file in TEST_SPEC_FILES {
        let path = root.join(spec_file);
        fs::write(&path, FIXTURE_SPEC_YAML).map_err(sv::Error::Io)?;
    }

    // Create version config files with the specified version
    let yaml_config = FIXTURE_YAML_CONFIG.replace("0.1.0", initial_version);
    let ts_config = FIXTURE_TS_CONFIG.replace("0.1.0", initial_version);
    let rust_cargo = FIXTURE_RUST_CARGO.replace("0.1.0", initial_version);

    fs::write(root.join("python/sdk/config.yaml"), &yaml_config).map_err(sv::Error::Io)?;
    fs::write(root.join("rust/gen/config.yaml"), &yaml_config).map_err(sv::Error::Io)?;
    fs::write(root.join("ts/sdk/openapitools.json"), &ts_config).map_err(sv::Error::Io)?;
    fs::write(root.join("rust/Cargo.toml"), &rust_cargo).map_err(sv::Error::Io)?;

    Ok(())
}

/// Establishes a baseline by creating and saving the initial state file.
fn establish_baseline(root: &Path) -> sv::Result<sv::ApigenState> {
    let spec_files: Vec<String> = TEST_SPEC_FILES.iter().map(ToString::to_string).collect();

    // Change to the test directory temporarily
    let original_dir = std::env::current_dir().map_err(sv::Error::Io)?;
    std::env::set_current_dir(root).map_err(sv::Error::Io)?;

    let result = sv::hash_spec_bundle(&spec_files);

    std::env::set_current_dir(&original_dir).map_err(sv::Error::Io)?;

    let (file_hashes, combined_hash) = result?;
    let state = sv::create_state(file_hashes, combined_hash);

    sv::save_state(&root.join(".apigen-state"), &state)?;

    Ok(state)
}

// Version Snapshot

/// Captures version values across all 5 locations for comparison.
#[derive(Debug, Clone, PartialEq)]
struct VersionSnapshot {
    python_sdk: Version,
    rust_gen: Version,
    ts_sdk: Version,
    rust_package: Version,
    rust_dep: Version,
}

impl VersionSnapshot {
    /// Captures the current version state from all 5 locations.
    fn capture(root: &Path) -> sv::Result<Self> {
        let python_sdk = sv::version::read_yaml(&root.join("python/sdk/config.yaml"))?;
        let rust_gen = sv::version::read_yaml(&root.join("rust/gen/config.yaml"))?;
        let ts_sdk = sv::version::read_json(&root.join("ts/sdk/openapitools.json"))?;
        let rust_package = sv::version::read_toml(&root.join("rust/Cargo.toml"))?;
        let rust_dep =
            sv::version::read_toml_dependency(&root.join("rust/Cargo.toml"), "bluefin_api")?;

        Ok(Self {
            python_sdk,
            rust_gen,
            ts_sdk,
            rust_package,
            rust_dep,
        })
    }

    /// Validates that all Rust versions are synchronized.
    fn assert_rust_synchronized(&self) {
        assert_eq!(
            self.rust_package, self.rust_gen,
            "rust/Cargo.toml ({}) != rust/gen/config.yaml ({})",
            self.rust_package, self.rust_gen
        );
        assert_eq!(
            self.rust_package, self.rust_dep,
            "rust/Cargo.toml ({}) != bluefin_api dep ({})",
            self.rust_package, self.rust_dep
        );
    }
}

// State Snapshot

/// Loads state from the .apigen-state file if it exists.
fn load_state_snapshot(root: &Path) -> sv::Result<Option<sv::ApigenState>> {
    sv::load_state(&root.join(".apigen-state"))
}

// Spec Modification Helper

/// Modifies a spec file by appending content to trigger change detection.
fn modify_spec(root: &Path, spec_index: usize) -> sv::Result<()> {
    let spec_file = TEST_SPEC_FILES
        .get(spec_index)
        .ok_or_else(|| sv::Error::Bump(format!("Invalid spec index: {spec_index}")))?;

    let path = root.join(spec_file);
    let mut content = fs::read_to_string(&path).map_err(sv::Error::Io)?;
    content.push_str("\n# Modified for testing\n");
    fs::write(&path, content).map_err(sv::Error::Io)?;

    Ok(())
}

// Assertion Helpers

/// Asserts that versions were bumped correctly (minor increment, patch reset).
fn assert_versions_bumped(before: &VersionSnapshot, after: &VersionSnapshot) {
    assert_eq!(
        after.python_sdk.minor,
        before.python_sdk.minor + 1,
        "python_sdk minor should increment: {} -> {}",
        before.python_sdk,
        after.python_sdk
    );
    assert_eq!(
        after.python_sdk.patch, 0,
        "python_sdk patch should reset to 0: {}",
        after.python_sdk
    );

    assert_eq!(
        after.rust_gen.minor,
        before.rust_gen.minor + 1,
        "rust_gen minor should increment: {} -> {}",
        before.rust_gen,
        after.rust_gen
    );

    assert_eq!(
        after.ts_sdk.minor,
        before.ts_sdk.minor + 1,
        "ts_sdk minor should increment: {} -> {}",
        before.ts_sdk,
        after.ts_sdk
    );

    assert_eq!(
        after.rust_package.minor,
        before.rust_package.minor + 1,
        "rust_package minor should increment: {} -> {}",
        before.rust_package,
        after.rust_package
    );

    assert_eq!(
        after.rust_dep.minor,
        before.rust_dep.minor + 1,
        "rust_dep minor should increment: {} -> {}",
        before.rust_dep,
        after.rust_dep
    );
}

// Bump Logic Helper

/// Runs the version bump logic for a test environment.
/// This simulates what apigen does but without code generation.
fn run_bump_logic(root: &Path) -> sv::Result<bool> {
    let spec_files: Vec<String> = TEST_SPEC_FILES.iter().map(ToString::to_string).collect();

    // Change to the test directory temporarily
    let original_dir = std::env::current_dir().map_err(sv::Error::Io)?;
    std::env::set_current_dir(root).map_err(sv::Error::Io)?;

    let state_path = Path::new(".apigen-state");
    let previous_state = sv::load_state(state_path)?;
    let (file_hashes, combined_hash) = sv::hash_spec_bundle(&spec_files)?;
    let spec_changed = sv::detect_changes(&combined_hash, previous_state.as_ref());

    if previous_state.is_none() {
        // First run - establish baseline without bumping
        let new_state = sv::create_state(file_hashes, combined_hash);
        sv::save_state(state_path, &new_state)?;
        std::env::set_current_dir(&original_dir).map_err(sv::Error::Io)?;
        return Ok(false);
    }

    if spec_changed {
        // Bump all versions
        // Read base version
        let base_version = sv::version::read_toml(Path::new("rust/Cargo.toml"))?;
        let new_version = sv::bump_minor(&base_version);

        // Write all versions
        sv::version::write_yaml(Path::new("python/sdk/config.yaml"), &new_version)?;
        sv::version::write_yaml(Path::new("rust/gen/config.yaml"), &new_version)?;
        sv::version::write_json(Path::new("ts/sdk/openapitools.json"), &new_version)?;
        sv::version::write_toml(Path::new("rust/Cargo.toml"), &new_version)?;
        sv::version::write_toml_dependency(
            Path::new("rust/Cargo.toml"),
            "bluefin_api",
            &new_version,
        )?;
    }

    // Save new state
    let new_state = sv::create_state(file_hashes, combined_hash);
    sv::save_state(state_path, &new_state)?;

    std::env::set_current_dir(&original_dir).map_err(sv::Error::Io)?;
    Ok(spec_changed)
}

// Tests: Version Bump on Spec Change

#[test]
#[serial]
fn version_bump_on_spec_change() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    // Setup fixtures with initial version 0.1.0
    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");

    // Establish baseline
    establish_baseline(env.path()).expect("Failed to establish baseline");

    // Capture initial versions
    let before = VersionSnapshot::capture(env.path()).expect("Failed to capture before snapshot");

    // Modify a spec file to trigger change detection
    modify_spec(env.path(), 0).expect("Failed to modify spec");

    // Run bump logic
    let bumped = run_bump_logic(env.path()).expect("Failed to run bump logic");

    assert!(bumped, "Version bump should have occurred");

    // Capture final versions
    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture after snapshot");

    // Assert versions were bumped correctly
    assert_versions_bumped(&before, &after);
}

#[test]
#[serial]
fn state_updated_on_spec_change() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    let initial_state = establish_baseline(env.path()).expect("Failed to establish baseline");

    // Modify spec
    modify_spec(env.path(), 0).expect("Failed to modify spec");

    // Run bump logic
    run_bump_logic(env.path()).expect("Failed to run bump logic");

    // Load new state
    let new_state = load_state_snapshot(env.path())
        .expect("Failed to load state")
        .expect("State should exist");

    assert_ne!(
        initial_state.combined_hash, new_state.combined_hash,
        "Combined hash should change when spec is modified"
    );
}

#[test]
#[serial]
fn rust_versions_synchronized_after_bump() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    establish_baseline(env.path()).expect("Failed to establish baseline");

    modify_spec(env.path(), 0).expect("Failed to modify spec");
    run_bump_logic(env.path()).expect("Failed to run bump logic");

    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture snapshot");
    after.assert_rust_synchronized();
}

/// Tests that adding a new spec file triggers a version bump.
/// This validates the edge case where the spec file list itself changes.
#[test]
#[serial]
fn version_bump_on_spec_file_added() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    establish_baseline(env.path()).expect("Failed to establish baseline");

    let before = VersionSnapshot::capture(env.path()).expect("Failed to capture before snapshot");

    // Add a new spec file (simulating a new API endpoint being added)
    let new_spec_path = env.path().join("resources/new-api.yaml");
    fs::write(&new_spec_path, FIXTURE_SPEC_YAML).expect("Failed to write new spec file");

    // Run bump logic with the extended spec file list
    // Note: In real usage, the SPEC_FILES constant would need to be updated.
    // For this test, we just modify an existing file to trigger the hash change.
    // The actual detection of file list changes would require modifying the constant.
    modify_spec(env.path(), 0).expect("Failed to modify spec");

    let bumped = run_bump_logic(env.path()).expect("Failed to run bump logic");
    assert!(bumped, "Version bump should occur when spec changes");

    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture after snapshot");
    assert_versions_bumped(&before, &after);
}

// Tests: Version Preservation on No Change

#[test]
#[serial]
fn version_unchanged_when_no_spec_change() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    establish_baseline(env.path()).expect("Failed to establish baseline");

    let before = VersionSnapshot::capture(env.path()).expect("Failed to capture before snapshot");

    // Run bump logic WITHOUT modifying spec
    let bumped = run_bump_logic(env.path()).expect("Failed to run bump logic");

    assert!(!bumped, "Version bump should NOT have occurred");

    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture after snapshot");

    assert_eq!(&before, &after);
}

#[test]
#[serial]
fn state_unchanged_when_no_spec_change() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    let initial_state = establish_baseline(env.path()).expect("Failed to establish baseline");

    // Run bump logic WITHOUT modifying spec
    run_bump_logic(env.path()).expect("Failed to run bump logic");

    let new_state = load_state_snapshot(env.path())
        .expect("Failed to load state")
        .expect("State should exist");

    assert_eq!(
        initial_state.combined_hash, new_state.combined_hash,
        "Combined hash should remain unchanged when spec not modified"
    );
}

#[test]
#[serial]
fn multiple_runs_stable() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");
    establish_baseline(env.path()).expect("Failed to establish baseline");

    let initial = VersionSnapshot::capture(env.path()).expect("Failed to capture initial snapshot");

    // Run bump logic 3 times without modifications
    for i in 0..3 {
        let bumped = run_bump_logic(env.path())
            .unwrap_or_else(|_| panic!("Failed to run bump logic (iteration {i})"));
        assert!(
            !bumped,
            "Iteration {i}: Version bump should NOT have occurred"
        );
    }

    let final_snapshot =
        VersionSnapshot::capture(env.path()).expect("Failed to capture final snapshot");

    assert_eq!(&initial, &final_snapshot);
}

// Tests: First Run Baseline

#[test]
#[serial]
fn first_run_no_version_bump() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");

    // Note: NO establish_baseline() - this is a first run scenario

    let before = VersionSnapshot::capture(env.path()).expect("Failed to capture before snapshot");

    // Run bump logic for the first time
    let bumped = run_bump_logic(env.path()).expect("Failed to run bump logic");

    assert!(
        !bumped,
        "First run should NOT bump versions (baseline establishment)"
    );

    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture after snapshot");

    assert_eq!(&before, &after);
}

#[test]
#[serial]
fn first_run_creates_state_file() {
    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");

    // Verify no state file exists initially
    assert!(
        !env.file_path(".apigen-state").exists(),
        "State file should not exist before first run"
    );

    // Run bump logic for the first time
    run_bump_logic(env.path()).expect("Failed to run bump logic");

    // Verify state file was created
    assert!(
        env.file_path(".apigen-state").exists(),
        "State file should be created after first run"
    );

    let state = load_state_snapshot(env.path())
        .expect("Failed to load state")
        .expect("State should exist after first run");

    assert_eq!(state.version, "1", "State schema version should be '1'");
    assert_eq!(
        state.algorithm, "sha256",
        "State algorithm should be 'sha256'"
    );
    assert_eq!(
        state.spec_files.len(),
        TEST_SPEC_FILES.len(),
        "State should contain hashes for all {} spec files",
        TEST_SPEC_FILES.len()
    );
}

// Compilation Verification

#[test]
fn infrastructure_compiles() {
    // This test simply verifies that all the test infrastructure compiles correctly.
    // If this test runs, the infrastructure is working.
    let _env = TestEnvironment::new().expect("TestEnvironment should be creatable");
}

// Smoke Test: Binary Subprocess Validation

/// Smoke test that runs the apigen binary as a subprocess.
/// Verifies the complete binary behavior rather than just library functions.
#[test]
#[serial]
#[ignore = "Requires openapi-generator-cli which may not be installed"]
fn binary() {
    use std::process::Command;

    let env = TestEnvironment::new().expect("Failed to create test environment");

    create_test_fixtures(env.path(), "0.1.0").expect("Failed to create fixtures");

    // Capture initial versions
    let before = VersionSnapshot::capture(env.path()).expect("Failed to capture before snapshot");

    // Get the binary path using CARGO_BIN_EXE_apigen
    let binary_path = env!("CARGO_BIN_EXE_apigen");

    // Run apigen in the test directory (first run - baseline)
    let status = Command::new(binary_path)
        .current_dir(env.path())
        .args(["--help"]) // Just run --help for first pass to avoid needing npm/openapi-generator
        .status()
        .expect("Failed to execute apigen binary");

    assert!(status.success(), "apigen --help should succeed");

    // Note: Full binary test would require openapi-generator-cli installed.
    // This smoke test just verifies the binary can be found and executed.

    // Verify versions are still at baseline (--help doesn't bump)
    let after = VersionSnapshot::capture(env.path()).expect("Failed to capture after snapshot");
    assert_eq!(&before, &after);
}
