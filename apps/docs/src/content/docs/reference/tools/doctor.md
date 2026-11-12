---
title: "doctor"
description: "Reference, CLI, MCP, and REST API commands for doctor."
sidebar:
  order: 99
---

**Status**: ✅ Verified  |  **Format**: System / Diagnostics

### What it does
Runs a comprehensive suite of diagnostic probes across the system to ensure PaperPilot is ready to execute tasks locally and offline:
- Ephemeral temporary directory read, write, and deletion verification (`paperpilot_probe.tmp`).
- Gateway port availability and status (`127.0.0.1:7823`).
- On-device neural OCR model weights presence (`models/ocr/ppocrv4` or `PAPERPILOT_OCR_MODELS_DIR`).
- Free disk storage on the scratch volume (> 5 GB check).

### CLI
```bash
# Human-readable terminal output with status checkmarks
paperpilot doctor

# Machine-readable JSON output
paperpilot doctor --json
```

### MCP JSON-RPC
`doctor` is an environment and diagnostics inspection command and is available via CLI, REST, and Desktop UI.

### REST API
```bash
# Run system diagnostics via local Gateway
curl -s -X GET http://127.0.0.1:7823/api/v1/doctor
```
