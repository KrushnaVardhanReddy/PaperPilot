# Spec 139: Headless Hot Folder Watcher Daemon (`paperpilot watch`)

**Status:** Draft / Active  
**Phase:** Phase 4.7.1 (Automation Engine)  
**Parent:** [Phase 4 — AI & Automation Layer](../../phases/phase-04-ai-layer.md)  
**Related Specs:**
- [Spec 136: Conditional Workflow Engine](../136-conditional-pipeline-execution-engine/spec.md)
- [Spec 137: Advanced Multi-Condition Engine](../137-advanced-multi-condition-engine/spec.md)
- [Spec 135: Batch AcroForm Mail Merge](../135-cli-acroform-batch-mail-merge/spec.md)
- [Spec 140: Desktop V2 Workspace UX Refresh](../140-desktop-v2-workspace-ux-refresh/spec.md) *(Watched Folders sidebar)*

---

## 0. Interface Coverage Mandate

PaperPilot's **Single Core Runner + 5 Wrappers** rule requires every capability to be exposed across all five interfaces. The Hot Folder Watcher core logic (`WatcherEngine`) must live in `paperpilot-pdf` / `paperpilot-core` and be consumed by thin wrappers:

| Interface | Wrapper | Coverage in this Spec |
|---|---|---|
| **CLI** | `paperpilot-cli` — `paperpilot watch --dir … --recipe …` | ✅ Section 3 |
| **REST Gateway** | `paperpilot-gateway` — `POST /v1/automation/watchers` | ✅ Section 7 |
| **MCP** | `paperpilot-mcp` — `watcher_start`, `watcher_stop`, `watcher_list`, `recipe_run` | ✅ Section 8 |
| **Desktop UI** | Watched Folders sidebar in `LibraryPanel.svelte` (Spec 140) | ✅ via Spec 140 |
| **WASM** | **Exempt** — filesystem watch cannot execute in a browser sandbox | N/A |

---

## 1. Overview & Problem Statement

Users and automated systems (such as physical network scanners, automated export scripts, or desktop users dropping receipts into an `Inbox`) need PaperPilot to automatically process documents without manual CLI invocations or GUI interaction.

While PaperPilot possesses a unified, declarative pipeline execution engine (`.ppflow` recipes via `paperpilot run`), the CLI watcher (`paperpilot-cli/src/commands/watch.rs`) is currently incomplete:
1. It does not execute `.ppflow` recipes using the unified workflow runner (`paperpilot_pdf::operations::workflow::run_workflow`).
2. It lacks initial directory sweep on startup (any PDFs already present in the folder are ignored).
3. It does not handle error quarantine (`.error/` directory) with persistent JSON execution receipts.
4. CLI flag syntax is inconsistent (`paperpilot watch <DIR>` vs `--dir <DIR>`, `--recipe <RECIPE.ppflow>`).
5. Ingress debounce safety needs reliable file settling verification before attempting byte reads.

Spec 139 formalizes the production architecture for the headless Hot Folder Watcher daemon.

---

## 2. Goals & Non-Goals

### Goals
- **Unified Recipe Execution**: Directly invoke `paperpilot_pdf::operations::workflow::run_workflow` when `--recipe <PATH>` is supplied.
- **Initial Sweep & In-Order FIFO Queue**: On daemon startup, detect and process all existing `.pdf` files sequentially before listening to new filesystem events.
- **Debounced Settling**: Guard against reading partial/half-written files (e.g. slow network copies, browser downloads, scanner writes) with a configurable `--settle-delay-ms` (default 500ms).
- **Output Routing & Archiving**:
  - `--output <DIR>`: Write the resulting processed PDF into the destination directory.
  - `--move-original <DIR>`: Optionally move the ingress PDF to an archive directory upon success.
- **Error Quarantine & Audit Receipts**:
  - If processing fails (corrupt PDF, missing password, recipe step error), move the ingress file to `<WATCH_DIR>/.error/` (or `--error-dir <DIR>`).
  - Write `<STEM>_receipt.json` alongside the output or error file detailing timestamp, file hash, duration, and error messages.
- **Graceful Termination**: Clean shutdown on `SIGINT` / `SIGTERM` (Ctrl+C), allowing in-flight file processing to complete.

### Non-Goals
- Cloud / remote S3 bucket watching (local filesystem only for Community tier).
- GUI configuration dashboard (reserved for Phase 4.7.8 Desktop Automations Tab).

---

## 3. CLI Command Interface

```bash
paperpilot watch [OPTIONS] --dir <WATCH_DIR>
```

