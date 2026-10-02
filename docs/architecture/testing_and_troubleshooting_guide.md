# PaperPilot — 3-Tier Testing & Troubleshooting Architecture Guide

> **Target Audience:** Autonomous Coding Agents (Google Jules, Antigravity, OpenCode) & Human Contributors.  
> **Purpose:** Establishes the authoritative diagnostic methodology for verifying and debugging PaperPilot across its three distinct software tiers: **MCP Engine**, **Tauri API / IPC Bridge**, and **Svelte 5 Desktop UI**.

---

## 🏛️ System Architecture Layers

PaperPilot is deliberately structured into three decoupled layers:

```
┌────────────────────────────────────────────────────────┐
│  Tier 3: Svelte 5 Desktop UI                           │
│  - User interaction, canvas rendering, runes reactive  │
│  - Visual styling, CSS design tokens, accessibility    │
└──────────────────────────▲─────────────────────────────┘
                           │ IPC invoke() / Events
┌──────────────────────────▼─────────────────────────────┐
│  Tier 2A: Tauri 2.0 API & IPC Bridge                   │
│  - Command dispatch (generate_handler![]), permissions │
│  - Security capabilities (capabilities/default.json)   │
│  - Serialization / deserialization boundary (serde)    │
└──────────────────────────▲─────────────────────────────┘
                           │
┌──────────────────────────▼─────────────────────────────┐
│  Tier 2B: REST API & Remote MCP Gateway (Axum :7823)   │
│  - Driving Adapter for Automation (n8n, curl, scripts) │
│  - Remote MCP over HTTP/SSE (/mcp/sse)                 │
│  - OpenAPI / Swagger spec documentation (/docs)        │
└──────────────────────────▲─────────────────────────────┘
                           │ Hexagonal Ports (PdfOperation & MCP Registry)
┌──────────────────────────▼─────────────────────────────┐
│  Tier 1: MCP Server & Rust Core Engine                 │
│  - 45+ PDF operations (lopdf, pdf-writer, pdfium)      │
│  - Stdio JSON-RPC 2.0 Model Context Protocol (MCP)     │
│  - Hexagonal Ports & Adapters (StoragePort, PdfBackend)│
└────────────────────────────────────────────────────────┘
```

---

## 🔬 Tier-by-Tier Testing & Diagnostic Methodology

When an operation, user action, or test fails, **NEVER guess or apply random trial-and-error edits.** Follow this bottom-up diagnostic protocol to isolate the exact failing tier:

---

### TIER 1: MCP Protocol & Rust Core Engine

**Question:** *"Is the underlying PDF algorithm and JSON-RPC engine sound?"*

#### How to Test:
1. **Unit & Integration Tests:**
   ```bash
   cargo test --package paperpilot-core
   cargo test --package paperpilot-pdf
   cargo test --package paperpilot-mcp
   ```
