#!/usr/bin/env python3
"""
Version bump script for pro-sdk.

Detects OpenAPI spec changes via SHA-256 hashing and bumps SDK versions
in the generator config files.

Usage:
    python3 scripts/version_bump.py [--no-bump] [--major]
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

STATE_FILE = ".apigen-state"
RESOURCES_DIR = "resources"

# --- Hashing ---


def hash_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def hash_spec_bundle(resources_dir: Path) -> tuple[dict[str, str], str]:
    spec_files = sorted(resources_dir.glob("*.yaml"))
    if not spec_files:
        return {}, ""

    file_hashes: dict[str, str] = {}
    for f in spec_files:
        file_hashes[str(f)] = hash_file(f)

    combined = "".join(file_hashes[k] for k in sorted(file_hashes))
    combined_hash = hashlib.sha256(combined.encode()).hexdigest()
    return file_hashes, combined_hash


# --- State ---


def load_state(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    state: dict[str, Any] = json.loads(path.read_text())
    return state


def save_state(path: Path, file_hashes: dict[str, str], combined_hash: str) -> None:
    state: dict[str, Any] = {
        "version": "1",
        "algorithm": "sha256",
        "generated_at": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "spec_files": file_hashes,
        "combined_hash": combined_hash,
    }
    path.write_text(json.dumps(state, indent=2) + "\n")


# --- Version bumping ---


def bump_major(version: str) -> str:
    major, _minor, _patch = version.split(".")
    return f"{int(major) + 1}.0.0"


def bump_minor(version: str) -> str:
    major, minor, _patch = version.split(".")
    return f"{major}.{int(minor) + 1}.0"


def read_yaml_version(path: Path) -> str:
    """Read packageVersion from a YAML config."""
    for line in path.read_text().splitlines():
        m = re.match(r"\s*packageVersion:\s*['\"]?([0-9.]+)['\"]?", line)
        if m:
            return m.group(1)
    raise ValueError(f"packageVersion not found in {path}")


def write_yaml_version(path: Path, new_version: str) -> None:
    """Write packageVersion in a YAML config."""
    text = path.read_text()
    text = re.sub(
        r"(packageVersion:\s*)['\"]?[0-9.]+['\"]?",
        rf"\g<1>{new_version}",
        text,
    )
    path.write_text(text)


def write_json_version(path: Path, new_version: str) -> None:
    """Write version fields in a JSON generator config."""
    data: dict[str, Any] = json.loads(path.read_text())
    props = data.get("additionalProperties")
    if isinstance(props, dict):
        props["npmVersion"] = new_version
        if "packageVersion" in props:
            props["packageVersion"] = new_version
    path.write_text(json.dumps(data, indent=2) + "\n")


def write_toml_version(path: Path, new_version: str) -> None:
    """Write version in a TOML file (pyproject.toml or Cargo.toml)."""
    text = path.read_text()
    text = re.sub(
        r'(version\s*=\s*")[0-9.]+"',
        rf'\g<1>{new_version}"',
        text,
        count=1,
    )
    path.write_text(text)


def write_package_json_version(path: Path, new_version: str) -> None:
    """Write version in a package.json file."""
    data: dict[str, Any] = json.loads(path.read_text())
    data["version"] = new_version
    path.write_text(json.dumps(data, indent=2) + "\n")


def print_changes(file_hashes: dict[str, str], previous: dict[str, Any]) -> None:
    prev_files: dict[str, str] = previous.get("spec_files", {})
    print("Changed files:")
    for path, current_hash in file_hashes.items():
        prev_hash = prev_files.get(path)
        if prev_hash is None:
            print(f"  {path} (new)")
        elif current_hash != prev_hash:
            print(f"  {path} (modified)")
    for path in prev_files:
        if path not in file_hashes:
            print(f"  {path} (deleted)")


# --- Main ---


def main() -> None:
    skip_bump = "--no-bump" in sys.argv
    major_bump = "--major" in sys.argv

    # cd to repo root
    repo_root = Path(__file__).resolve().parent.parent
    os.chdir(repo_root)

    resources = Path(RESOURCES_DIR)
    if not resources.is_dir():
        print(f"Error: {RESOURCES_DIR}/ not found")
        sys.exit(1)

    file_hashes, combined_hash = hash_spec_bundle(resources)
    if not file_hashes:
        print(f"No spec files found in {RESOURCES_DIR}/")
        sys.exit(0)

    state_path = Path(STATE_FILE)
    previous = load_state(state_path)

    if previous is None:
        print("First run — establishing baseline (no version bump)")
        for path, h in file_hashes.items():
            print(f"  {path}: {h[:16]}...")
        save_state(state_path, file_hashes, combined_hash)
        print(f"State saved to {STATE_FILE}")
        sys.exit(0)

    if combined_hash == previous.get("combined_hash"):
        print("No spec changes detected")
        sys.exit(0)

    print_changes(file_hashes, previous)

    if skip_bump:
        print("Version bump skipped (--no-bump)")
    else:
        current_version = read_yaml_version(Path("python/sdk/config.yaml"))
        new_version = bump_major(current_version) if major_bump else bump_minor(current_version)
        bump_type = "major (breaking)" if major_bump else "minor"
        print(f"Bumping versions ({bump_type}): {current_version} -> {new_version}")

        # Generator configs
        write_yaml_version(Path("python/sdk/config.yaml"), new_version)
        print(f"  python/sdk/config.yaml -> {new_version}")

        write_yaml_version(Path("rust/gen/config.yaml"), new_version)
        print(f"  rust/gen/config.yaml -> {new_version}")

        write_json_version(Path("ts/sdk/openapitools.json"), new_version)
        print(f"  ts/sdk/openapitools.json -> {new_version}")

        # Package manifests
        manifests: list[tuple[str, Any]] = [
            ("python/sdk/pyproject.toml", write_toml_version),
            ("rust/Cargo.toml", write_toml_version),
            ("ts/sdk/package.json", write_package_json_version),
        ]
        for rel_path, writer in manifests:
            p = Path(rel_path)
            if p.exists():
                writer(p, new_version)
                print(f"  {rel_path} -> {new_version}")

    save_state(state_path, file_hashes, combined_hash)
    print(f"State saved to {STATE_FILE}")


if __name__ == "__main__":
    main()