### Arguments & Flags:
- `-d, --dir <PATH>`: Directory to monitor for ingress `.pdf` files.
- `-r, --recipe <PATH>`: Path to `.ppflow` TOML or JSON workflow recipe.
- `-o, --output <PATH>`: Directory where successfully processed PDFs are saved. (Defaults to `<WATCH_DIR>/processed` or direct in-place naming).
- `--move-original <PATH>`: Directory to move the original source PDF after successful recipe execution.
- `--error-dir <PATH>`: Directory to quarantine failed files (defaults to `<WATCH_DIR>/.error`).
- `--settle-delay-ms <MS>`: Time in milliseconds of filesystem inactivity before a file is considered ready for reading (default: `500`).
- `--webhook <URL>`: Optional webhook URL to send execution receipt payload on completion.
- `--json`: Emit JSON-formatted logs to stdout for process supervisor integration.

---

## 4. Architecture & Execution Flow

```mermaid
flowchart TD
    Start([Daemon Starts: paperpilot watch]) --> InitialSweep[Scan directory for existing *.pdf]
    InitialSweep --> Queue[FIFO Processing Queue]
    
    Watcher[notify-rs Filesystem Watcher] -- Create / Modify --> Debounce[Debounce Buffer]
    Debounce -- Inactive for settle_delay_ms --> Queue
    
    Queue --> ProcessNext[Pop Next PDF File]
    ProcessNext --> RunRecipe[Execute Recipe via run_workflow]
    
    RunRecipe -->|Success| SaveOutput[Save Output PDF to output_dir]
    SaveOutput --> MoveOriginal{--move-original?}
    MoveOriginal -->|Yes| Archive[Move original to move_original dir]
    MoveOriginal -->|No| Keep[Leave or delete as configured]
    Archive --> WriteReceipt[Write execution_receipt.json]
    Keep --> WriteReceipt
    
    RunRecipe -->|Failure| Quarantine[Move to .error/ quarantine dir]
    Quarantine --> WriteErrorReceipt[Write failure receipt.json]
    
    WriteReceipt --> Webhook{--webhook?}
    WriteErrorReceipt --> Webhook
    Webhook -->|Yes| PostWebhook[POST JSON payload]
    Webhook -->|No| Next[Wait for next queue item]
    PostWebhook --> Next
```

---

## 5. Failure Handling & Quarantine Policy

1. **Unreadable / Locked Files**: If a file cannot be opened due to OS file lock, it re-enters the debounce buffer with exponential backoff up to 5 attempts.
2. **Corrupted / Invalid PDFs**: If `run_workflow` returns `PdfError`, the file is quarantined into `.error/` with an accompanying `.error.json` description. The daemon logs the incident and continues with the next queued file.
3. **No Halting**: A single bad file must NEVER crash or terminate the watcher daemon.

---

## 6. Verification & Test Plan

1. **Unit & Integration Tests (`tests/test_watch.rs`)**:
   - `test_watch_initial_sweep`: Verify existing PDFs in folder are processed immediately on start.
   - `test_watch_live_ingress`: Drop a new PDF while daemon is active, assert output generated.
   - `test_watch_recipe_execution`: Verify a multi-step recipe (e.g. compress + watermark) executes cleanly.
   - `test_watch_quarantine_error`: Drop a malformed byte file, verify quarantine into `.error/` and receipt generated.
   - `test_watch_debounce`: Rapid append writes simulate slow transfer; verify single execution only after settle delay.
2. **Resource Constraints**:
   - All tests compile with `-j 2`.
   - Zero mock data.

---

## 7. REST Gateway Interface (`paperpilot-gateway`)

Add automation routes to `paperpilot-gateway/src/handlers.rs` and register them in `server.rs`.

### Endpoints

#### `POST /v1/automation/watchers` — Start a Watcher

**Request body (JSON)**:
```json
{
  "dir": "/home/user/Inbox",
  "recipe": "/home/user/recipes/ocr_archive.ppflow",
  "output_dir": "/home/user/Archive",
  "error_dir": "/home/user/Inbox/.error",
  "settle_delay_ms": 500,
  "move_original": "/home/user/Inbox/processed"
}
```
**Response (201 Created)**:
```json
{
  "watcher_id": "watcher_a1b2c3d4",
  "status": "watching",
  "dir": "/home/user/Inbox",
  "recipe": "ocr_archive.ppflow"
}
```

#### `GET /v1/automation/watchers` — List Active Watchers

**Response (200 OK)**:
```json
{
  "watchers": [
    {
      "watcher_id": "watcher_a1b2c3d4",
      "status": "watching",
      "dir": "/home/user/Inbox",
      "recipe": "ocr_archive.ppflow",
      "last_processed_file": "invoice_oct.pdf",
      "last_processed_at": "2026-10-28T17:00:00Z",
      "total_processed": 42
    }
  ]
}
```

