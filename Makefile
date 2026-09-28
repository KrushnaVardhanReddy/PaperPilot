.PHONY: all build check test lint format clean test-e2e

# Default target
all: format lint test build

# Build the entire workspace
build:
	cargo build --workspace

# Check for compilation errors without building binaries
check:
	cargo check --workspace

# Run all unit and integration tests
test:
	cargo test --workspace

# Run the end-to-end and fixture-based integration tests specifically
test-e2e:
	cargo test --test '*'

# Format the codebase
format:
	cargo fmt --all

# Lint the codebase using Clippy
lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	cargo clean
