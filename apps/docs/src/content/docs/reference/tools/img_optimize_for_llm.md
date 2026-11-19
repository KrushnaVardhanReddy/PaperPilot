---
title: "img_optimize_for_llm"
description: "Reference, CLI, MCP, and REST API commands for img_optimize_for_llm."
sidebar:
  order: 69
---

**Status**: ✅ Verified  |  **Format**: Image (`.png`, `.jpeg`, `.webp`, `.tiff`) to LLM-Optimized Image

### What it does
**Downscales and sharpens images for Vision LLM token budget economy and OCR legibility.**  
Scales oversized high-DPI document scans using Lanczos3 interpolation down to target dimension thresholds (e.g., 1024px) to drastically reduce vision token consumption while applying a subtle unsharp high-pass contrast filter that boosts text edge clarity and character recognition accuracy for models like GPT-4o, Claude 3.5 Sonnet, and Gemini 1.5 Pro.

---

### CLI
Optimize a document image for multimodal LLMs via the PaperPilot CLI `img-optimize-for-llm` subcommand:

```bash
paperpilot img-optimize-for-llm \
  --input tests/e2e_fixtures/real/highres_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/llm_optimized.png \
  --max-dim 1024 \
  --quality 85 \
  --json
```

#### CLI Parameters
- `--input` *(string, required)*: Path to the input image file.
- `--output` *(string, required)*: Destination file path for the optimized output image.
- `--max-dim` *(integer, optional, default: `1024`)*: Maximum width or height dimension in pixels. Aspect ratio is preserved.
- `--quality` *(integer, optional, default: `85`)*: Image compression quality (`1`–`100`).
- `--json` *(flag, optional)*: Emit structured JSON execution result.

---

### MCP JSON-RPC
When integrating via Model Context Protocol (MCP), invoke the tool `img_optimize_for_llm`:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_optimize_for_llm",
    "arguments": {
      "input": "tests/e2e_fixtures/real/highres_scan.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/llm_optimized.png",
      "max_dim": 1024,
      "quality": 85
    }
  },
  "id": 1
}
```

#### MCP Parameters
- `input` *(string, required)*: Path to the input image.
- `output` *(string, required)*: Destination path for the optimized image.
- `max_dim` *(number, optional, default: `1024`)*: Maximum width/height pixel boundary.
- `quality` *(number, optional, default: `85`)*: Compression quality factor.
- `enhance_text` *(boolean, optional, default: `true`)*: Whether to apply edge sharpening filter to preserve text legibility at lower resolutions.

---

### REST API
Invoke the REST API endpoint `/api/v1/pdf/img-optimize-for-llm`:

```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-optimize-for-llm \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/highres_scan.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/llm_optimized.png",
    "max_dim": 1024,
    "quality": 85
  }'
```

#### Response Example
```json
{
  "success": true,
  "request_id": "62378a67-4444-5555-6666-777788889999",
  "message": "Optimize Image for Vision LLMs executed successfully",
  "output": {
    "isError": false,
    "resultType": "complete",
    "content": [
      {
        "type": "text",
        "text": "{\"success\":true,\"message\":\"Optimize Image for Vision LLMs executed successfully\",\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/llm_optimized.png\",\"data\":{\"output_path\":\"tests/e2e_fixtures/out/penta_e2e/llm_optimized.png\"}}"
      }
    ]
  }
}
```

---

### Real-World Use Cases
- **Multimodal AI Agent Ingestion:** Reduce vision token overhead by 60–80% when sending scanned documents or UI screenshots to LLM context windows without degrading transcription accuracy.
- **RAG Preprocessing:** Pre-process document image slices and diagrams before embedding or indexing in visual retrieval pipelines.
- **Mobile Network Optimization:** Shrink high-resolution mobile camera uploads prior to transmitting them to inference endpoints.