#### `DELETE /v1/automation/watchers/{watcher_id}` — Stop a Watcher

**Response (204 No Content)** on success.

#### `POST /v1/automation/run` — Run a Recipe Against an Uploaded File

This is the stateless one-shot equivalent — does NOT start a daemon, just runs a recipe on a single file immediately (useful for CI/testing pipelines).

**Request**: `multipart/form-data` with fields:
- `file`: the input PDF bytes.
- `recipe`: the `.ppflow` TOML text inline, OR `recipe_path` pointing to a server-local recipe file.

**Response (200 OK)**:
```json
{
  "success": true,
  "output_bytes": "<base64 encoded output PDF>",
  "receipt": {
    "job_id": "run_xyz123",
    "duration_ms": 342,
    "stages": [
      { "tool": "pdf_ocr", "success": true, "duration_ms": 280 },
      { "tool": "pdf_compress", "success": true, "duration_ms": 62 }
    ]
  }
}
```

### Gateway State: Watcher Registry

Gateway maintains an in-memory `WatcherRegistry` (Tokio `Mutex<HashMap<String, WatcherHandle>>`) mapping `watcher_id → (JoinHandle, CancellationToken)`. This allows:
- `DELETE` to cleanly cancel the async task via `CancellationToken::cancel()`.
- `GET` to query running watcher state without filesystem polling.

> ⚠️ **Important**: Watcher registry is in-memory only. It does not persist across gateway restarts. Persistence (SQLite-backed registry) is a future task.

---

## 8. MCP Interface (`paperpilot-mcp`)

Add 4 MCP tools so AI agents can orchestrate document automation. These absorb **Task 4.7.9** into this spec.

### Tool: `watcher_start`

```json
{
  "name": "watcher_start",
  "description": "Start a headless hot folder watcher daemon on a local directory. When a PDF is dropped into the watched directory it automatically runs the specified recipe.",
  "inputSchema": {
    "type": "object",
    "required": ["dir", "recipe"],
    "properties": {
      "dir": { "type": "string", "description": "Absolute path to the directory to watch" },
      "recipe": { "type": "string", "description": "Absolute path to a .ppflow TOML recipe file" },
      "output_dir": { "type": "string", "description": "Directory to write processed PDFs (optional)" },
      "settle_delay_ms": { "type": "integer", "default": 500 }
    }
  }
}
```

**Returns**: `{ watcher_id: string, status: "watching" }`

### Tool: `watcher_stop`

```json
{
  "name": "watcher_stop",
  "description": "Stop a running hot folder watcher by its ID.",
  "inputSchema": {
    "type": "object",
    "required": ["watcher_id"],
    "properties": {
      "watcher_id": { "type": "string" }
    }
  }
}
```

### Tool: `watcher_list`

```json
{
  "name": "watcher_list",
  "description": "List all currently active hot folder watchers with their status and statistics.",
  "inputSchema": { "type": "object", "properties": {} }
}
```

### Tool: `recipe_run`

```json
{
  "name": "recipe_run",
  "description": "Execute a .ppflow recipe against a local PDF file immediately (one-shot, no daemon).",
  "inputSchema": {
    "type": "object",
    "required": ["input", "recipe"],
    "properties": {
      "input": { "type": "string", "description": "Absolute path to input PDF" },
      "recipe": { "type": "string", "description": "Absolute path to .ppflow TOML recipe file" },
      "output": { "type": "string", "description": "Absolute path for output PDF" }
    }
  }
}
```

### MCP Shared State

Because the MCP server and the REST gateway share the same process when running in `paperpilot serve` mode, they MUST share the same `WatcherRegistry` instance via `Arc<Mutex<WatcherRegistry>>`injected at server startup.

---

## 9. Updated Verification & Test Plan

### REST Gateway Tests (`paperpilot-gateway/tests/test_automation.rs`)
- `test_post_watcher_start`: POST valid body, assert `201`, watcher_id returned.
- `test_get_watcher_list`: Start a watcher, GET list, assert it appears with `status: watching`.
- `test_delete_watcher_stop`: Start a watcher, DELETE it, assert `204`, watcher no longer in list.
- `test_post_automation_run`: Upload a real PDF + inline recipe, assert output bytes returned + receipt JSON correct.

### MCP Tests (`paperpilot-mcp/tests/test_automation_tools.rs`)
- `test_mcp_watcher_start_stop`: Call `watcher_start`, verify tool returns `watcher_id`. Call `watcher_stop`, verify success.
- `test_mcp_watcher_list`: Start two watchers, call `watcher_list`, assert both appear.
- `test_mcp_recipe_run`: Call `recipe_run` with real fixture PDF and a 2-step recipe, assert output PDF is written and receipt valid.
