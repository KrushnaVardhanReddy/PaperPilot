---
title: "watcher_start"
description: "Reference, CLI, MCP, REST API, and Desktop specifications for watcher_start."
sidebar:
  order: 91
---

**Status**: ✅ Verified  |  **Format**: Automation / Hot Folder

### What it does
**Registers an operating-system folder watcher, processing arriving PDFs with validated recipes and durable audit receipts.**  
Monitors a local filesystem directory for dropped or newly written PDF documents. Arriving files are debounced until file sizes stabilize, executed through declarative multi-step recipes or presets, and recorded with cryptographic SHA-256 receipts.

---

### Key Capabilities
- **Pre-Registration Validation**: Rejects invalid paths, non-existent directories, and unsupported recipes prior to registration.
- **Settle Delay Stabilization**: Debounces filesystem writes with configurable delay (`settle_delay_ms`, 50–60,000ms, default: 500ms).
- **Execution Receipts**: Writes `.receipt.json` audit files containing input/output SHA-256 hashes, execution timing, and workflow stage statuses.
- **Collision Protection**: Existing output files are never overwritten; automatic numeric suffixes are applied.
- **Error Quarantine**: Processing failures retain the source document and write detailed diagnostic failure receipts into the error directory (`.error`).

---

### CLI
Start or test folder watching from the command line:

#### 1. Continuous Watch Daemon
```bash
paperpilot watch \
  --directory tests/e2e_fixtures/out/batch18/watch_in \
  --output tests/e2e_fixtures/out/batch18/watch_out \
  --recipe preset-legal-discovery \
  --settle-delay-ms 500
```

#### 2. One-Shot Batch Sweep (`--once`)
Process all existing PDFs currently residing in the directory and exit immediately:
```bash
paperpilot watch \
  --directory tests/e2e_fixtures/out/batch18/watch_in \
  --output tests/e2e_fixtures/out/batch18/watch_out \
  --recipe preset-legal-discovery \
  --once
```

#### 3. Custom Output Template
```bash
paperpilot watch \
  --directory ./inbox \
  --output ./processed \
  --recipe ./prep_pipeline.ppflow \
  --name-template "{input_stem}-{date:%Y%m%d}-{hash:8}-{index:04}.pdf"
```

#### CLI Parameters
- `-d, --directory <DIRECTORY>` *(string, required)*: Directory to monitor for incoming PDF files.
- `-r, --recipe <RECIPE>` *(string, optional)*: Path to `.ppflow` TOML/JSON recipe or preset ID (e.g. `preset-legal-discovery`).
- `-o, --output <OUTPUT>` *(string, optional)*: Output directory for successfully processed PDFs.
- `--name-template <NAME_TEMPLATE>` *(string, optional)*: Custom output basename template with variable tokens.
- `--move-original <MOVE_ORIGINAL>` *(string, optional)*: Directory to move processed source files.
- `--error-dir <ERROR_DIR>` *(string, optional, default: `<dir>/.error`)*: Directory for failure receipts.
- `--settle-delay-ms <SETTLE_DELAY_MS>` *(integer, optional, default: `500`)*: Milliseconds of filesystem inactivity before processing.
- `--once` *(flag, optional)*: Process existing files and exit cleanly.
- `--webhook <WEBHOOK>` *(string, optional)*: Webhook URL to deliver completion notifications and receipt paths.

---

### MCP JSON-RPC
Register and manage folder watchers via Model Context Protocol (MCP) clients:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "watcher_start",
    "arguments": {
      "dir": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in",
      "output_dir": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out",
      "recipe": "preset-legal-discovery",
      "settle_delay_ms": 500
    }
  },
  "id": 1
}
```

#### MCP Output Response
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"watcher_id\":\"watcher_c9b8f2d87e1445bfbb5bcf112a912803\",\"status\":\"watching\",\"watcher\":{\"id\":\"watcher_c9b8f2d87e1445bfbb5bcf112a912803\",\"directory\":\"/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in\",\"recipe_name\":\"preset-legal-discovery\",\"status\":\"watching\",\"succeeded\":0,\"failed\":0,\"last_processed_file\":null,\"last_processed_at\":null,\"last_error\":null,\"last_output\":null,\"last_receipt\":null}}"
      }
    ],
    "isError": false
  }
}
```

#### MCP Schema Parameters
- `dir` *(string, required)*: Absolute path to the directory to watch.
- `recipe` *(string, required)*: Local `.ppflow` recipe path, preset ID, or serialized workflow.
- `output_dir` *(string, optional)*: Output directory (defaults to a separate processed subfolder).
- `name_template` *(string, optional)*: Output basename template using dynamic tokens.
- `settle_delay_ms` *(integer, optional, default: `500`)*: Inactivity delay before processing (50–60000 ms).

---

### REST API
Register a watcher through the PaperPilot Gateway:

```bash
curl -s -X POST http://127.0.0.1:7823/v1/automation/watchers \
  -H 'Content-Type: application/json' \
  -d '{
    "dir": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in",
    "output_dir": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_out",
    "recipe": "preset-legal-discovery",
    "settle_delay_ms": 500,
    "name_template": "{input_stem}-{index:04}.pdf"
  }'
```

#### Response (`HTTP 201 Created`)
```json
{
  "watcher_id": "watcher_7f2b189c44",
  "status": "watching",
  "directory": "/home/krushna/Project/PaperPilot/tests/e2e_fixtures/out/batch18/watch_in",
  "recipe": "preset-legal-discovery"
}
```

---

### Known Limitations & Troubleshooting
- **Process Boundary**: Watchers operate inside the hosting process. The Desktop UI, its embedded REST gateway, and MCP runtime share an in-memory `WatcherManager`; standalone CLI invocations operate in their own process.
- **Non-Recursive Monitoring**: Watcher scopes monitor the exact folder specified; subdirectories within the inbox are not traversed recursively.
- **Directory Separation**: The input monitored directory and the designated output directory must be distinct paths to prevent recursive trigger loops.
- **WASM Support**: Filesystem watcher daemons require OS filesystem event notification primitives (`notify`) and multi-threading; they are not available in browser WASM environments.
