#!/usr/bin/env bash

# =====================================================
# rust-jpl crate publish automation
# Author: Chinmay Vivek
# Purpose: Automate pre-checks and publish to crates.io
# =====================================================

set -e  # exit on any error
set -o pipefail

# --- Config ---
MIN_RUST_VERSION="1.70.0"
CRATE_NAME="rust-jpl"

echo "🚀 Starting publish process for $CRATE_NAME..."

# --- 1. Check Rust version ---
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "Detected Rust version: $RUST_VERSION"

if [ "$(printf '%s\n' "$MIN_RUST_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$MIN_RUST_VERSION" ]; then
    echo "⚠️ Rust version must be >= $MIN_RUST_VERSION"
    exit 1
fi

# --- 2. Ensure Git repo is clean ---
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️ Git repository is not clean. Commit or stash changes first."
    git status
    exit 1
fi

# --- 3. Run formatting and linting ---
echo "🔧 Running cargo fmt..."
cargo fmt --all -- --check

echo "🔍 Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# --- 4. Run tests ---
echo "🧪 Running tests..."
cargo test --all

# --- 5. Build docs ---
echo "📚 Building documentation..."
cargo doc --no-deps

# --- 6. Check package ---
echo "📦 Verifying package contents (dry run)..."
cargo package --allow-dirty --no-verify --dry-run

# --- 7. Confirm publish ---
read -p "✅ All checks passed. Proceed to publish $CRATE_NAME to crates.io? [y/N]: " CONFIRM
CONFIRM=${CONFIRM,,}  # convert to lowercase
if [[ "$CONFIRM" != "y" ]]; then
    echo "❌ Publish aborted."
    exit 0
fi

# --- 8. Publish ---
echo "🚀 Publishing $CRATE_NAME to crates.io..."
cargo publish

echo "🎉 Publish completed successfully!"