2. **Direct JSON-RPC Stdio Call (Zero-Mocking):**
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"pdf_compress","arguments":{"path":"tests/e2e_fixtures/sample.pdf","quality":75}}}' | cargo run -p paperpilot-mcp
   ```

#### Common Tier 1 Root Causes:
- **Corrupted PDF Output / 0-byte write:** Check file descriptor flush in `paperpilot-pdf/src/`.
- **Malformed JSON-RPC Response:** `paperpilot-mcp/src/server.rs` must return `{"jsonrpc": "2.0", "id": ..., "result": { "content": [{ "type": "text", "text": "..." }] }}`.
- **Panic on Encrypted or Malformed Input:** Unhandled `lopdf::Error` without returning `OperationResult::Err`.

---

### TIER 2: Tauri 2.0 API & IPC Bridge

**Question:** *"Can the frontend talk to the Rust backend without permission or argument errors?"*

#### How to Test:
1. **Check Tauri Command Registration in `apps/desktop/src-tauri/src/lib.rs`:**
   Every command called via `invoke('command_name')` **MUST** be registered in `tauri::generate_handler![]`:
   ```rust
   tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![
           invoke_mcp_tool,
           open_pdf_dialog,
           // Ensure new commands are listed here!
       ])
   ```
2. **Check Tauri Security Capabilities:**
   Inspect `apps/desktop/src-tauri/capabilities/default.json`.
   If using file dialogs, openers, or path resolution, the corresponding permission must be present:
   ```json
   "permissions": [
     "core:default",
     "opener:default",
     "dialog:default",
     "fs:default"
   ]
   ```
3. **Payload Type Mismatch:**
   Ensure TypeScript argument keys match Rust argument names exactly:
   - TypeScript: `await invoke('invoke_mcp_tool', { tool: 'pdf_split', args: { path, ranges } })`
   - Rust: `pub fn invoke_mcp_tool(tool: String, args: serde_json::Value)`

#### Common Tier 2A Root Causes:
- **`command ... not found`:** Command is defined in Rust but missing from `generate_handler![]`.
- **`permission denied` / `plugin dialog not allowed`:** Capability missing from `capabilities/default.json`.
- **Webview sandbox block:** File path cannot be read directly by webview without `convertFileSrc` or native Tauri FS commands.

---

### TIER 2B: REST API & Remote MCP Gateway (Axum / HTTP)

**Question:** *"Are automation endpoints and remote MCP clients receiving properly serialized responses?"*

#### How to Test:
1. **Server Health & Docs Check:**
   ```bash
   curl -s http://127.0.0.1:7823/health
   # Must return {"status": "ok", "version": "...", "tools_count": 44}
   curl -s http://127.0.0.1:7823/docs
   ```
2. **Direct REST Multipart Execution:**
   ```bash
   curl -X POST http://127.0.0.1:7823/api/v1/pdf/info -F "file=@tests/e2e_fixtures/sample.pdf"
   ```
3. **Dynamic 44-Tool Router Execution (`/api/v1/pdf/tools/:tool_name`):**
   ```bash
   # Any of PaperPilot's 44 tools can be invoked directly by name:
   curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/rotate \
     -H "Content-Type: application/json" \
     -d '{"input": "sample.pdf", "angle": 90, "output": "rotated.pdf"}'

   curl -X POST http://127.0.0.1:7823/api/v1/pdf/tools/remove_blank \
     -H "Content-Type: application/json" \
     -d '{"input": "sample.pdf", "output": "clean.pdf", "sensitivity": 95}'
   ```
4. **Remote MCP SSE Handshake Check:**
   ```bash
   curl -N http://127.0.0.1:7823/mcp/sse
   ```

#### Common Tier 2B Root Causes:
- **`400 Bad Request` on `/api/v1/pdf/tools/:tool_name`:** Missing required schema parameter (e.g., omitting `angle` for `rotate` or `password` for `encrypt`). Check `paperpilot-mcp/src/server.rs` tool input schema.
- **`404 Not Found` on tool dispatch:** Tool name does not exist in the 44-tool registry. Tool names can be called with or without `pdf_` prefix (`rotate` or `pdf_rotate`).
- **`415 Unsupported Media Type` or `422 Unprocessable Entity`:** Multipart boundary header missing in request, or JSON body schema mismatch with the Axum extractor.
- **Port Conflict (`EADDRINUSE`):** Port 7823 occupied by previous process. Kill existing listener (`fuser -k 7823/tcp`) or run with `--port <NEW_PORT>`.
- **SSE Connection Drop:** Client disconnected before receiving session ID endpoint URI.

---

### TIER 3: Svelte 5 Desktop UI

**Question:** *"Are runes reactive, elements accessible, and visual styles correctly rendered?"*

#### How to Test:
1. **Static Type & Rune Checking:**
   ```bash
   npm --prefix apps/desktop run check
   ```
   *Requirement:* MUST report `0 errors and 0 warnings`.
2. **Component Vitest Suite:**
   ```bash
   npm --prefix apps/desktop test
   ```
3. **Autonomous Browser Testing (Agent Browser / Playwright):**
   ```bash
   npx playwright test
   # OR autonomous exploration:
   npx agent-browser open "http://localhost:5173"
   ```

#### Common Tier 3 Root Causes:
- **Button Click Does Nothing:**
  - Used legacy Svelte 4 syntax (`on:click={...}`) instead of Svelte 5 (`onclick={...}`).
- **State Changes Don't Update DOM:**
  - Mutated a non-reactive variable or forgot `$state()` / `$derived()`.
- **Invisible Text / Low Contrast in Dark Mode:**
  - Native browser `<select>` or `<input>` controls falling back to OS light mode styles.
  - Fix: Add `color-scheme: dark; -webkit-appearance: none;` and explicit `background-color: var(--bg-surface); color: var(--text-primary);`.
- **Overlapping Elements:**
  - Fixed `position: absolute` without proper container relative bounds or colliding z-indexes.

---

## 🚦 Autonomous Agent Diagnostic Decision Tree (For Jules)

When Jules encounters a failure during automated testing or exploration, Jules must follow this flowchart:

```
[TEST / ACTION FAILS]
          │
          ▼
Does `cargo test --workspace` pass?
   ├── ❌ NO  ──► TIER 1 BUG: Fix Rust code in `paperpilot-pdf` or `paperpilot-mcp`.
   │
   └── ✅ YES
          │
          ▼
Does `npm run check` pass with 0 errors?
   ├── ❌ NO  ──► TIER 3 SYNTAX/TYPE BUG: Fix Svelte 5 runes or TypeScript types.
   │
   └── ✅ YES
          │
          ▼
Does the browser console show an IPC rejection / permission error?
   ├── ❌ YES ──► TIER 2A IPC BUG: Check `generate_handler![]` or `capabilities/default.json`.
   │
   └── ❌ NO
          │
          ▼
Did a REST API or Remote MCP test fail (e.g. curl /api/v1/...)?
   ├── ❌ YES ──► TIER 2B GATEWAY BUG: Check Axum route, multipart parser, or SSE event formatting.
   │
   └── ❌ NO  (Silent failure or visual glitch)
          │
          ▼
   TIER 3 UI/UX BUG:
   - Check click handlers (`onclick`)
   - Check CSS contrast (`color-scheme: dark`, CSS tokens)
   - Check DOM event propagation (`stopPropagation`)
```

---

## 📋 Rules for Autonomous QA Submissions

1. **Root-Cause Isolation Required:** When submitting a PR fixing a bug, state which Tier caused the failure:
   - Example: *"Fixes Tier 2A IPC argument mismatch in `pdf_split` handler"*
   - Example: *"Fixes Tier 2B Axum multipart boundary extraction in `pdf_merge` endpoint"*
   - Example: *"Fixes Tier 3 dark mode contrast in `.zoom-select`"*
2. **Zero-Mocking Policy:** Never mock Rust MCP output with fake static fixtures in production UI code. Always test against actual engine outputs or valid test fixtures.
3. **All Tiers Verified:** Every submitted PR must pass:
   - Tier 1: `cargo test --workspace`
   - Tier 2A/2B: `cargo check --workspace` & REST/MCP integration tests
   - Tier 3: `npm --prefix apps/desktop run check` and `npm --prefix apps/desktop test`
