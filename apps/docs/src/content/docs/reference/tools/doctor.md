---
title: "doctor"
description: "Run environment and system health diagnostics across filesystem, OCR models, disk space, and REST gateway."
sidebar:
  order: 91
---

### What it does

`paperpilot doctor` inspects the local runtime environment to ensure PaperPilot is properly configured and operational. It checks:
- OS Temporary directory (`/tmp` or `%LOCALAPPDATA%\Temp`) write, read, and ephemeral deletion permissions.
- Current working directory permissions.
- REST API Gateway port 7823 availability.
- Local OCR and vision neural model weights presence (`models/ocr/ppocrv4` or `PAPERPILOT_OCR_MODELS_DIR`).
- Scratch filesystem disk space availability (ensuring sufficient free space for large PDF rendering).

It can emit an ANSI-colored summary report for terminal inspection or a structured JSON report via `--json`.

### CLI

```bash
# Human-readable checklist
paperpilot doctor

# Structured JSON output
paperpilot doctor --json
```

### MCP JSON-RPC

```json
{
  "name": "doctor",
  "arguments": {}
}
```

### REST API

```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/doctor
```
