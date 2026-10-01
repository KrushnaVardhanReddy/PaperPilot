.PHONY: all build check test test-backend test-frontend test-e2e test-all lint format clean dev

# Default target
all: format lint test-all build

# Build the Rust workspace
build:
	cargo build --workspace

# Check for Rust compilation errors and Svelte errors
check:
	cargo check --workspace
	cd apps/desktop && npm run check

# Run all backend unit and integration tests
test-backend:
	cargo test --workspace

# Run frontend unit/component tests (if any, otherwise just check)
test-frontend:
	cd apps/desktop && npm run check

# Run the end-to-end Playwright tests for the frontend/Tauri app
test-e2e:
	cd apps/desktop && npx playwright test

# Run Playwright in UI mode to manually step through and test every feature/button
test-e2e-ui:
	cd apps/desktop && npx playwright test --ui

# Run all tests across the stack
test-all: test-backend test-frontend test-e2e

# Backwards compatibility alias for backend tests
test: test-backend

# Format the Rust and frontend codebase
format:
	cargo fmt --all
	cd apps/desktop && npm run format || true

# Lint the codebase using Clippy and ESLint
lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd apps/desktop && npm run lint || true

# Clean build artifacts
clean:
	cargo clean
	rm -rf apps/desktop/node_modules apps/desktop/.svelte-kit apps/desktop/playwright-report apps/desktop/test-results

# Start the Tauri dev environment
dev:
	cd apps/desktop && npm run tauri dev
