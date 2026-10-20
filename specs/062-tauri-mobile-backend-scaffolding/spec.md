# Spec 062: Tauri 2 Native Mobile Backend Scaffolding & Native Engine Integration

## 1. Overview & Objective
PaperPilot is built on **Tauri 2**, which natively supports cross-compilation to **Android** (`aarch64-linux-android`) and **iOS** (`aarch64-apple-ios`). This allows our pure-Rust engine (`paperpilot-core`, `paperpilot-pdf`) to execute directly on mobile hardware with native CPU speed and zero cloud dependencies.

Crucially, **mobile platforms do not require desktop-centric services like the REST HTTP Gateway (Axum server on localhost) or the MCP (Model Context Protocol JSON-RPC) server**. Removing/feature-gating these dependencies on mobile reduces compiled binary size by 10MB–20MB, prevents battery/wake-lock draining, and avoids mobile OS background networking restrictions.

This specification sets up the native mobile project scaffolding, lean mobile feature gating, Tauri 2 mobile capabilities, sandboxed file I/O abstraction, and cross-compilation build validation.

---

## 2. Requirements & Deliverables

### Requirement 1: Lean Mobile Feature Gating (Strip MCP & Gateway from Mobile)
- In `apps/desktop/src-tauri/Cargo.toml`:
  - Introduce feature gating for desktop-only dependencies:
    ```toml
    [features]
    default = ["desktop-features"]
    desktop-features = ["dep:paperpilot-gateway", "dep:paperpilot-mcp"]

    [dependencies]
    paperpilot-core = { path = "../../../paperpilot-core" }
    paperpilot-pdf = { path = "../../../paperpilot-pdf" }
    paperpilot-nlp = { path = "../../../paperpilot-nlp" }
    paperpilot-ai = { path = "../../../paperpilot-ai" }

    # Desktop-only services (excluded on mobile)
    paperpilot-gateway = { path = "../../../paperpilot-gateway", optional = true }
    paperpilot-mcp = { path = "../../../paperpilot-mcp", optional = true }
    ```
- In `apps/desktop/src-tauri/src/lib.rs`:
  - Wrap gateway commands (`start_gateway`, `stop_gateway`, `is_gateway_running`) and MCP dispatcher (`invoke_mcp_tool`) with `#[cfg(feature = "desktop-features")]` guards.
  - On mobile, all document processing runs directly against `paperpilot-pdf` traits in-memory, with zero Axum server or MCP JSON-RPC overhead.

### Requirement 2: Tauri 2 Mobile Project Scaffolding
- In `apps/desktop`:
  - Configure `src-tauri/tauri.conf.json`:
    - Identifier: `com.paperpilot.app`
    - Version: `0.1.0`
  - Generate the standard Tauri 2 mobile structure:
    - Android: `gen/android` with Min SDK 26 (Android 8.0) and Target SDK 34 (Android 14).
    - iOS: `gen/apple` targeting iOS 15.0+.
- Configure mobile capabilities in `apps/desktop/src-tauri/capabilities/mobile.json`:
  - Enable native permissions: `core:default`, `dialog:default`, `fs:default`, `opener:default`.

### Requirement 3: Sandboxed Mobile Document Processing (`mobile_bridge.rs`)
- Android Scoped Storage / Storage Access Framework (SAF) and iOS App Sandboxes forbid desktop-style arbitrary absolute paths.
- Add native Tauri command handler in `apps/desktop/src-tauri/src/mobile_bridge.rs`:
  - `process_mobile_buffer(file_name: String, input_bytes: Vec<u8>, operation: String, params: Value) -> Result<Value, String>`
  - Directly invokes `paperpilot-pdf` operations on the byte buffer in-memory.
  - Returns result bytes (Base64 encoded or byte array) and operation metadata.
  - Ensures temporary sandbox files are kept strictly inside the app's sandboxed cache directory and automatically cleaned up via RAII drop guards.

### Requirement 4: Cross-Compilation Validation & Documentation
- Ensure `apps/desktop/src-tauri` compiles cleanly for mobile targets:
  - Add a check script or make target: `cargo check --no-default-features --target aarch64-linux-android` (or verify that no desktop-only dependencies like X11/Wayland break mobile compilation).
- Create `docs/MOBILE_SETUP.md` with:
  - Prerequisites (Android NDK, Android Studio, Xcode, Rust `rustup target add aarch64-linux-android aarch64-apple-ios`).
  - Step-by-step local build and emulator run instructions: `pnpm tauri android dev` / `pnpm tauri ios dev`.
- Generate report: `reports/TAURI_MOBILE_BACKEND_SCAFFOLDING_REPORT.md`.

---

## 3. Success Criteria
- [ ] Mobile build is stripped of MCP and Axum Gateway via `desktop-features` flag.
- [ ] Tauri 2 mobile configurations and capabilities established in `apps/desktop`.
- [ ] In-memory byte buffer processing command available for mobile sandboxes.
- [ ] Cross-compilation passes without desktop-only dependency conflicts.
- [ ] Developer guide documented in `docs/MOBILE_SETUP.md`.
