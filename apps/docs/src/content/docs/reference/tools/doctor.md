---
title: "doctor"
description: "Reference, CLI, MCP, and REST API commands for doctor."
sidebar:
  order: 99
---

**Status**: ✅ Verified  |  **Format**: System / Diagnostics

### What it does
**Runs a comprehensive suite of diagnostic probes across the system to ensure PaperPilot is ready to execute tasks locally and offline.**  
Probes local dependencies, filesystem permissions, network ports, on-device neural model weights, and storage limits to verify cleanroom offline-readiness:
- **Ephemeral Scratch Volume**: Verifies `/tmp` (or configured temp directory) is readable, writable, and supports clean ephemeral deletion (`paperpilot_probe.tmp`).
- **Gateway Port Availability**: Checks whether the PaperPilot REST/MCP Gateway port (`127.0.0.1:7823`) is available or actively listening and healthy.
- **On-Device Neural OCR Models**: Confirms presence and integrity of ONNX detection and recognition models (`models/ocr/ppocrv4/det.onnx`, `rec.onnx`, or custom path via `PAPERPILOT_OCR_MODELS_DIR`).
- **Disk Storage Capacity**: Verifies scratch storage has at least 5 GB of free space for zero-leak document processing.

---

### CLI
Run diagnostics directly from the command line:

#### 1. Human-Readable Terminal Diagnostic
```bash
paperpilot doctor
```

##### Example Terminal Output
```text
PaperPilot System Diagnostics (v0.1.0)
==================================================
Status:    Healthy
Timestamp: 1794844683

[✓] Temp Directory: /tmp (writable & ephemeral cleanup verified)
[✓] Gateway Port: 7823 available
[✓] OCR Models: models/ocr/ppocrv4 (det.onnx, rec.onnx found)
[✓] Disk Space: > 5 GB available (19 GB found)

Result: All critical subsystems are operational.
```

#### 2. Machine-Readable JSON Output
```bash
paperpilot doctor --json
```

##### Example JSON Output
```json
{
  "timestamp": "1794844680",
  "status": "Healthy",
  "checks": [
    {
      "name": "Temp Directory: /tmp (writable & ephemeral cleanup verified)",
      "passed": true,
      "message": "Temp directory is writable and files can be cleanly deleted.",
      "suggestion": null
    },
    {
      "name": "Gateway Port: 7823 available",
      "passed": true,
      "message": "Port 7823 is open and ready.",
      "suggestion": null
    },
    {
      "name": "OCR Models: models/ocr/ppocrv4 (det.onnx, rec.onnx found)",
      "passed": true,
      "message": "OCR models verified at models/ocr/ppocrv4.",
      "suggestion": null
    },
    {
      "name": "Disk Space: > 5 GB available",
      "passed": true,
      "message": "> 5 GB available (19 GB found)",
      "suggestion": null
    }
  ]
}
```

#### CLI Parameters
- `--json` *(flag, optional)*: Emit report in machine-readable JSON format.
- `--log-format` *(string, optional)*: Output format for logs (`text`, `json`, `compact`, `pretty`).
- `--log-level` *(string, optional)*: Log level verbosity (`trace`, `debug`, `info`, `warn`, `error`).
- `--webhook` *(string, optional)*: Webhook URL to deliver diagnostic reports.

---

### REST API
Trigger system diagnostics through PaperPilot Gateway:

```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/doctor
```

#### Response (`HTTP 200 OK`)
```json
{
  "timestamp": "1794844680",
  "status": "Healthy",
  "checks": [
    {
      "name": "Temp Directory: /tmp (writable & ephemeral cleanup verified)",
      "passed": true,
      "message": "Temp directory is writable and files can be cleanly deleted.",
      "suggestion": null
    },
    {
      "name": "Gateway Port: 7823 available",
      "passed": true,
      "message": "Port 7823 is open and ready.",
      "suggestion": null
    },
    {
      "name": "OCR Models: models/ocr/ppocrv4 (det.onnx, rec.onnx found)",
      "passed": true,
      "message": "OCR models verified at models/ocr/ppocrv4.",
      "suggestion": null
    },
    {
      "name": "Disk Space: > 5 GB available",
      "passed": true,
      "message": "> 5 GB available (19 GB found)",
      "suggestion": null
    }
  ]
}
```

---

### MCP JSON-RPC
Diagnostics are an administrative host environment inspection utility and are accessed via CLI, REST API (`/api/v1/doctor`), or the Desktop UI status dashboard.

---

### Known Limitations & Troubleshooting
- **Degraded Status**: When an optional or non-fatal check fails (such as missing optional OCR ONNX model weights or an occupied local port), `doctor` returns a `"Degraded"` status rather than hard failure, reporting actionable remediation steps in the `suggestion` field.
- **Port Conflict**: If another local process holds port `7823`, `doctor` reports `Gateway Port: 7823 available [✗]` with a suggestion to restart the service or terminate conflicting background daemons.
- **WASM Support**: System-level probes (disk space, temp directories, network listening ports) rely on native operating system APIs and are not available in sandboxed browser WASM runtimes.
