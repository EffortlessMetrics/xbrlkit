#!/bin/bash
# Pre-push quality gate for xbrlkit
# Usage: ./scripts/pre-push.sh

set -e

echo "🔍 Running pre-push checks..."

echo "  → cargo fmt --all --check"
cargo fmt --all --check

echo "  → cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo "  → cargo test --workspace"
cargo test --workspace

echo "  → cargo xtask alpha-check"
cargo xtask alpha-check

echo "✅ All pre-push checks passed!"
