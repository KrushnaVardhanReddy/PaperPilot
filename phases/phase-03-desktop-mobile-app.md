## Phase 3 — Desktop + Mobile Application (Tauri 2.0)

**Goal:** A polished, cross-platform Tauri 2.0 app for PDF manipulation — running natively on Windows, macOS, Linux, iOS, and Android from a single codebase.

**Technology Decision (LOCKED):** Tauri 2.0 + Svelte 5 + TypeScript.
- **Why Tauri 2.0:** Official iOS and Android support landed in v2.0. The Rust backend (`paperpilot-core`) is shared across all targets with zero changes — only the UI shell differs. Apache 2.0 licensed.
- **Why Svelte:** Minimal bundle size (critical for mobile), no runtime overhead, compiles to vanilla JS.

**Exit condition:** A user with no technical knowledge can open the app on any platform (desktop or mobile), drop in PDFs, perform all basic operations, and get output files — without touching the CLI.

**[BLOCKED BY Phase 1 exit condition]**
**[Can start in parallel with Phase 2]**

---

### 3.1 — Project Setup `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 3.1.1 | Initialize Tauri 2.0 project ✅ | `apps/desktop/src-tauri/` fully scaffolded |
| 3.1.2 | Configure Svelte 5 + TypeScript + Vite ✅ | Svelte 5.56, TypeScript 6.0, Vite 8 |
| 3.1.3 | Set up Tauri ↔ Rust command bindings ✅ | `invoke_mcp_tool` in `src-tauri/src/lib.rs` (Phase 3.5) |
| 3.1.4 | Set up component library / design system ✅ | Custom vanilla CSS design system in `app.css` |
| 3.1.5 | Define Tauri IPC schema ✅ | `jobs.svelte.ts` + `serde_json::Value`-typed commands |
| 3.1.6 | Set up frontend tests ✅ | Playwright E2E & Vitest component unit tests merged |
| 3.1.7 | Configure iOS target | Requires macOS (Xcode) to run `cargo tauri ios init` |
| 3.1.8 | Configure Android target ✅ | `apps/desktop/src-tauri/gen/android` scaffolded |

---

### 3.2 — Core UI Shell `[PARALLEL within group, BLOCKED BY 3.1]`

| # | Task | Notes |
|---|---|---|
| 3.2.1 | App layout and navigation ✅ | Merged (Session 6096383570980483046) |
| 3.2.2 | File drop zone ✅ | Merged (Session 6096383570980483046) |
| 3.2.3 | Document list ✅ | Merged (Session 11119733351266944393) |
| 3.2.4 | Settings panel ✅ | Merged (Session 11119733351266944393) |
| 3.2.5 | Notification / toast system ✅ | Merged (Session 6096383570980483046) |

---

### 3.3 — PDF Preview `[PARALLEL with 3.2, BLOCKED BY 3.1]`

| # | Task | Notes |
|---|---|---|
| 3.3.1 | Page renderer ✅ | Merged (Session 10488084682857299204) |
| 3.3.2 | Page thumbnail strip ✅ | Merged (Session 10488084682857299204) |
| 3.3.3 | Page selection ✅ | Merged (Session 10488084682857299204) |
| 3.3.4 | Page reorder via drag ✅ | Merged (Session 6565731315931125703) |
| 3.3.5 | Page rotation controls ✅ | Merged (Session 6565731315931125703) |
| 3.3.6 | Page deletion UI ✅ | Merged (Session 6565731315931125703) |

---

### 3.4 — Operations UI `[BLOCKED BY 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.4.1 | Operation panel ✅ | Merged (Session 11119733351266944393) |
| 3.4.2 | Merge UI ✅ | Merged (Session 11119733351266944393) |
| 3.4.3 | Split UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.4 | Compress UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.5 | Rotate UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.6 | Watermark UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.7 | Encrypt/Decrypt UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.8 | Extract UI ✅ | Merged (Session 10488084682857299204) |
| 3.4.9 | Output file picker ✅ | Merged (Session 10488084682857299204) |
| 3.4.10 | **Visual Pipeline Builder (Basic)** ✅ | Merged |
| 3.4.11 | **Conditional branching UI `[Spec 138]`** 🚀 | Add IF/ELSE diamond nodes and branching visual connections to `PipelineCanvas.svelte`. In Progress — Jules Session `15126192642974815608` on branch `feat/desktop-conditional-branching-ui-task-3-4-11`. |
| 3.4.12 | **Pipeline trigger config** | Choose trigger: manual, watch folder, webhook, or schedule |
| 3.4.13 | **Desktop V2 Workspace UX Refresh `[Spec 140]`** 📋 | Refactor to an integrated 3-pane layout: (1) Left — Library & Automations (recent files, smart SLM categories, watched folders, shared-with-you), (2) Center — Focused document canvas with frosted-glass floating toolbar and status bar, (3) Right — Contextual AI-driven tool dock (recommendations + grouped tools). Replaces the current tab-switching navigation with a unified workspace that keeps the tool dock and document reader side by side at all times. **Blocked by 3.4.11.** |

---

### 3.5 — Job System `[BLOCKED BY 3.1, PARALLEL with 3.2]`

| # | Task | Notes |
|---|---|---|
| 3.5.1 | Rust async job runner ✅ | Merged (Session 8999512547197594932) |
| 3.5.2 | Tauri event bridge ✅ | Merged (Session 8999512547197594932) |
| 3.5.3 | Progress bar component ✅ | Merged |
| 3.5.4 | Job cancel support ✅ | Merged |
| 3.5.5 | Job history panel ✅ | Merged (Session 8999512547197594932) |
| 3.5.6 | Responsive mobile layout ✅ | Merged (Session 13229007488227647984) |
| 3.5.7 | E2E Desktop tests ✅ | Merged (Session 617520399890233763) |
| 3.5.8 | E2E Mobile tests (Simulator) | iOS Simulator + Android Emulator automated tests via Appium or Detox (No Mocking) |

---

### 3.6 — Release / Distribution `[BLOCKED BY 3.2, 3.4]`

| # | Task | Notes |
|---|---|---|
| 3.6.1 | Portable Windows Build (`.exe`) ✅ | Merged |
| 3.6.2 | Portable Linux Build (`AppImage`) ✅ | Merged |
| 3.6.3 | Portable macOS Build (`.app` / `.dmg`) ✅ | Merged |
| 3.6.4 | MSI / MDM Installers (Enterprise) | Create traditional MSI installers for IT deployment (Intune, SCCM). |

---

