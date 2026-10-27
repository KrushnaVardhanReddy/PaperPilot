.PHONY: all build build-release run run-release dist check test test-backend test-frontend test-e2e test-all test-wasm build-wasm check-wasm lint format clean clean-sweep clean-cache clean-all-cache dev dev-ui test-tri-e2e preview-docs build-docs run-docs build-linux build-windows build-macos dist-linux dist-windows dist-macos docker-build docker-up docker-down docker-logs docker-shell build-android build-android-release run-android install-android install-android-release log-android

# Auto-detect Windows and configure Git Bash / tools in PATH for GNU Make
ifeq ($(OS),Windows_NT)
  ifneq ($(wildcard C:/Program\ Files/Git/usr/bin/sh.exe),)
    export PATH := C:/Program Files/Git/usr/bin;C:/Program Files/Git/bin;$(PATH)
  else ifneq ($(wildcard $(LOCALAPPDATA)/Programs/Git/bin/sh.exe),)
    export PATH := $(LOCALAPPDATA)/Programs/Git/usr/bin;$(LOCALAPPDATA)/Programs/Git/bin;$(PATH)
  endif
endif

# Default target
all: format lint test-all build

# Build the Rust workspace (debug)
build:
	cargo build --workspace

# Build and run the standalone desktop application
run: build-release
ifeq ($(OS),Windows_NT)
	./target/release/desktop.exe
else
	./target/release/desktop
endif

# Install desktop dependencies
install-desktop:
	@node -e "const fs = require('fs'); const missing = !fs.existsSync('apps/desktop/node_modules') || (!fs.existsSync('apps/desktop/node_modules/.bin/tauri') && !fs.existsSync('apps/desktop/node_modules/.bin/tauri.cmd')) || !fs.existsSync('apps/desktop/node_modules/xlsx'); if (missing) { console.log('Installing desktop dependencies...'); require('child_process').execSync('npm install', { cwd: 'apps/desktop', stdio: 'inherit' }); }"

# Run desktop in live-reload dev mode (starts Vite + debug Tauri)
dev: install-desktop
ifeq ($(OS),Windows_NT)
	@powershell -NoProfile -Command "Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $$_.OwningProcess -Force -ErrorAction SilentlyContinue }" 2>/dev/null || true
else
	@fuser -k 1420/tcp 2>/dev/null || true
endif
	cd apps/desktop && (npx tauri dev || npm run tauri dev)

# Run desktop UI in pure browser Vite dev mode (without Tauri native wrapper)
dev-ui:
	cd apps/desktop && pnpm run dev

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

# Android builds & installation
# Note: uses CARGO_BUILD_JOBS=2 and --no-default-features for lean, responsive mobile compilation
# Auto-detect Android SDK if not already set by user environment
ifeq ($(ANDROID_HOME),)
  ifneq ($(wildcard $(HOME)/Android/Sdk),)
    export ANDROID_HOME := $(HOME)/Android/Sdk
  else ifneq ($(wildcard $(HOME)/AndroidSDK),)
    export ANDROID_HOME := $(HOME)/AndroidSDK
  else ifneq ($(wildcard /usr/lib/android-sdk),)
    export ANDROID_HOME := /usr/lib/android-sdk
  endif
endif

