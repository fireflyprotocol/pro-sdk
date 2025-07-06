#!/bin/bash
set -e

echo "Installing OpenAPI Generator CLI..."
npm install -g @openapitools/openapi-generator-cli

echo "Setting Rust toolchain to stable..."
rustup default stable

echo "Updating Rust..."
rustup update

echo "Installing Rust components..."
rustup component add rustfmt clippy

echo "Installing cargo-edit..."
cargo install cargo-edit

echo "Installing cargo-update..."
cargo install cargo-update

echo "Installing tools..."
cargo install --path /workspace/tools

echo "Installing Node.js dependencies..."
npm install ts-node

echo "Installing Python dependencies..."
pipx install poetry

echo "Setting up Python environment..."
cd /workspace/python/sdk && \
python -m venv .venv && \
source .venv/bin/activate && \
poetry install

echo "Post-create setup complete!"
