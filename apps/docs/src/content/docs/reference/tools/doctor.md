---
title: "Doctor"
description: "Run system diagnostics to check offline dependencies."
sidebar:
  order: 99
---

# System Diagnostics (Doctor)

<span class="status-badge stable">Stable</span>

### What it does

Runs a suite of diagnostic probes across the system to ensure PaperPilot is ready to execute tasks locally and offline. It verifies:
- Temporary directory is writable and files can be cleanly deleted.
- Local OCR models (`det.onnx`, `rec.onnx`) are correctly downloaded and located.
- The default local REST API gateway port (7823) is bindable or available.
- The system has sufficient available disk space (> 5 GB).

### CLI

```bash
# Run diagnostics and print colored check output
paperpilot doctor

# Output raw JSON diagnostics report
paperpilot doctor --json
```

### MCP JSON-RPC

Currently, `doctor` is designed primarily as an environment verification tool and does not expose a specific MCP action wrapper, but can be invoked via system tool execution.

### REST API

```bash
# Run system diagnostics
curl -s -X GET http://127.0.0.1:7823/api/v1/doctor
```