# Auto-detect NDK from ANDROID_HOME if not already set
ifeq ($(NDK_HOME),)
  ifneq ($(wildcard $(ANDROID_HOME)/ndk),)
    export NDK_HOME := $(lastword $(sort $(wildcard $(ANDROID_HOME)/ndk/*)))
  else ifneq ($(ANDROID_NDK_HOME),)
    export NDK_HOME := $(ANDROID_NDK_HOME)
  endif
endif

ifneq ($(ANDROID_HOME),)
  export PATH := $(ANDROID_HOME)/cmdline-tools/latest/bin:$(ANDROID_HOME)/platform-tools:$(PATH)
endif

build-android:
	cd apps/desktop && CARGO_BUILD_JOBS=$${CARGO_BUILD_JOBS:-2} npx tauri android build --debug --target aarch64 --apk -- --no-default-features

build-android-release:
	cd apps/desktop && CARGO_BUILD_JOBS=$${CARGO_BUILD_JOBS:-2} npx tauri android build --target aarch64 --apk -- --no-default-features

run-android:
	cd apps/desktop && CARGO_BUILD_JOBS=$${CARGO_BUILD_JOBS:-2} npx tauri android dev

install-android: build-android
	adb install -r apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk 2>/dev/null || \
	adb install -r apps/desktop/src-tauri/gen/android/app/build/outputs/apk/arm64-v8a/debug/app-arm64-v8a-debug.apk 2>/dev/null || \
	find apps/desktop/src-tauri/gen/android/app/build/outputs/apk -name "*.apk" -exec adb install -r {} +

install-android-release: build-android-release
	adb install -r apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-signed.apk 2>/dev/null || \
	adb install -r apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk 2>/dev/null || \
	find apps/desktop/src-tauri/gen/android/app/build/outputs/apk -name "*release*.apk" -exec adb install -r {} +

log-android:
	adb logcat -v color | grep -iE "paperpilot|tauri|chromium"

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

# Run conformance tests (requires qpdf, verapdf optional)
.PHONY: test-conformance test-cleanroom-top12 test-cleanroom-all50

# Run Clean-Room Audit for Top 12 Launch Tools
test-cleanroom-top12:
	cargo test --test top12_cleanroom_audit -j 2

# Run Clean-Room Audit for All 50 Tools
test-cleanroom-all50:
	cargo test --test all50_cleanroom_audit -j 2

.PHONY: test-conformance
test-conformance:
	./scripts/verify_pdf_conformance.sh

# Run all tests across the stack
test-all: test-backend test-frontend test-e2e test-edge test-embed test-web test-tri-e2e
	@if command -v qpdf >/dev/null 2>&1; then \
		$(MAKE) test-conformance; \
	else \
		echo "Skipping test-conformance as qpdf is not installed"; \
	fi

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

# Run multi-scanner security audit locally (Semgrep + Cargo Audit + Trivy)
audit:
	@echo "🛡️ Running Semgrep SAST scan..."
	@which semgrep >/dev/null 2>&1 || python3 -m pip install semgrep
	semgrep scan --config auto --exclude=".github"
	@echo "🛡️ Running Cargo Audit on Rust dependencies..."
	@which cargo-audit >/dev/null 2>&1 || cargo install cargo-audit --locked
	cargo audit || true
	@echo "🛡️ Running Trivy vulnerability scan..."
	@which trivy >/dev/null 2>&1 || curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b ~/.cargo/bin
	trivy fs --severity HIGH,CRITICAL .


# Clean build artifacts
clean:
	cargo clean
	rm -rf apps/desktop/node_modules apps/desktop/.svelte-kit apps/desktop/playwright-report apps/desktop/test-results paperpilot-wasm/pkg

# Clean stale build artifacts older than 30 days and older toolchains via cargo-sweep
clean-sweep:
	@which cargo-sweep >/dev/null 2>&1 || cargo install cargo-sweep
	cargo sweep --installed
	cargo sweep --time 30

# Clean global ~/.cargo registry and git cache safely via cargo-cache
clean-cache:
	@which cargo-cache >/dev/null 2>&1 || cargo install cargo-cache
	cargo cache --trim-all

# Comprehensive system-wide cleanup: project artifacts + stale builds + global ~/.cargo cache
clean-all-cache: clean clean-sweep clean-cache

# Build client-side WebAssembly package
build-wasm:
	wasm-pack build --target web paperpilot-wasm

# Build self-contained air-gapped single-file HTML distribution (Spec 125)
build-portable: build-wasm
	node scripts/build_portable_html.js

# Test portable air-gapped HTML sandbox (asserts 0 network requests)
test-portable: build-portable
	npx --prefix apps/web playwright test tests/portable_html.spec.ts

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
.PHONY: test-penta-e2e test-penta-page-ops test-penta-security-forms test-penta-analysis test-penta-conversions test-penta-edge-cases bench-penta-e2e

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

# Run Phase 5.9.6: High-Throughput & Concurrency Performance Benchmark
bench-penta-e2e:
	cargo run -p penta-interface-e2e -- --group bench


.PHONY: demo-legal-discovery

# Run Phase 5.9.10: Local AI Agent Legal Discovery Demo
demo-legal-discovery:
	./scripts/run_ai_legal_demo.sh

# ==============================================================================
# Docker Container Targets (Multi-Interface Testing for Any Machine)
# ==============================================================================

# Build the all-interface Docker image
docker-build:
	@mkdir -p data
	docker compose build

# Start the PaperPilot container in background (Gateway + Swagger on :7823)
docker-up:
	@mkdir -p data
	docker compose up -d
	@echo "\nPaperPilot is running!"
	@echo "👉 Open Swagger UI: http://localhost:7823/swagger-ui"
	@echo "👉 Drop files to test into: ./data/"

# Stop the PaperPilot container
docker-down:
	docker compose down

# Stream container runtime logs
docker-logs:
	docker compose logs -f

# Open interactive shell inside the container to test CLI & MCP
docker-shell:
	docker compose exec paperpilot bash || docker exec -it paperpilot bash

# ==============================================================================
# Cloud Agent Delegation Targets (Jules Dispatcher)
# ==============================================================================
.PHONY: jules-list jules-dry-run jules-submit

# List all pending prompt files available for Jules delegation
jules-list:
	./scripts/jules_submit.sh --list

# Test and preview the JSON payload sent to the Jules cloud API without dispatching
# Example: make jules-dry-run FILE=prompts/tasks/phase5/P5_9_75_mobile_lean_wasm_and_ocr.txt
jules-dry-run:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make jules-dry-run FILE=prompts/tasks/phase5/<prompt-file>.txt"; \
		exit 1; \
	fi
	./scripts/jules_submit.sh --file $(FILE) --dry-run

# Dispatch an async Jules session to generate PR for the specified prompt file
# Example: make jules-submit FILE=prompts/tasks/phase5/P5_9_75_mobile_lean_wasm_and_ocr.txt
jules-submit:
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make jules-submit FILE=prompts/tasks/phase5/<prompt-file>.txt"; \
		exit 1; \
	fi
	./scripts/jules_submit.sh --file $(FILE)

# Penta-Scaffold Code Generator
new-tool:
	cargo run -p penta-scaffold -- --name $(NAME) --title "$(TITLE)" --category $(CATEGORY) --icon "$(ICON)" --input-format $(IN) --output-format $(OUT) --description "$(DESC)"
