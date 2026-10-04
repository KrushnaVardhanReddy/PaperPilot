# Spec: Phase 4.F.18 — Developer Mode (Local REST API Sidecar)

## 1. Overview
PaperPilot provides unlimited, offline, local-first PDF processing. To deliver Stirling-PDF API parity and empower local developers, power users, homelab automations (e.g. Python scripts, n8n workflows), PaperPilot features **Developer Mode**.

When Developer Mode is enabled:
1. PaperPilot manages an embedded background process for `paperpilot-gateway` serving on port 7823 (`http://127.0.0.1:7823`).
2. The REST API and live Swagger UI (`http://127.0.0.1:7823/swagger-ui`) become immediately accessible locally.
3. The desktop app exposes the interactive "API & Gateway" tab (`ApiDocsView.svelte`) with live status probing.
4. When Developer Mode is toggled off or the Desktop application exits, the server process is cleanly terminated so port 7823 is freed.

---

## 2. Architecture & Technical Strategy

### 2.1 Backend Tauri Integration
- **Direct Subprocess / Managed Task**:
  - The desktop backend (`apps/desktop/src-tauri`) manages the gateway lifecycle directly via Rust commands (`start_gateway_sidecar`, `stop_gateway_sidecar`, `get_gateway_status`).
  - Leveraging standard Rust process management (`std::process::Command` / `tokio::process::Command` or spawning `paperpilot_gateway::server::start(7823, "127.0.0.1")` in a managed background tokio task / child process) ensures cross-platform reliability without complex platform-specific external binaries or triplet naming during development.
  - The process / task handle is stored in a thread-safe `Arc<Mutex<Option<...>>>` state.
  - When the Tauri app exits (via `tauri::RunEvent::ExitRequested` or `Exit`), any running gateway instance is terminated immediately.

### 2.2 IPC Interface (`src-tauri/src/lib.rs`)
Commands exposed to the frontend:
- `start_gateway(port: Option<u16>) -> Result<GatewayInfo, String>`
- `stop_gateway() -> Result<(), String>`
- `get_gateway_status() -> Result<GatewayStatus, String>`

Data Structures:
```rust
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GatewayStatus {
    pub is_running: bool,
    pub port: u16,
    pub swagger_url: String,
}
```

### 2.3 Frontend State & Preferences (`apps/desktop`)
1. **Settings Persistence**:
   - Persist `developerMode: boolean` in `localStorage` under `paperpilot:developer-mode`.
2. **AppState Extension (`apps/desktop/src/lib/state/app.svelte.ts`)**:
   - `developerMode = $state(false)`
   - `gatewayRunning = $state(false)`
   - `toggleDeveloperMode(enable?: boolean)`: invokes Tauri backend to start/stop the server, updates local state and localStorage.
3. **Settings Panel UI (`apps/desktop/src/lib/components/layout/SettingsPanel.svelte`)**:
   - Dedicated "Developer Mode" card with switch toggle (`id="developer-mode-toggle"`).
   - Shows connection status badge: Active on `http://127.0.0.1:7823`, link to `Swagger UI`, or Inactive.
4. **Navigation Integration (`apps/desktop/src/lib/components/layout/Sidebar.svelte`)**:
   - The "API & Gateway" navigation item (`#nav-api`) is visible and active when Developer Mode is enabled.

---

## 3. Completion & Verification Criteria
1. `cargo check --workspace` and `npm run check` in `apps/desktop` pass with 0 errors.
2. Toggling Developer Mode on triggers the gateway, allowing HTTP requests to `http://127.0.0.1:7823/health` and `http://127.0.0.1:7823/swagger-ui`.
3. Toggling Developer Mode off terminates the process and releases the port.
4. Exiting the desktop app shuts down the gateway server.
5. Unit tests for Tauri commands and component state pass cleanly.
