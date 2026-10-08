.PHONY: all build build-release run run-release dist check test test-backend test-frontend test-e2e test-all test-wasm build-wasm check-wasm lint format clean dev test-tri-e2e preview-docs build-docs run-docs build-linux build-windows build-macos dist-linux dist-windows dist-macos

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

# Run web portal in live-reload dev mode
run-web:
	cd apps/web && npm run dev

# Run embed widget dev server
run-embed:
	cd apps/embed && npm run dev

# Run Cloudflare edge worker locally
run-edge:
	cd apps/edge && npm run dev

# Run documentation portal in live-reload dev mode
run-docs:
	cd apps/docs && npm run dev

# Run both web portal (5173) and docs portal (4321) concurrently
run-all-web:
	npx concurrently -n "web,docs" -c "blue,green" "cd apps/web && npm run dev" "cd apps/docs && npm run dev"

# Build documentation portal static bundle (with search index)
build-docs:
	cd apps/docs && npm run build

# Preview documentation portal production build with working Pagefind search
preview-docs: build-docs
	cd apps/docs && npm run preview

# Build the production standalone desktop app (release) with embedded frontend
build-release:
	cd apps/desktop && npx tauri build --no-bundle
	@echo "\n=== Release Binary Size ==="
	@ls -lh target/release/desktop

# Cross-compilation & Multi-Platform builds
# Linux (native or target)
build-linux:
	cargo build --workspace --release --target x86_64-unknown-linux-gnu

# Windows cross-compilation
build-windows:
	cargo build --workspace --release --target x86_64-pc-windows-gnu

# macOS cross-check / notice (macOS binaries require Darwin host/SDK or GitHub Actions runner)
build-macos:
	@echo "Notice: Compiling Tauri / macOS binaries locally requires macOS (Darwin) or a cross-toolchain."
	@echo "Trigger GitHub Actions Rust CI or use: gh workflow run rust.yml for macOS build."

# Package standalone desktop installers (AppImage, deb)
dist:
	cd apps/desktop && npm run tauri build
	@echo "\n=== Built Package Bundles ==="
	@find target/release/bundle -type f -exec ls -lh {} + 2>/dev/null || true

# Distribution packaging per target platform
dist-linux:
	cd apps/desktop && npx tauri build --bundles appimage,deb

dist-windows:
	cd apps/desktop && npx tauri build --target x86_64-pc-windows-msvc --bundles nsis,msi

dist-macos:
	@echo "Notice: Packaging macOS DMGs/bundles requires a macOS runner (see .github/workflows/rust.yml)."


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
test-all: test-backend test-frontend test-e2e test-edge test-embed test-web test-tri-e2e

# Run Edge Microservice tests
test-edge:
	cd apps/edge && npm run test

# Run Embed Widget tests
test-embed:
	cd apps/embed && npx playwright test

# Run Web Portal tests
test-web:
	cd apps/web && npx playwright test

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

# Run Tri-Interface E2E Test (Legacy Python suite)
test-tri-e2e:
	python3 scripts/test_tri_interface_e2e.py

# ==============================================================================
# Pure-Rust Penta-Interface E2E Real Semantic Assertions Suite (Spec 031)
# ==============================================================================
.PHONY: test-penta-e2e test-penta-page-ops test-penta-security-forms test-penta-analysis test-penta-conversions test-penta-edge-cases

# Run all 44 tools across all 5 interfaces (CLI, MCP, REST, WASM, Edge) and 4 tiers
test-penta-e2e:
	cargo run -p penta-interface-e2e

# Run Group A: 9 Page Operations tools (180 tests)
test-penta-page-ops:
	cargo run -p penta-interface-e2e -- --group page_ops

# Run Group B: 14 Security, Stamping & Forms tools (280 tests)
test-penta-security-forms:
	cargo run -p penta-interface-e2e -- --group security_forms

# Run Group C: 13 Extraction, Analysis & Optimization tools (260 tests)
test-penta-analysis:
	cargo run -p penta-interface-e2e -- --group analysis

# Run Group D: 8 Document & Office Conversions tools (160 tests)
test-penta-conversions:
	cargo run -p penta-interface-e2e -- --group conversions

# Run Phase 5.9.5: Penta-Interface Edge Cases & Deep Boundary Verification Suite
test-penta-edge-cases:
	cargo run -p penta-interface-e2e -- --group edge_cases

