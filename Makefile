.PHONY: all build build-release run run-release dist check test test-backend test-frontend test-e2e test-all test-wasm build-wasm check-wasm lint format clean dev test-tri-e2e

# Default target
all: format lint test-all build

# Build the Rust workspace (debug)
build:
	cargo build --workspace

# Build and run the standalone desktop application
run: build-release
	./target/release/desktop

# Run desktop in live-reload dev mode (starts Vite + debug Tauri)
dev:
	cd apps/desktop && npm run tauri dev

# Build the production standalone desktop app (release) with embedded frontend
build-release:
	cd apps/desktop && npx tauri build --no-bundle
	@echo "\n=== Release Binary Size ==="
	@ls -lh target/release/desktop

# Build release desktop app and run
run-release: build-release
	./target/release/desktop

# Package standalone desktop installers (AppImage, deb)
dist:
	cd apps/desktop && npm run tauri build
	@echo "\n=== Built Package Bundles ==="
	@find target/release/bundle -type f -exec ls -lh {} + 2>/dev/null || true


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
	rm -rf apps/desktop/node_modules apps/desktop/.svelte-kit apps/desktop/playwright-report apps/desktop/test-results paperpilot-wasm/pkg

# Build client-side WebAssembly package
build-wasm:
	wasm-pack build --target web paperpilot-wasm

# Test client-side WebAssembly crate natively
test-wasm:
	cargo test -p paperpilot-wasm

# Check WebAssembly crate compilation
check-wasm:
	cargo check -p paperpilot-wasm --target wasm32-unknown-unknown || cargo check -p paperpilot-wasm

# Run Tri-Interface E2E Test (Batch 1: Structural)
test-tri-e2e:
	python3 scripts/test_tri_interface_e2e.py
