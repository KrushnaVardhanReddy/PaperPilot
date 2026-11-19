---
title: PaperPilot Tools Handbook
description: Comprehensive reference for all 100 PaperPilot tools across CLI, MCP, and REST API interfaces.
---

# PaperPilot Tools Handbook (100 Tools)

This handbook provides verified execution snippets and parameter references for all **100 PaperPilot tools** across the CLI, MCP (AI Agent JSON-RPC), and local REST API interfaces.

## 1. `batch_fill`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**High-throughput AcroForm mail-merge engine for automated document generation with audit receipts.**  
Fills a local template PDF once per CSV/JSON record or inline JSON record array. Generates independent PDF files and writes a cryptographic `execution_receipt.json` containing SHA-256 hashes, timestamps, and row-level success/error reports. Flattens form appearances by default to prevent downstream tampering.

---

### CLI
```bash
paperpilot fill-batch \
  --template tests/e2e_fixtures/real/form.pdf \
  --data tests/e2e_fixtures/real/batch_data.csv \
  --output-dir tests/e2e_fixtures/out/batch9_test/batch_out \
  --name-template "{index:04}.pdf" \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"data\":\"tests/e2e_fixtures/real/batch_data.csv\",\"data_sha256\":\"f54db1a3d0d0ddd838ce6cacdfece9c4f5f5f03dc290d8e429adafc30f569a14\",\"data_source\":\"file\",\"date\":\"2026-11-16\",\"execution_time_ms\":9,\"failed\":0,\"job_id\":\"fill-batch-1794843815207495-95\",\"operations_applied\":[\"pdf_fill_form\",\"pdf_flatten\"],\"receipt_path\":\"/tests/e2e_fixtures/out/batch9_test/mcp_batch_out/execution_receipt.json\",\"results\":[...],\"started_at\":\"2026-11-16T15:43:35.207495714+00:00\",\"succeeded\":4,\"success\":true,\"template\":\"tests/e2e_fixtures/real/form.pdf\",\"template_sha256\":\"25316f96...\",\"total\":4}"
      }
    ],
    "isError": false
  }
}
```

### REST API
```bash
N/A
```

## 2. `cli_ask`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Executes PDF operations using natural language prompts.**  
Interprets plain English instructions (such as `"compress aggressively"`, `"rotate 90 degrees"`, or `"extract text"`) and resolves them into deterministic execution plans and operations using a fast, privacy-first offline NLP resolver.

- **Offline Intent Resolution**: Operates entirely locally with zero cloud dependencies or external API calls.
- **Deterministic Action Routing**: Resolves ambiguous conversational prompts into precise structured tool invocations (e.g., `pdf_compress`, `pdf_rotate`, `pdf_extract_text`).
- **Interactive or Direct Automation**: Supports interactive confirmation, `--yes` / `-y` flag for automated headless pipelines, and `--dry-run` to output execution plans without modifying documents.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "name": "cli_ask",
  "arguments": {
    "prompt": "compress this document aggressively",
    "input": "input.pdf",
    "output": "compressed.pdf"
  }
}
```

### REST API
```bash
N/A
```

## 3. `cli_fill_batch`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Command-line batch mail-merge engine for generating populated, flattened AcroForm PDFs with audit receipts.**  
Parses the input PDF template once, generates populated PDF documents for every record in a CSV or JSON dataset, and saves an `execution_receipt.json` audit manifest containing row outcomes, execution durations, and SHA-256 digests. Powered by the same underlying core engine as [`batch_fill`](/reference/tools/batch_fill/).

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "cli_fill_batch",
    "arguments": {
      "template": "tests/e2e_fixtures/real/form.pdf",
      "data": "tests/e2e_fixtures/real/batch_data.csv",
      "output_dir": "tests/e2e_fixtures/out/batch9_test/mcp_cli_batch",
      "name_template": "{row.FullName}.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/v1/automation/fill-batch \
  -H "Content-Type: application/json" \
  -d '{
    "template": "tests/e2e_fixtures/real/form.pdf",
    "data": "tests/e2e_fixtures/real/batch_data.csv",
    "output_dir": "tests/e2e_fixtures/out/batch9_test/api_cli_batch_out",
    "name_template": "{row.FullName}.pdf"
  }'
```

## 4. `doctor`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Runs a comprehensive suite of diagnostic probes across the system to ensure PaperPilot is ready to execute tasks locally and offline.**  
Probes local dependencies, filesystem permissions, network ports, on-device neural model weights, and storage limits to verify cleanroom offline-readiness:
- **Ephemeral Scratch Volume**: Verifies `/tmp` (or configured temp directory) is readable, writable, and supports clean ephemeral deletion (`paperpilot_probe.tmp`).
- **Gateway Port Availability**: Checks whether the PaperPilot REST/MCP Gateway port (`127.0.0.1:7823`) is available or actively listening and healthy.
- **On-Device Neural OCR Models**: Confirms presence and integrity of ONNX detection and recognition models (`models/ocr/ppocrv4/det.onnx`, `rec.onnx`, or custom path via `PAPERPILOT_OCR_MODELS_DIR`).
- **Disk Storage Capacity**: Verifies scratch storage has at least 5 GB of free space for zero-leak document processing.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
curl -s -X GET http://127.0.0.1:7823/api/v1/doctor
```

## 5. `document_pii_scan`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scans document content streams for Personally Identifiable Information (PII) and sensitive data leaks.**  
Heuristically analyzes text layers across pages using validated regex patterns and checksum validation to detect Social Security Numbers (SSN), Credit Card Numbers (CCN), Email Addresses, Phone Numbers, and International Bank Account Numbers (IBAN). Emits structured findings with masked previews, page indexes, and confidence scores.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "document_pii_scan",
    "arguments": {
      "input": "tests/e2e_fixtures/real/text_pii.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/pii_report.json",
      "scan_ssn": true,
      "scan_emails": true,
      "scan_phones": true
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 6. `document_workflow`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Runs a declarative version-1 document workflow recipe through one unified in-memory PDF engine.
Accepts custom TOML `.ppflow` files, JSON recipe definitions, or built-in system presets (e.g. `preset-legal-discovery`, `preset-clean-scan`).

- **Unified Multi-Stage Execution**: Supports compound document transformations without intermediate on-disk tempfile writing.
- **Runtime Conditions & Branching**: Evaluates dynamic conditions (scanned checks, page counts, form fields, text matching, byte size) on the actual document entering each stage.
- **Topological Dependencies**: Supports declared stage dependencies (`depends_on = ["stage-id"]`) for fan-out execution.
- **Universal Interface Support**: Available across CLI (`paperpilot workflow run`), MCP (`document_workflow`), REST API (`POST /api/v1/workflow/run-json` and multipart `POST /api/v1/workflow/run`), and WebAssembly (`WasmPdfEngine.workflow`).
- **Audit Trails**: Disk-based executions automatically persist terminal receipts (`.paperpilot-receipts/run-*.json`) queryable via [`execution_receipt_list`](/reference/tools/execution_receipt_list/) and [`execution_receipt_get`](/reference/tools/execution_receipt_get/).

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "name": "document_workflow",
  "arguments": {
    "input": "incoming_document.pdf",
    "output": "processed.pdf",
    "recipe": "audit_pipeline.ppflow"
  }
}
```

### REST API
```bash
N/A
```

## 7. `excel_to_csv`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Exports Excel workbooks (.xlsx, .xls) into standard RFC 4180 CSV text files.**  
Extracts spreadsheet data quickly with high fidelity without requiring Microsoft Office or external runtime dependencies.

- **Multi-Sheet Automatic Decomposition**: When exporting a multi-sheet workbook without specifying a single sheet, the tool creates the target CSV for the primary sheet and automatically exports each individual worksheet as `<output>_<SheetName>.csv`.
- **Targeted Sheet Extraction**: Pass `--sheet <name>` (or `"sheet"` in MCP/REST) to export only a specific sheet.
- **Data Type Fidelity**: Accurately formats text, numeric, date, and boolean cell values into standard comma-separated representations with proper quoting.

---

### CLI
```bash
# Export all sheets (primary + individual per-sheet CSVs)
paperpilot office excel-to-csv \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.csv \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "excel_to_csv",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_excel.csv",
      "sheet": "Sheet1"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Office route
curl -s -X POST http://127.0.0.1:7823/api/v1/office/excel-to-csv \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_excel.csv"
  }'
```

## 8. `excel_to_json`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Ingests Excel workbooks (.xlsx, .xls) and converts sheets, rows, and columns into structured JSON arrays.**  
Parses rows and tabular columns into key-value JSON objects ready for AI agents, RAG pipelines, REST APIs, or database imports.

- **Cell Type Preservation**: Accurately preserves numbers as floats/integers, booleans, dates, and strings rather than flattening everything into text.
- **Configurable Header Parsing**: Automatically treats the first row as object keys (`has_headers: true` by default), or maps rows into index-based structures when disabled (`--no-headers`).
- **Disk or Direct Memory Output**: Optionally write output directly to an on-disk JSON file, or stream structured JSON directly in the response payload.

---

### CLI
```bash
# Extract to disk with JSON output format
paperpilot office excel-to-json \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.json \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "excel_to_json",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "has_headers": true
    }
  },
  "id": 1
}
```

### REST API
```bash
# Office route
curl -s -X POST http://127.0.0.1:7823/api/v1/office/excel-to-json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx"
  }'
```

## 9. `execution_receipt_get`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Reads one durable execution receipt by its exact filename (`receipt_id`) in an explicitly specified directory. Returns normalized metadata, cryptographic SHA-256 hashes, start/finish timestamps, durations, error information, and the complete execution report under `details`.

- **Comprehensive Stage / Row Audit**: Unpacks the full stage tree for workflow recipes (including completed, skipped, and failed stages with error reasons) or per-row outcomes for batch merge runs.
- **Path Isolation**: Rejects path traversal attempts (`../`), symlinks, and files outside the designated receipt directory.
- **Legacy Compatibility**: Reads workflow receipts (`run-*.json`), watcher receipts (`*.receipt.json`), and batch mail-merge receipts (`execution_receipt.json`) without modifying or rewriting original files on disk.

---

### CLI
```bash
paperpilot workflow history \
  --directory ./out/.paperpilot-receipts \
  --id run-HrUGVW.json \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "execution_receipt_get",
  "arguments": {
    "directory": "/local/out/.paperpilot-receipts",
    "receipt_id": "run-HrUGVW.json"
  }
}
```

### REST API
```bash
N/A
```

## 10. `execution_receipt_list`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Lists normalized receipt summaries from an explicitly selected local receipts directory, ordered newest to oldest.
No process registry or volatile daemon cache is used; completed execution records remain persistent and queryable across process restarts.

- **Universal Receipt Aggregation**: Reads receipts produced by:
  - Workflow / Recipe runs (`.paperpilot-receipts/run-*.json`)
  - Watcher hot folder daemons (`*.receipt.json`)
  - Batch mail-merge runs (`execution_receipt.json`)
- **Bounded Resource Consumption**: List responses summarize core metadata (IDs, SHA-256 hashes, timestamps, durations, and outcomes) while omitting verbose stage trees to bound memory and response sizes. Full per-stage trees or per-row records can be retrieved individually using [`execution_receipt_get`](/reference/tools/execution_receipt_get/).
- **Safe Directory Traversal**: Rejects path traversals, symbolic links, and receipt files exceeding 16 MiB.
- **Configurable Limits**: Supports bounding return count with `limit` (default: 50; valid: 1–1000).

---

### CLI
```bash
# Query receipt history in a receipts directory
paperpilot workflow history --directory ./out/.paperpilot-receipts --limit 50 --json
```

### MCP JSON-RPC
```json
{
  "name": "execution_receipt_list",
  "arguments": {
    "directory": "/local/out/.paperpilot-receipts",
    "limit": 50
  }
}
```

### REST API
```bash
N/A
```

## 11. `images_to_gif`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines an ordered sequence of images into a single animated looping GIF.**  
Takes a sequence of raster image files (PNG, JPEG, WebP) and stitches them into a looping GIF file with custom frame delays, frame rates, and consistent width scaling. Designed for time-lapse sequences, stop-motion previews, and slide-deck animations.

---

### CLI
```bash
paperpilot images-to-gif \
  --input tests/e2e_fixtures/real/img1.png,tests/e2e_fixtures/real/img2.png \
  --output tests/e2e_fixtures/out/penta_e2e/slideshow.gif \
  --fps 4 \
  --width 800 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "images_to_gif",
    "arguments": {
      "images": [
        "tests/e2e_fixtures/real/img1.png",
        "tests/e2e_fixtures/real/img2.png"
      ],
      "output": "tests/e2e_fixtures/out/penta_e2e/slideshow.gif",
      "fps": 4,
      "width": 800
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/images-to-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "images": [
      "tests/e2e_fixtures/real/img1.png",
      "tests/e2e_fixtures/real/img2.png"
    ],
    "output": "tests/e2e_fixtures/out/penta_e2e/slideshow.gif",
    "fps": 4,
    "width": 800
  }'
```

## 12. `img_convert`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs pure-Rust transcoding between image formats without system-level image libraries.**  
Converts images seamlessly between PNG, JPEG, WebP, TIFF, and AVIF formats with configurable compression quality. Operates with zero external dependencies (no libpng, libjpeg, or ImageMagick required) for cross-platform and sandboxed environments.

---

### CLI
```bash
paperpilot img-convert \
  --input tests/e2e_fixtures/real/document.png \
  --output tests/e2e_fixtures/out/penta_e2e/document.webp \
  --format webp \
  --quality 90 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_convert",
    "arguments": {
      "input": "tests/e2e_fixtures/real/document.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.webp",
      "format": "webp",
      "quality": 90
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-convert \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/document.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.webp",
    "format": "webp",
    "quality": 90
  }'
```

## 13. `img_crop_to_content`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Detects document boundaries and automatically trims scanner bed and background margins.**  
Samples outer image borders and corner regions to determine background luminance, finds the minimum enclosing bounding box of the document content that deviates by more than a configurable luminance threshold, and crops with an optional protective padding margin.

---

### CLI
```bash
paperpilot img-crop-to-content \
  --input tests/e2e_fixtures/real/scanner_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/cropped.png \
  --threshold 0.1 \
  --padding 10 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_crop_to_content",
    "arguments": {
      "input": "tests/e2e_fixtures/real/scanner_scan.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/cropped.png",
      "threshold": 0.1,
      "padding": 10
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-crop-to-content \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/scanner_scan.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/cropped.png",
    "threshold": 0.1,
    "padding": 10
  }'
```

## 14. `img_deskew`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Detects text baseline skew angle and straightens tilted document scans.**  
Applies pure-Rust projection profile analysis to calculate skew angles between -45° and +45°, automatically rotating the canvas and filling exposed borders with a clean background color. Essential for digitizing legacy paperwork, camera photo captures, and preparing scans for high-accuracy OCR.

---

### CLI
```bash
paperpilot img-deskew \
  --input tests/e2e_fixtures/real/tilted_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/deskewed.png \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_deskew",
    "arguments": {
      "input": "tests/e2e_fixtures/real/tilted_scan.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/deskewed.png"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-deskew \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/tilted_scan.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/deskewed.png"
  }'
```

## 15. `img_optimize_for_llm`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Downscales and sharpens images for Vision LLM token budget economy and OCR legibility.**  
Scales oversized high-DPI document scans using Lanczos3 interpolation down to target dimension thresholds (e.g., 1024px) to drastically reduce vision token consumption while applying a subtle unsharp high-pass contrast filter that boosts text edge clarity and character recognition accuracy for models like GPT-4o, Claude 3.5 Sonnet, and Gemini 1.5 Pro.

---

### CLI
```bash
paperpilot img-optimize-for-llm \
  --input tests/e2e_fixtures/real/highres_scan.png \
  --output tests/e2e_fixtures/out/penta_e2e/llm_optimized.png \
  --max-dim 1024 \
  --quality 85 \
  --json
```

### MCP JSON-RPC
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

### REST API
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

## 16. `img_redact`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies permanent, irreversible pixel-level redaction over sensitive image regions.**  
Overwrites pixel data within specified rectangular bounding boxes (`x,y,width,height`) with solid color fills (black, white, red, or hex `#RRGGBB`). Unlike visual overlay layers, the original underlying pixel values are completely destroyed and unrecoverable from metadata or hidden alpha channels.

---

### CLI
```bash
paperpilot img-redact \
  --input tests/e2e_fixtures/real/license.png \
  --output tests/e2e_fixtures/out/penta_e2e/redacted.png \
  --rect "120,450,300,50" \
  --color "#000000" \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "img_redact",
    "arguments": {
      "input": "tests/e2e_fixtures/real/license.png",
      "output": "tests/e2e_fixtures/out/penta_e2e/redacted.png",
      "rect": [
        "120,450,300,50",
        "320,400,180,60"
      ],
      "color": "#000000"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/img-redact \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/license.png",
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.png",
    "rect": ["120,450,300,50"],
    "color": "#000000"
  }'
```

## 17. `json_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Synthesizes publication-ready PDF documents from structured JSON document definitions.**  
Transforms structured layout trees, text blocks, table elements, styling metadata, and embedded images from JSON directly into clean, rendered PDF documents without requiring a web browser engine.

- **Deterministic Layout Engine**: Compiles JSON document specifications into exact PDF page geometries, standard fonts, and coordinates.
- **Embedded Visual Elements**: Supports rendering structured metadata, headers, paragraphs, and embedded base64 graphics.
- **Universal Cross-Platform Support**: Supported natively in CLI, MCP, REST Gateway, and directly in WebAssembly (`paperpilot-wasm`).

---

### CLI
```bash
# Direct json-to-pdf command
paperpilot json-to-pdf \
  --input tests/e2e_fixtures/real/json_doc.json \
  --output tests/e2e_fixtures/out/batch16_test/json_out.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "json_to_pdf",
    "arguments": {
      "input": "tests/e2e_fixtures/real/json_doc.json",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_json.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/json_to_pdf \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/json_doc.json",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_json.pdf"
  }'
```

## 18. `office_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts raw, unformatted text strings from Microsoft Office documents (.docx, .xlsx, .pptx).**  
Bypasses visual styles, runs, formulas, and slide transitions to dump pure text contents for high-speed indexing, grep search, and LLM embedding/RAG ingest pipelines.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/penta_e2e/office_extracted.txt"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 19. `office_modernize_legacy`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Upgrades obsolete legacy binary Microsoft Office files (Word 97-2003 `.doc`, Excel 97-2003 `.xls`, PowerPoint 97-2003 `.ppt`) to modern OpenXML formats (`.docx`, `.xlsx`, `.pptx`).**  
Parses CFBF (Compound File Binary Format) structures, extracts record streams and BIFF cell matrices, and rebuilds standard ECMA-376 OpenXML package containers.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_modernize_legacy",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xls",
      "output": "tests/e2e_fixtures/out/penta_e2e/modernized.xlsx"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 20. `office_to_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts Microsoft Word documents, Excel spreadsheets, and PowerPoint slide decks into structured Markdown.**  
Preserves heading hierarchies, bullet points, bold/italic formatting, and data tables with standard Markdown syntax (`| col | col |`), optimized for LLM RAG context ingestion.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "office_to_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/penta_e2e/output.md"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 21. `pdf_annotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Appends interactive and visual annotations (highlights, underlines, notes, and stamps) onto PDF pages.**  
Programmatically injects standard PDF markup annotations (`/Highlight`, `/Underline`, `/Text`, `/Square`, `/FreeText`) at exact spatial coordinates or dynamically anchored to search patterns.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_annotate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_annotate/Input/research_paper_to_annotate.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/annotated.pdf",
    "annotations": [
      {
        "id": "1",
        "type": "highlight",
        "page": 1,
        "x": 50.0,
        "y": 50.0,
        "w": 100.0,
        "h": 20.0,
        "color": "#ffff00",
        "content": "Verified section"
      }
    ]
  }'
```

## 22. `pdf_attachments`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Inspects, extracts, adds, or removes embedded file attachments in PDF documents and portfolios.**  
Allows users to manage encapsulated attachments within PDF documents. A PDF portfolio or container can store arbitrary file types (e.g. CSVs, raw images, spreadsheets, reports) encapsulated securely inside its `/EmbeddedFiles` NameTree. Supports four core lifecycle actions: `list`, `add`, `extract`, and `remove`.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 23. `pdf_auto_rotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Heuristically detects page aspect ratios and orientations to automatically upright pages.**  
Analyzes page dimensions (`MediaBox`) and existing `/Rotate` dictionary properties, rotating landscape-oriented pages to portrait (or vice versa based on target orientation) by 90-degree increments to ensure consistent reading orientation without re-encoding content streams.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_auto_rotate",
    "arguments": {
      "input": "tests/e2e_fixtures/rotated.pdf",
      "output": "tests/e2e_fixtures/out/batch18/auto_rotated_mcp.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 24. `pdf_bates`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies consecutive legal Bates numbering stamps across documents.**  
Stamps formatted identification numbers (e.g. `CONF-000001`, `EXHIBIT-A-000042`) onto page margins for legal discovery, audit trails, and court filings.

---

### CLI
```bash
paperpilot bates \
  --input ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/bates_cli_Simple.pdf \
  --prefix "CONF-" \
  --start 1 \
  --position bottom-right \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_bates",
    "arguments": {
      "input": "ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf",
      "prefix": "CONF-",
      "start_number": 1,
      "padding": 6,
      "position": "bottom-right"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_bates \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_bates/Input/legal_discovery_document.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/bates.pdf",
    "prefix": "CONF-",
    "start_number": 1,
    "position": "bottom-right"
  }'
```

## 25. `pdf_booklet`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Transforms a document into print-ready saddle-stitch booklet signatures.**  
Calculates saddle-stitch page pairings and reorders pages into booklet sequence so that when printed double-sided, folded down the spine, and stapled, the pages read in natural numerical sequence. Documents whose page counts are not multiples of 4 are automatically padded with blank trailing pages.

---

### CLI
```bash
paperpilot pdf-booklet \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/booklet.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_booklet",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/booklet.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 26. `pdf_bookmarks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Reads, extracts, and parses the document outline and Table of Contents bookmarks hierarchy into structured JSON.**  
Allows inspection of internal document navigation trees, section hierarchies, titles, and target page destinations.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "success": true,
  "message": "Successfully extracted 0 bookmark(s) to JSON.",
  "data": {
    "bookmarks_extracted": 0,
    "output": "tests/e2e_fixtures/out/bookmarks_extracted.json"
  },
  "output_path": "tests/e2e_fixtures/out/bookmarks_extracted.json"
}
```

### REST API
```bash
N/A
```

## 27. `pdf_burst`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Explodes a multi-page PDF into separate 1-page PDF files.**  
If you have a 10-page document, this creates 10 individual single-page files (`page_1.pdf`, `page_2.pdf`, etc.) in an output directory.
- **Opposite of Merge**: Where `merge` combines multiple files into 1, `burst` splits 1 file into every individual page.
- **Different from Split**: `split` requires cut points or page ranges; `burst` automatically breaks up all pages individually.

### CLI
```bash
paperpilot burst --input tests/e2e_fixtures/real/multi_page.pdf --output tests/e2e_fixtures/out/penta_e2e/burst --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_burst",
  "arguments": {
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/burst -H 'Content-Type: application/json' -d '{"input": "tests/e2e_fixtures/real/multi_page.pdf", "output_dir": "tests/e2e_fixtures/out/penta_e2e/burst"}'
```

## 28. `pdf_change_permissions`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Configures user and owner permissions for printing, copying, and modification.**  
Enables granular control over document permissions across PDF viewers according to ISO 32000-1 specification encryption dictionaries (`/P` bitmask flags).

- **Granular Permissions Control**: Revoke or allow printing (`allow_printing`), copying (`allow_copying`), content modifications (`allow_modifying`), annotations (`allow_annotations`), form filling (`allow_form_filling`), and accessibility screen reading (`allow_accessibility`).
- **Cryptographic Bitmask Management**: Dynamically recalculates the PDF permission integer flags (`/P`) and re-encrypts the document under standard 128-bit key schemes.
- **Credential Segregation**: Supports distinct user opening passwords and administrative owner passwords.

### CLI
```bash
# Revoke printing and text copying permissions
paperpilot pdf-change-permissions \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/perm_restricted.pdf \
  --allow-print false \
  --allow-copy false \
  --json

# Change permissions with custom owner password
paperpilot pdf-change-permissions \
  --input document.pdf \
  --output secured.pdf \
  --allow-modify false \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_change_permissions",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf",
    "allow_print": false,
    "allow_copy": false,
    "owner_password": "adminsecret"
  }
}
```

### REST API
```bash
# Via dedicated REST endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/change-permissions \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf", "allow_print": false, "allow_copy": false}'

# Via dedicated endpoint (tool alias)
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_change_permissions \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/perm_restricted.pdf", "allow_print": false, "allow_copy": false}'
```

## 29. `pdf_chunk`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits PDF documents into intelligent, structure-aware semantic text chunks tailored for LLMs and AI agent context windows.**

Unlike naive character or fixed-window splitters, `pdf_chunk` respects the intrinsic hierarchy and structural flow of documents:
- **Heading Hierarchy Preservation**: Recognizes Markdown headings (`#`, `##`, `###`) and enforces clean hard breaks (when `break_on_headings: true`), ensuring topics are not awkwardly split across chunk boundaries.
- **Heading Context Tracking**: Every generated chunk includes `headings_context`, an array of active ancestor headings that preserves conversational context even for chunks deep within sub-sections.
- **Markdown Table Integrity**: Detects Markdown tables (`| ... |`) and keeps rows together in a single chunk whenever possible, avoiding fragmented data representations.
- **Boilerplate Suppression**: When `strip_boilerplate: true`, automatically detects and removes repetitive running headers and footers that repeat across consecutive pages (e.g., page numbers, confidential notices, company letterheads).
- **Dynamic Overlap**: When beginning subsequent chunks, rewinds by up to `overlap_tokens` sentences/words from previous content to maintain semantic continuity, without carrying over major heading boundaries.
- **Memory-Mapped Streaming**: Seamlessly integrates with `DocumentSource` zero-copy memory mapping on native platforms for fast, low-memory chunking of large multi-hundred-page files.

### CLI
```bash
# Basic semantic chunking with default 500 target tokens and 50 token overlap
paperpilot pdf-chunk --input tests/e2e_fixtures/real/single_page.pdf --json

# Custom chunking saving output directly to a JSON file
paperpilot pdf-chunk \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --target-tokens 500 \
  --overlap 50 \
  --strip-boilerplate \
  --break-on-headings \
  --output /tmp/pdf_chunk_out.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_chunk",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "target_tokens": 500,
    "overlap_tokens": 50,
    "strip_boilerplate": true,
    "break_on_headings": true
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/chunk \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "target_tokens": 500,
    "overlap_tokens": 50,
    "strip_boilerplate": true,
    "break_on_headings": true
  }'
```

## 30. `pdf_classify_type`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Classifies a PDF document into standard taxonomy types using structural and typographical layout heuristics.**  
`pdf_classify_type` inspects typography hierarchies, keywords, metadata tags, and tabular layouts to categorize the document (such as `Document Specification`, `Invoice`, `Academic Paper`, `Contract`, `Presentation`, or `Form`) and outputs a classification score with confidence levels.

### CLI
```bash
# Classify document type and print JSON output to stdout
paperpilot classify-type --input tests/e2e_fixtures/real/single_page.pdf --json

# Save classification report to JSON file
paperpilot classify-type \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output /tmp/classify.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_classify_type",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "/tmp/classify.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_classify_type \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

## 31. `pdf_comic_archive`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF documents or folders of images into Comic Book Archive (.cbz) packages.**  
Rasterizes each PDF page into high-fidelity sequential PNG images (`page_001.png`, `page_002.png`, etc.) or packs directories of illustration scans using natural sort order (`natural_cmp`) into a standardized, zero-compression ZIP container compatible with all major comic and e-reader applications.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_comic_archive",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/comic.cbz"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 32. `pdf_compare`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Compares two PDF documents page-by-page and generates a structured comparison report.**  
`pdf_compare` inspects text streams, font structures, and visual elements across both documents. It returns whether differences were detected (`diff_detected: boolean`) alongside the list of divergent page numbers (`pages: number[]`).

### CLI
```bash
# Compare two PDFs and output JSON execution summary
paperpilot compare \
  --input tests/e2e_fixtures/real/merge_a.pdf \
  --input-b tests/e2e_fixtures/real/merge_b.pdf \
  --json

# Write structured diff report directly to JSON file
paperpilot compare \
  --input tests/e2e_fixtures/real/merge_a.pdf \
  --input-b tests/e2e_fixtures/real/merge_b.pdf \
  --output /tmp/diff_report.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_compare",
  "arguments": {
    "input_a": "tests/e2e_fixtures/real/merge_a.pdf",
    "input_b": "tests/e2e_fixtures/real/merge_b.pdf",
    "output": "/tmp/diff_report.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_compare \
  -H 'Content-Type: application/json' \
  -d '{
    "file1": "tests/e2e_fixtures/real/merge_a.pdf",
    "file2": "tests/e2e_fixtures/real/merge_b.pdf",
    "output": "/tmp/diff_report.json"
  }'
```

## 33. `pdf_compress`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Reduces PDF file size through intelligent image resampling and stream compression.**  
Re-encodes embedded high-resolution raster images, applies standard Flate/Zlib compression to uncompressed content streams, and cleans unreferenced objects to produce lightweight, web- and email-friendly documents.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_compress",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "quality": "medium",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_compressed.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 34. `pdf_convert_excel`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Renders spreadsheet workbooks (Excel `.xlsx`/`.xls` or CSV) directly into publication-ready styled PDF documents.**  
Powered by a semantic table-to-PDF formatting engine, `pdf_convert_excel` transforms raw worksheets, columns, and tabular datasets into beautifully formatted PDF documents with customizable typography presets, orientation controls, and pagination.

- **Theme Presets**: Built-in CSS styling presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **Custom CSS Support**: Inject custom stylesheets or inline CSS overrides for table headers, row striping, borders, and typography.
- **Page Geometry Control**: Configurable page orientations (`portrait` or `landscape`) and standard paper sizes (`A4`, `Letter`).
- **Multi-Sheet Handling**: Formats sheets cleanly with appropriate page flow.

> [!NOTE]
> To extract data in the reverse direction (exporting PDF tables to Excel), see `pdf_to_xlsx` or `pdf_extract_tables`. `pdf_convert_excel` converts spreadsheets *into* PDF.

---

### CLI
```bash
# Using convert-excel
paperpilot convert-excel \
  --input tests/e2e_fixtures/real/test.xlsx \
  --output tests/e2e_fixtures/out/batch16_test/excel_out.pdf \
  --preset elegant \
  --orientation landscape \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_excel",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.xlsx",
      "output": "tests/e2e_fixtures/out/batch16_test/mcp_excel.pdf",
      "preset": "elegant",
      "orientation": "landscape",
      "page_size": "A4"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_excel \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.xlsx",
    "output": "tests/e2e_fixtures/out/batch16_test/rest_excel.pdf",
    "preset": "elegant",
    "orientation": "landscape"
  }'
```

## 35. `pdf_convert_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Renders HTML source documents and web pages directly into styled, publication-ready PDF files.**  
Powered by a lightweight headless layout engine, `pdf_convert_html` transforms HTML markup, tables, images, and embedded CSS into high-fidelity PDF documents with pagination, custom margins, and headers/footers.

- **CSS Theme Presets**: Built-in typography presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **Custom Stylesheet Injection**: Inject external `.css` files or inline CSS overrides directly into the rendering pipeline.
- **Page Geometry Control**: Configurable page formats (e.g. `A4`, `Letter`), margins (`margin_mm`), and orientations (`portrait`, `landscape`).

---

### CLI
```bash
# Direct Subcommand
paperpilot html-to-pdf \
  --input tests/e2e_fixtures/real/test.html \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_html",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.html",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
      "preset": "github",
      "page_size": "A4",
      "orientation": "portrait"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_html \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.html",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
    "preset": "github"
  }'
```

## 36. `pdf_convert_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Renders Markdown source documents directly into beautifully styled, publication-ready PDF files.**  
Parses standard CommonMark and GitHub Flavored Markdown (tables, blockquotes, syntax-highlighted code fences, task lists) into structured semantic layouts, then styles and paginates them into high-fidelity PDF documents.

- **Markdown Features Supported**: Headers (`h1`–`h6`), bullet and ordered lists, GFM tables with column alignment, blockquotes, code fences, and emphasis.
- **Theme Presets**: Integrated typography presets including `github`, `elegant`, `minimal`, `branded`, and `compact`.
- **CSS Customization**: Inject external CSS files or custom inline styles for headers, footers, margins, and page breaks.

---

### CLI
```bash
# Direct Subcommand
paperpilot md-to-pdf \
  --input tests/e2e_fixtures/real/test.md \
  --output tests/e2e_fixtures/out/penta_e2e/document.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_convert_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/test.md",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
      "preset": "elegant",
      "page_size": "A4",
      "orientation": "portrait"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_convert_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/test.md",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.pdf",
    "preset": "elegant"
  }'
```

## 37. `pdf_create_form_field`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Appends interactive AcroForm widget annotations (text fields or checkboxes) to a target PDF page.**  
Adds form field entries into the document Catalog `/AcroForm` `/Fields` array and attaches widget dictionaries (`/Subtype /Widget`) to the designated page's `/Annots` array with specified bounding box coordinates (`rect`).

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_create_form_field",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "field_name": "ApplicantName",
      "field_type": "text",
      "page": 1,
      "rect": [100, 500, 350, 530],
      "output": "tests/e2e_fixtures/out/batch9_test/mcp_created_field.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
{
  "success": true,
  "message": "Form field created successfully.",
  "data": null,
  "output_path": "tests/e2e_fixtures/out/batch9_test/api_created_field.pdf",
  "diff_detected": null
}
```

## 38. `pdf_crop`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Trims page margins or crops documents to a custom rectangular bounding box.**  
Adjusts PDF `/CropBox` and `/MediaBox` dictionary vectors to eliminate scanner margins, cut registration marks, or isolate target regions across all or designated pages.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_crop",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "box": "10,10,200,200",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_cropped.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 39. `pdf_decrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Removes password protection and usage restrictions from an unlocked PDF.**  
Decrypts the document using the supplied user or owner password and outputs an unrestricted PDF that can be opened, printed, and edited freely.

- **Password Verification**: Validates the input password against the document's encryption dictionary before stripping security handlers.
- **Full Structure Preservation**: Parses the catalog, page tree, streams, annotations, and metadata intact before serializing into clean, decrypted objects.
- **Downstream Pipeline Ready**: Produces an unencrypted standard PDF suitable for subsequent automated tasks like indexing, compression (`pdf_compress`), or OCR.
- **Fail-Safe Rejection**: Rejects incorrect passwords with descriptive error codes rather than producing corrupt or incomplete files.

### CLI
```bash
# Decrypt an encrypted PDF using the document password
paperpilot decrypt \
  --input tests/e2e_fixtures/real/encrypted.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/decrypt_cli_Simple.pdf \
  --password secret123 \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_decrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/encrypted.pdf",
    "password": "secret123",
    "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"
  }
}
```

### REST API
```bash
# Via dedicated REST route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_decrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "secret123", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'

# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_decrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/encrypted.pdf", "password": "secret123", "output": "tests/e2e_fixtures/out/penta_e2e/decrypted.pdf"}'
```

## 40. `pdf_dedup_streams`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs lossless indirect object stream deduplication and CMYK-safe document compression.**  
Leverages cryptographic hashing (SHA-256) to identify byte-identical streams across the entire document structure, ensuring zero false positives. Duplicate streams are merged into a single canonical indirect object, reducing file size significantly without sacrificing any data.

- **Cryptographic Stream Hashing**: Computes SHA-256 digests of all uncompressed and compressed object streams (fonts, vector forms, raster images, content streams).
- **Canonical Object Consolidation**: Rewires all indirect references across page dictionaries to point to the canonical stream object and eliminates redundant clones.
- **CMYK Color Space Guard**: Preserves original prepress and print quality profiles (`/DeviceCMYK` or `/ICCBased`) during compression workflows to prevent irreversible color-space distortion.
- **Lossless Size Reduction**: Ideal for compiled documents, slide decks, repetitive catalog pages, and multi-page reports with repeated header/footer art or embedded logos.

---

### CLI
```bash
# Deduplicate redundant streams and save to optimized target file
paperpilot pdf-dedup-streams \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/deduped.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_dedup_streams",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_dedup_streams \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/deduped.pdf"
  }'
```

## 41. `pdf_delete_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Deletes specific pages or page ranges from a PDF document.**  
Prunes unwanted pages (such as blank sheets, cover letters, or draft appendices) from the PDF page tree, updates object references, and cleanly rebuilds the document catalog without corrupting internal cross-reference tables.

---

### CLI
```bash
paperpilot delete-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 2,4 \
  --output tests/e2e_fixtures/out/penta_e2e/deleted.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_delete_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "pages": "2,4",
      "output": "tests/e2e_fixtures/out/penta_e2e/deleted.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 42. `pdf_encrypt`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Secures a PDF document with standard password encryption.**  
Protects document contents by establishing encryption keys with user opening passwords and optional owner administrative passwords to restrict printing, copying text, or editing.

- **Password Protection**: Prevents unauthorized access by requiring a password to open and view the document (`--user-password` / `--password`).
- **Administrative Rights**: Sets optional owner passwords (`--owner-password`) to separate document ownership and permission modifications from general viewing.
- **Standards Compliant**: Generates standard PDF encryption dictionaries adhering to Adobe / ISO PDF encryption specifications.

### CLI
```bash
# Basic password encryption using user password
paperpilot encrypt \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e_security/encrypt_cli_Simple.pdf \
  --user-password secret123 \
  --json

# Encrypt specifying both user password and owner administrative password
paperpilot encrypt \
  --input document.pdf \
  --output encrypted_doc.pdf \
  --user-password userpass \
  --owner-password adminpass \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_encrypt",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "password": "secret123",
    "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf"
  }
}
```

### REST API
```bash
# Via dedicated REST endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_encrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf", "user_password": "secret123"}'

# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_encrypt \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/encrypted.pdf", "password": "secret123"}'
```

## 43. `pdf_extract_images`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts embedded photos, raster graphics, and figures from inside a PDF file.**  
Dumps embedded image streams directly without lossy re-compression to an output directory, allowing recovery of full-resolution graphics, logos, and illustrations.

---

### CLI
```bash
paperpilot extract-images \
  --input tests/e2e_fixtures/real/image_doc.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/images \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_images",
    "arguments": {
      "input": "tests/e2e_fixtures/real/image_doc.pdf",
      "output_dir": "tests/e2e_fixtures/out/penta_e2e/images"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 44. `pdf_extract_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts specific pages or page ranges into a new standalone PDF document.**  
Pulls the selected pages (e.g. `1,3,5-7`) from a multi-page PDF into a fresh file while preserving original formatting, annotations, and media without altering the source file.

---

### CLI
```bash
paperpilot extract-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --pages 1,3 \
  --output tests/e2e_fixtures/out/penta_e2e/extracted.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "pages": "1,3",
      "output": "tests/e2e_fixtures/out/penta_e2e/extracted.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 45. `pdf_extract_patterns`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scans PDF documents for custom keywords and arbitrary regular expression patterns, capturing match contexts and spatial coordinates across all pages.**  
Allows compliance officers, legal auditors, and AI pipelines to search arbitrary keyword lists (e.g. `["Confidential", "NDA", "Acquisition"]`) and regex patterns (e.g. `{"ssn": "\\d{3}-\\d{2}-\\d{4}", "po_number": "PO-\\d{6}"}`).

Every match captures its assigned label, value, page number, bounding box rectangle (`rect`), and surrounding context snippet ($\pm 30$ characters).

---

### CLI
```bash
paperpilot pdf-extract-patterns \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --keywords "PaperPilot,Enterprise" \
  --pattern "year=\d{4}" \
  --context-len 25 \
  --output tests/e2e_fixtures/out/penta_e2e/patterns.json \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_patterns",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/patterns.json",
      "keywords": ["PaperPilot", "Enterprise"],
      "patterns": {
        "year": "\\d{4}"
      },
      "case_sensitive": false,
      "context_len": 25
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 46. `pdf_extract_tables`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts structured tabular data from PDF documents into JSON or CSV.**  
Automatically parses text baseline layout, column alignments, and grid boundaries to reconstruct clean, multi-row tabular records without manual copy-pasting.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_tables",
    "arguments": {
      "input": "tests/e2e_fixtures/real/financial_tables.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/tables.json",
      "format": "json",
      "pages": "all"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 47. `pdf_extract_text`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts all readable textual content from a PDF document with layout and reading-order awareness.**  
Decodes ToUnicode font mappings and content streams in visual reading order, dumping pure text without formatting or headers into a UTF-8 `.txt` file or structured JSON response.

---

### CLI
```bash
paperpilot extract-text \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/extracted_text.txt \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_extract_text",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/extracted_text.txt",
      "pages": "all"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 48. `pdf_fill_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Fills interactive AcroForm fields (text fields, checkboxes, choices) from a JSON dataset.**  
Updates form field values in place, sets field values (`/V`), updates widget appearances (`/AP`), and maintains document integrity.

For bulk batch mail-merge across multiple rows of CSV or JSON, see [`cli_fill_batch`](/reference/tools/cli_fill_batch/) or [`batch_fill`](/reference/tools/batch_fill/).

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_fill_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "values": {
        "FullName": "Jane Doe",
        "Email": "jane.doe@enterprise.corp",
        "AgreedToTerms": "Yes",
        "AccountType": "Enterprise"
      },
      "output": "tests/e2e_fixtures/out/batch9_test/mcp_form_filled.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
{
  "success": true,
  "message": "Form filled successfully.",
  "data": null,
  "output_path": "tests/e2e_fixtures/out/batch9_test/api_form_filled.pdf",
  "diff_detected": null
}
```

## 49. `pdf_flatten`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Flattens interactive AcroForm form fields and annotations directly into the static page content stream.**  
Paints AcroForm widget appearances into page content streams and strips interactive form fields, preventing further form modification or data tampering. Preserves existing page resources and unrelated graphical elements while rendering form values permanently into the document geometry.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 50. `pdf_hash`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Calculates cryptographic checksums and document integrity hashes of a PDF file.**  
Generates deterministic digital fingerprints (SHA-256) and reports exact byte sizes to verify document integrity, detect tampering across transmission pipelines, and establish a verifiable chain of custody.

- **Tamper Detection**: Instantly exposes bit-level alterations, modifications, or stream corruptions.
- **Audit Reports**: Emits structured JSON or plain text digest files reporting algorithm, filepath, SHA-256 hash, and byte length.
- **Universal Cross-Platform**: Supported natively across CLI, MCP, REST Gateway, and purely offline in WebAssembly (`paperpilot-wasm`).

---

### CLI
```bash
# Compute hash and print SHA-256 with JSON metadata
paperpilot hash \
  --input tests/e2e_fixtures/real/form.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_hash",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_hash \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/form.pdf"
  }'
```

## 51. `pdf_header_footer`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Adds customizable headers and footers with configurable positioning, margins, and text alignment across pages.**  
Stamps notices, organizational branding, confidential classifications, dates, or custom dynamic headers across all pages in a PDF document.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
N/A
```

## 52. `pdf_images_to_pdf`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines one or more image files (PNG, JPG, WebP) into a single PDF document.**  
Each image is placed onto its own page with original image dimensions preserved at standard print resolution (72 DPI baseline). Indirect object streams are created cleanly and embedded into a valid PDF document structure. Ideal for photo albums, scanned receipts, document compilations, or creating PDF presentations from graphic slides.

---

### CLI
```bash
paperpilot images-to-pdf \
  --images tests/e2e_fixtures/img1.png tests/e2e_fixtures/img2.png \
  --output tests/e2e_fixtures/out/batch14_test/cli_images_to_pdf.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_images_to_pdf",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/img1.png",
        "tests/e2e_fixtures/img2.png"
      ],
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_images_to_pdf.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 53. `pdf_invert_colors`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Inverts colors or applies dark mode styling across PDF vector content while preserving selectable text.**  
Transforms text color, stroke paths, and fill operations at the PDF object level without rasterizing pages. Supports dark mode, true inversion, high contrast, and sepia modes.

---

### CLI
```bash
paperpilot pdf-invert-colors \
  --input tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_invert.pdf \
  --mode dark_mode \
  --pages 1 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_invert_colors",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_invert.pdf",
      "mode": "dark_mode",
      "pages": "1"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 54. `pdf_linearize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Optimizes a PDF for Fast Web View (byte-serving linearization).**  
Restructures internal cross-reference tables and streams so web browsers and cloud document viewers can instantly display Page 1 before the entire file finishes downloading.

- **Fast Web View Preparation**: Re-orders object streams such that all first-page resources (page dictionary, content stream, fonts, and images) reside at the head of the file.
- **Linearization Dictionary Generation**: Synthesizes the required `/Linearized` catalog dictionary at the beginning of the file, specifying file length, primary hint stream offset, and first-page object numbers.
- **Primary Hint Streams**: Generates navigation hint tables that enable web servers supporting HTTP byte ranges (`Range: bytes=...`) to stream individual pages on demand.
- **Improved Streaming Latency**: Eliminates reader freezing when opening 100+ page manuals, books, or technical specifications over network connections.

---

### CLI
```bash
# Linearize PDF for Fast Web View
paperpilot pdf-linearize \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf \
  --json

# Alternative CLI alias syntax
paperpilot linearize \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/linearized.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_linearize",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_linearize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/linearized.pdf"
  }'
```

## 55. `pdf_merge`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines multiple separate PDF documents into a single continuous file.**  
Appends pages in order, renumbers indirect objects to resolve ID collisions, consolidates document catalogs, and preserves font resources and page hierarchies into one master document.

---

### CLI
```bash
paperpilot merge \
  --input tests/e2e_fixtures/real/merge_a.pdf tests/e2e_fixtures/real/merge_b.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/merge_cli_simple.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_merge",
    "arguments": {
      "inputs": [
        "tests/e2e_fixtures/real/merge_a.pdf",
        "tests/e2e_fixtures/real/merge_b.pdf"
      ],
      "output": "tests/e2e_fixtures/out/penta_e2e/merge_mcp_simple.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 56. `pdf_merge_alternate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Interleaves and collates pages from two PDF documents with independent step intervals and reversing.**  
Designed specifically for duplex scanning workflows where front pages (odd pages) and back pages (even pages) are scanned in separate batches or inverted stacks. Objects from the secondary document are renumbered and integrated with corrected reference IDs.

---

### CLI
```bash
paperpilot pdf-merge-alternate \
  --input-a tests/e2e_fixtures/page_1.pdf \
  --input-b tests/e2e_fixtures/page_2.pdf \
  --output tests/e2e_fixtures/out/merged_alternate.pdf \
  --step 1 \
  --step-b 1 \
  --reverse-b \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_merge_alternate",
    "arguments": {
      "input_a": "tests/e2e_fixtures/page_1.pdf",
      "input_b": "tests/e2e_fixtures/page_2.pdf",
      "output": "tests/e2e_fixtures/out/merged_alternate_mcp.pdf",
      "step": 1,
      "step_b": 1,
      "reverse_a": false,
      "reverse_b": true
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 57. `pdf_metadata`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Inspects, reads, updates, or edits document metadata fields (Title, Author, Subject, Keywords).**  
Allows extracting document information properties or updating metadata tags into output PDF files for compliance, indexing, and publication standards.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/mcp-exec \
  -H 'Content-Type: application/json' \
  -d '{
    "tool": "pdf_metadata",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf"
    }
  }'
```

### REST API
```bash
N/A
```

## 58. `pdf_nup`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Imposes multiple pages onto a single sheet in an N-Up grid layout (2, 4, 9, or 16 pages per sheet).**  
Arranges pages into printable grid matrices with automatic scaling, aspect ratio preservation, customizable margins, inter-tile spacing, and optional cut/separator borders.

---

### CLI
```bash
paperpilot pdf-nup \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/nup_2up.pdf \
  --nup 2 \
  --ordering horizontal \
  --margin 18 \
  --spacing 10 \
  --draw-borders \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_nup",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/nup_4up.pdf",
      "nup": 4,
      "ordering": "horizontal",
      "margin_pt": 20.0,
      "spacing_pt": 10.0,
      "draw_borders": true
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 59. `pdf_ocr`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs local neural optical character recognition (OCR) on scanned documents and raster images.**  
Embeds an invisible searchable text layer behind scanned page bitmaps or extracts recognized text directly into a plain text file, operating 100% offline without transmitting confidential scans to external cloud APIs.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_ocr",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/ocr_searchable.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 60. `pdf_overlay_underlay`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies letterheads, watermarked stationary, and templates as background or foreground.**  
Leverages PDF Form XObjects to embed stationary templates once and project them across selected target pages with configurable opacity and layer stacking mode (`underlay` behind existing content or `overlay` in the foreground).

---

### CLI
```bash
paperpilot pdf-overlay-underlay \
  --input tests/e2e_fixtures/multi_page.pdf \
  --stationery tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/overlay_underlay.pdf \
  --mode underlay \
  --opacity 0.85 \
  --pages "all" \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_overlay_underlay",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/overlay_mcp.pdf",
      "template_path": "tests/e2e_fixtures/single_page.pdf",
      "mode": "underlay",
      "opacity": 0.85,
      "pages": "all"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 61. `pdf_page_numbers`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Adds dynamic page numbering stamps (e.g. "Page 1 of 5", "1 / 10", or "Page 1") across PDF pages.**  
Stamps clean, professional page numbers at designated screen coordinates (`bottom-center`, `bottom-right`, `top-center`, `top-right`) without shifting or altering preexisting text or vector graphics.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_page_numbers",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "position": "bottom-right",
      "format": "Page {n} of {total}",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_page_numbers.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_page_numbers \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "position": "bottom-center",
    "format": "Page {n} of {total}",
    "output": "tests/e2e_fixtures/out/batch3_test/api_page_numbers.pdf"
  }'
```

## 62. `pdf_posterize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Tiles oversized PDF pages across a multi-sheet grid of standard printable paper with overlap margins and cut guides.**  
Takes single-page or multi-page documents with large dimensions (architectural blueprints, engineering schematics, large maps, or exhibition posters) and slices them into a grid of standard sheets (e.g. A4, Letter, A3) suitable for home or office printing and manual assembly.

---

### CLI
```bash
paperpilot pdf-posterize \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/poster_tiles.pdf \
  --target-sheet a4 \
  --grid 2x2 \
  --overlap 14.17 \
  --scale 1.0 \
  --guides \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_posterize",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/poster_tiles.pdf",
      "target_sheet": "a4",
      "grid": "2x2",
      "overlap_pt": 14.17,
      "scale": 1.0,
      "include_guides": true
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 63. `pdf_rag_chunks`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits a PDF document into section-level semantic text chunks tailored for RAG vector indexing.**  
`pdf_rag_chunks` parses the layout and heading hierarchy of a PDF, partitioning the content into discrete chunks categorized by `page`, `section_title`, `content`, and estimated `token_count`.

### CLI
```bash
# Extract RAG chunks and print JSON output to stdout
paperpilot rag-chunks --input tests/e2e_fixtures/real/single_page.pdf --json

# Save structured RAG chunks directly to a JSON file
paperpilot rag-chunks \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output /tmp/rag_chunks_out.json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_rag_chunks",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "/tmp/rag_chunks_out.json"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_rag_chunks \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

## 64. `pdf_read_form`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts AcroForm field names and current values from a PDF document.**  
Reads text fields, checkboxes, choices, and hierarchical field names. Exports field state to standard stdout, formatted JSON output files, or structured MCP/REST responses.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_read_form",
    "arguments": {
      "input": "tests/e2e_fixtures/real/form.pdf",
      "output": "tests/e2e_fixtures/out/batch9_test/read_fields_mcp.json"
    }
  },
  "id": 1
}
```

### REST API
```bash
{
  "success": true,
  "message": "{\n  \"AccountType\": \"Enterprise\",\n  \"AgreedToTerms\": \"Off\",\n  \"Email\": \"\",\n  \"FullName\": \"\"\n}",
  "data": {
    "AccountType": "Enterprise",
    "AgreedToTerms": "Off",
    "Email": "",
    "FullName": ""
  },
  "output_path": null,
  "diff_detected": null
}
```

## 65. `pdf_redact`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Permanently blackouts and scrubs sensitive text, regular expressions, or coordinate bounding boxes from a PDF.**  
Unlike cosmetic black highlight overlays, PaperPilot's redaction engine sanitizes underlying PDF content streams, glyph positions, and font descriptors to guarantee that redacted characters cannot be extracted or recovered.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
N/A
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_redact \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "ManaulTestingFixtures/pdf_redact/Input/sensitive_customer_records.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/redacted.pdf",
    "query": "Customer",
    "overlay_text": "[CONFIDENTIAL]"
  }'
```

## 66. `pdf_remove_blank`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scans a PDF and strips out all blank or empty pages.**  
Analyzes text operators, vector paths, and pixel ink density across every page. Any accidental empty pages (common from double-sided scanning or printer slips) are safely removed while preserving all content, structural integrity, and annotations on the remaining pages.

---

### CLI
```bash
paperpilot remove-blank \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_remove_blank.pdf \
  --sensitivity 95 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_blank",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_remove_blank.pdf",
      "sensitivity": 95
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 67. `pdf_remove_signatures`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Strips digital signatures, signature field appearances, cryptographic lock dictionaries, and signature widget annotations from a PDF.**  
Removes digital signature cryptographic objects while preserving document tree structure, remaining interactive form fields, page content, and visual fidelity.

---

### CLI
```bash
paperpilot pdf-remove-signatures \
  --input tests/e2e_fixtures/signed.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_remove_signatures.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_remove_signatures",
    "arguments": {
      "input": "tests/e2e_fixtures/signed.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_remove_signatures.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 68. `pdf_render`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Renders PDF document pages into high-resolution raster images.**  
Rasterizes vector paths, text glyphs, font shapes, and embedded graphics at a target DPI resolution for web previews, thumbnail generation, or image processing pipelines.

---

### CLI
```bash
paperpilot render \
  --input tests/e2e_fixtures/single_page.pdf \
  --output tests/e2e_fixtures/out/batch14_test/cli_render.png \
  --page 1 \
  --dpi 150 \
  --format png \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_render",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch14_test/mcp_render.png",
      "page": 1,
      "dpi": 150,
      "format": "png"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 69. `pdf_reorder_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rearranges the sequence of pages in a PDF according to a custom specified order.**  
Restructures the document page tree hierarchy to match the requested permutation (e.g. `5,4,3,2,1` or `2,1,3,4,5`), enabling scan correction, page inversion, and presentation slide reorganizing.

---

### CLI
```bash
paperpilot reorder-pages \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --order 5,4,3,2,1 \
  --output tests/e2e_fixtures/out/penta_e2e/reordered.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_reorder_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "order": "2,1,3,4,5",
      "output": "tests/e2e_fixtures/out/penta_e2e/reordered.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 70. `pdf_repair`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rebuilds corrupted, truncated, or unreadable PDF files.**  
Scans raw file streams, recovers damaged cross-reference tables (`xref`), reconstructs broken object trailers, and salvages readable page structures.

- **Cross-Reference Reconstruction**: Scans the binary document stream sequentially to locate all indirect object definitions (`obj ... endobj`) and synthesizes a compliant cross-reference table.
- **Trailer Rebuilding**: Regenerates missing or truncated `/Trailer` dictionaries and re-establishes pointers to the document `/Root` and `/Info` objects.
- **Orphan Object Pruning**: Drops unreferenced or malformed byte sequences while preserving valid text, vector, and image content streams.
- **Fail-Safe Sanitization**: Rewrites the salvaged objects into a clean, standard-compliant PDF file readable by standard PDF viewers.

---

### CLI
```bash
# Repair damaged PDF and output to recovered target file
paperpilot pdf-repair \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf \
  --json

# Alternative CLI alias syntax
paperpilot repair \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/repaired.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_repair",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_repair \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/repaired.pdf"
  }'
```

## 71. `pdf_rotate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Rotates PDF pages by 90, 180, or 270 degrees clockwise.**  
Modifies page `/Rotate` dictionary attributes in place to correct upside-down scans or landscape documents without re-encoding page content or degrading image clarity.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_rotate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "angle": 90,
      "pages": "1",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_rotated.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 72. `pdf_sanitize`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs deep structural sanitization and threat neutralization on PDF files.**  
Strips sensitive document metadata, removes embedded file attachments, neutralizes JavaScript actions and launch triggers, deletes embedded thumbnails, and cleans private application data.

- **Metadata Eradication**: Clears standard `/Info` dictionary entries (`Author`, `Creator`, `Producer`, `Title`, `Subject`) and strips embedded XMP XML metadata packets.
- **Active Code Neutralization**: Detects and purges `/JavaScript`, `/JS`, `/Launch`, `/EmbeddedFiles`, and automated `/OpenAction` triggers that can execute arbitrary payloads.
- **Attachment Extraction/Removal**: Strips embedded binary files from the document Names dictionary (`/Names -> /EmbeddedFiles`).
- **Private Data Purging**: Deletes `/PieceInfo` and vendor-specific application data (e.g. Adobe Illustrator / Photoshop private editing layers).
- **Embedded Thumbnail Scrubbing**: Deletes uncompressed and compressed image thumbnails stored inside page object dictionaries.

---

### CLI
```bash
# Deep sanitization stripping metadata, scripts, attachments, and thumbnails
paperpilot sanitize \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf \
  --json

# Selective sanitization with specific options
paperpilot sanitize \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/sanitized.pdf \
  --remove-metadata \
  --remove-attachments \
  --remove-scripts \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_sanitize",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf",
      "remove_metadata": true,
      "remove_attachments": true,
      "remove_scripts": true
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_sanitize \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/sanitized.pdf",
    "remove_metadata": true,
    "remove_attachments": true,
    "remove_scripts": true
  }'
```

## 73. `pdf_scale_pages`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Scales PDF page dimensions between standard sheet formats (A4, Letter, Legal, A3, A5, Tabloid) or custom scale percentages.**  
Transforms page media and crop geometry, calculates exact scaling ratios with aspect ratio preservation, and centers content on target dimensions.

---

### CLI
```bash
paperpilot pdf-scale-pages \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/scaled_a4.pdf \
  --target-size A4 \
  --scale-factor 1.0 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_scale_pages",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/scaled_a4.pdf",
      "target_size": "A4",
      "scale_factor": 1.0
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 74. `pdf_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Searches for a text query inside a PDF document or a folder of PDFs and returns matching 1-based page numbers.**  
Full-text token search evaluates decoded page streams using case-insensitive normalized matching. When pointing to a directory, `pdf_search` scans all `.pdf` files in that folder and aggregates matching pages.

### CLI
```bash
# Basic keyword search on a PDF file
paperpilot search --input tests/e2e_fixtures/real/single_page.pdf --query "Single"

# Search with structured JSON output and write to file
paperpilot search \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --query "Single" \
  --output /tmp/search_results.json \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_search",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "Single"
  }
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_search \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "query": "Single"
  }'
```

## 75. `pdf_sign`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies digital signature metadata and AcroForm signature flags to a PDF document.**  
Configures `/AcroForm` signature dictionary structures and flags (`SigFlags = 3`) within the document catalog, reserving signature space and preparing the document for cryptographic workflow validation.

- **Signature Flag Configuration**: Updates catalog root dictionaries with ISO 32000-compliant `/AcroForm` and `/SigFlags` entries.
- **Workflow Preparation**: Prepares documents for downstream signature indexing and signature-field extraction workflows.
- **Certificate Compatibility**: Accepts certificate input paths (`--cert`) in CLI invocations for integration pipelines.

### CLI
```bash
# Apply signature field structure with certificate path
paperpilot signature \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --cert tests/e2e_fixtures/real/dummy.p12 \
  --output tests/e2e_fixtures/out/penta_e2e_security/sign_cli_Simple.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_sign",
  "arguments": {
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"
  }
}
```

### REST API
```bash
# Via generic tools endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_sign \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/real/single_page.pdf", "output": "tests/e2e_fixtures/out/penta_e2e/signed.pdf"}'
```

## 76. `pdf_split`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits a PDF into smaller documents based on page numbers, ranges, cut points, or periodic intervals.**  
Divides a multi-page PDF into multiple discrete documents (e.g., splitting at page 3 partitions pages 1–3 into one document and pages 4–5 into another). Supports cut points, page intervals (`--every`), and full single-page bursts.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output_dir": "tests/e2e_fixtures/out/penta_e2e/pdf_split_simple",
      "pages": "1,3"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 77. `pdf_split_by_outline`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Splits multi-chapter PDF documents into discrete section files based on their Bookmark (`/Outlines`) hierarchy.**  
Parses document catalog outlines and bookmark destination pointers to identify chapter boundaries. Each identified section is extracted into an independent PDF named after its sanitized outline title and packed into a consolidated ZIP archive.

---

### CLI
```bash
paperpilot pdf-split-by-outline \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/split_outline.zip \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split_by_outline",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/split_outline_mcp.zip"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 78. `pdf_split_scanned_book`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Detects center binding gutter spines and splits two-up scanned book spreads into individual pages.**  
Splits landscape book spreads into separate portrait pages (or vice-versa) by calculating gutter centers and adjusting page viewport bounding boxes (`/MediaBox` and `/CropBox`). Because it operates non-destructively on underlying vector objects and content streams, pages retain original vector fidelity without rasterization blur.

---

### CLI
```bash
paperpilot pdf-split-scanned-book \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/split_book_pages.pdf \
  --reading-direction ltr \
  --gutter-overlap 5.0 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_split_scanned_book",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/split_book_mcp.pdf",
      "reading_direction": "ltr",
      "gutter_overlap": 5.0,
      "split_horizontal": false
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 79. `pdf_stamp`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Places coordinate-exact images, logos, badges, and signatures onto PDF pages with configurable transparency.**  
Injects image XObjects (PNG or JPEG) into PDF page content streams and `/Resources` dictionaries at precise point offsets `(x, y)` with custom bounding dimensions `(width, height)` and opacity levels.

---

### CLI
```bash
paperpilot pdf-stamp \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/stamped.pdf \
  --image tests/e2e_fixtures/img1.png \
  --x 50 \
  --y 50 \
  --width 120 \
  --height 80 \
  --opacity 0.85 \
  --pages "all" \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_stamp",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/stamped.pdf",
      "image_path": "tests/e2e_fixtures/img1.png",
      "x": 72.0,
      "y": 72.0,
      "width": 144.0,
      "height": 72.0,
      "opacity": 0.9,
      "pages": "1"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 80. `pdf_stitch_canvas`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Combines multiple pages vertically or horizontally into a seamless, continuous single-page canvas.**  
Transforms each source page into a Form XObject and computes combined canvas bounding box dimensions, applying optional outer margins and inter-page spacing gaps. This is ideal for continuous mobile scrolling viewports, endless-scroll web presentation assets, roll banners, receipts, and panoramic print layouts.

---

### CLI
```bash
paperpilot pdf-stitch-canvas \
  --input tests/e2e_fixtures/multi_page.pdf \
  --output tests/e2e_fixtures/out/stitched_canvas.pdf \
  --direction vertical \
  --margin 10.0 \
  --spacing 5.0 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_stitch_canvas",
    "arguments": {
      "input": "tests/e2e_fixtures/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/stitched_mcp.pdf",
      "direction": "vertical",
      "margin_pt": 15.0,
      "spacing_pt": 10.0
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 81. `pdf_timestamp`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Applies RFC 3161 cryptographically verified TSA timestamp tokens.**  
Affixes immutable timestamp tokens from trusted Time Stamping Authorities (TSA) or offline verifiable timestamp tokens directly into the PDF document trailer and security structures to establish non-repudiation of document state.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_timestamp",
    "arguments": {
      "input": "tests/e2e_fixtures/single_page.pdf",
      "output": "tests/e2e_fixtures/out/batch18/timestamped_mcp.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 82. `pdf_to_docx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Decompiles PDF documents into editable Microsoft Word (.docx) files.**  
Reconstructs text layout, headings, side-by-side tables, and embeds images into a fully editable Word document.

- **Layout & Structure Preservation**: Reconstructs paragraphs, text blocks, and headings according to font sizes and spatial bounds.
- **Table Detection**: Identifies structured cell boundaries, column layouts, and multi-row grids and converts them into native Word OpenXML tables (`w:tbl`).
- **Offline & Privacy-Preserving**: Performs all decompression, font-to-unicode extraction, and OpenXML packaging locally without external cloud dependencies.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format docx \
  --output tests/e2e_fixtures/out/penta_e2e/document.docx \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_docx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.docx"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_docx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.docx"
  }'
```

## 83. `pdf_to_epub`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts fixed-layout PDF documents into reflowable EPUB 3.0 eBooks.**  
Analyzes document layout, extracts logical reading flows, removes artificial line wraps, and bundles chapters, navigation manifest, and CSS stylesheets into standard Open Container Format (OCF) `.epub` packages readable by standard e-readers (Apple Books, Calibre, Kindle, Kobo).

- **EPUB 3.0 Compliant**: Creates valid PKZIP archives with `mimetype` uncompressed first, complete `META-INF/container.xml`, OPF package manifest, and NCX / XHTML navigation.
- **TOC & Chapter Extraction**: Maps PDF outline bookmarks to chapter boundaries and table-of-contents landmarks.
- **Clean Semantic Flow**: Reflows paragraphs and cleans hyphenation artifacts for responsive mobile reading.

---

### CLI
```bash
paperpilot pdf-to-epub \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/document.epub \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_epub",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.epub"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Gateway Endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-epub \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.epub"
  }'
```

## 84. `pdf_to_gif`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts multi-page PDF documents into animated looping GIF flipbooks.**  
Renders successive pages of a PDF document and compiles them into a smooth, lightweight animated GIF. Configurable frame rate (`fps`), frame delay (`delay`), target image width, and maximum page count allow creating interactive flipbook previews, thumbnail reels, and social media shareables without external dependencies.

---

### CLI
```bash
paperpilot pdf-to-gif \
  --input tests/e2e_fixtures/real/multi_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/preview.gif \
  --fps 2 \
  --width 800 \
  --max-pages 10 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_gif",
    "arguments": {
      "input": "tests/e2e_fixtures/real/multi_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/preview.gif",
      "fps": 2,
      "width": 800,
      "max_pages": 5
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/to-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/multi_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/preview.gif",
    "fps": 2,
    "width": 800
  }'
```

## 85. `pdf_to_html`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts a PDF into styled, standalone HTML5.**  
Extracts document layout, fonts, headings, paragraphs, and inline styles into responsive web-friendly markup without requiring browser plugins.

- **Standalone Document Generation**: Produces complete HTML5 documents with self-contained `@page` styling, clean layout margins, and print-ready stylesheets.
- **Semantic Text Hierarchy**: Preserves document structure including headers (`h1`–`h4`), ordered and unordered paragraphs, and bold/italic font styling.
- **CSS Presets & Custom Styling**: Supports preset templates or custom injected inline CSS rules when executed through the conversion engine.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format html \
  --output tests/e2e_fixtures/out/penta_e2e/document.html \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_html",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_html \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.html"
  }'
```

## 86. `pdf_to_json`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Exports full multimodal PDF layout AST to structured JSON.**  
Extracts text blocks, page arrays, character metrics, structural hierarchy, and document metadata into clean, machine-readable JSON for downstream AI workflows and data pipelines.

- **Document AST & Page Arrays**: Structures pages sequentially with per-page text blocks, character counts, and document-level identifiers.
- **Agent & RAG Pipeline Friendly**: Directly consumed by agentic frameworks, knowledge graphs, and embedding generators without custom regex parsers.
- **MCP Native Integration**: When invoked via MCP, returns parsed JSON directly in the `data` response object for immediate programmatic consumption by AI agents.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format json \
  --output tests/e2e_fixtures/out/penta_e2e/document.json \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_json",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_json \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.json"
  }'
```

## 87. `pdf_to_markdown`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Decompiles PDF documents into clean, LLM-ready Markdown.**  
Extracts semantic headings, page divisions, bullet lists, inline formatting, code blocks, tables, and text sections into high-fidelity Markdown text.

- **LLM Context Optimization**: Removes page artifacts, repetitive running headers, and noise, outputting structured Markdown suitable for RAG and agent context windows.
- **Page Anchor Preservation**: Identifies page boundaries and inserts logical separators (`# Page 1`, `# Page 2`) for auditability and document referencing.
- **Header & List Detection**: Infers semantic heading levels (`#`, `##`, `###`, `####`) based on font size differentials and typographic weights.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format markdown \
  --output tests/e2e_fixtures/out/penta_e2e/document.md \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_markdown",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Gateway Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_markdown \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document.md"
  }'
```

## 88. `pdf_to_pdf_a`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF documents into the ISO 19005 archival format (PDF/A-1b).**  
Injects PDF/A conformance metadata, standard OutputIntents, and archival flags into document dictionary headers to ensure permanent, device-independent long-term document preservation and regulatory compliance.

- **ISO 19005 Archival Standard**: Attaches `GTS_PDFXVersion` / `PDF/A-1b` conformance keys to the document `/Info` dictionary.
- **Regulatory & Legal Compliance**: Prepares digital records, legal filings, and municipal archives for compliance audits.
- **Device-Independent Output**: Strips external device dependencies and dynamic actions to guarantee consistent rendering decades into the future.

---

### CLI
```bash
paperpilot pdf-a \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pdf_a",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Gateway Endpoint
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_to_pdf_a \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/document_pdfa.pdf"
  }'
```

## 89. `pdf_to_pptx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Converts PDF document pages into editable Microsoft PowerPoint presentation slides (.pptx).**  
Extracts each PDF page's textual content and structural elements into individual PowerPoint presentation slides. The resulting `.pptx` container packages standard OpenXML relationships, presentation manifest, slide layouts, and content types for full native compatibility with Microsoft PowerPoint, Apple Keynote, and Google Slides.

- **Page-to-Slide Mapping**: Generates exactly one slide per PDF page, mapping extracted page texts to positioned text frame objects.
- **OpenXML Conformance**: Implements standard Open Packaging Conventions (OPC) with slide manifests and MIME content types (`application/vnd.openxmlformats-officedocument.presentationml.slide+xml`).
- **Pipeline Intermediary**: PPTX output files can be fed directly downstream to `office_extract_text` or `office_to_markdown`.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --format pptx \
  --output tests/e2e_fixtures/out/penta_e2e/slides.pptx \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_pptx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/slides.pptx"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Standard Tool API Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_pptx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/slides.pptx"
  }'
```

## 90. `pdf_to_xlsx`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Extracts tabular data and financial statements from PDF into an Excel workbook (.xlsx).**  
Detects table structures, spatial cell boundaries, and textual values to generate clean, editable multi-column spreadsheets.

- **Heuristic Table Boundary Detection**: Identifies aligned multi-column rows and financial tables from spatial coordinate layouts.
- **Direct Workbook Generation**: Compiles extracted cell records directly into modern OpenXML spreadsheet sheets (`.xlsx`).
- **Privacy & Security**: Operates completely in-memory and locally without third-party cloud data transmission.

---

### CLI
```bash
paperpilot convert \
  --input tests/e2e_fixtures/real/financial_tables.pdf \
  --format xlsx \
  --output tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_to_xlsx",
    "arguments": {
      "input": "tests/e2e_fixtures/real/financial_tables.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Universal MCP Tool Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/tools/pdf_to_xlsx \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/financial_tables.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/financial_statement.xlsx"
  }'
```

## 91. `pdf_validate`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Runs deep diagnostic checks on PDF structure and compliance.**  
Validates cross-reference tables, font descriptors, catalog integrity, indirect objects, and identifies syntax errors or corruption warnings.

- **Structural Catalog Audit**: Confirms the presence and validity of the PDF document catalog (`/Root`), page tree nodes (`/Pages`), and object streams.
- **Cross-Reference Table Verification**: Ensures xref offsets and subsection indices point to valid byte offsets in the file.
- **Page Tree Integrity**: Recursively counts and validates every page object dictionary and verifies correct page hierarchy resolution.
- **Dual Output Modes**: Emits concise human-readable verification status or structured machine-readable JSON reports.

---

### CLI
```bash
# Verify document structure with human-readable console output
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf

# Verify document structure with structured JSON response
paperpilot validate --input tests/e2e_fixtures/real/single_page.pdf --json

# Save structured validation report directly to disk
paperpilot validate \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/val_report.json \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_validate",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Route
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_validate \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf"
  }'
```

## 92. `pdf_validate_signatures`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Cryptographically verifies digital signatures, certificate chains, and document integrity.**  
Inspects PDF signature dictionaries (`/ByteRange`, `/SubFilter`, `/Contents`, `/Cert`) to validate signatures and assess whether document contents were altered post-signing.

- **Dual-Layer Forensic Audit**: Verifies PKCS#7 / CMS cryptographic byte ranges and compares content streams against an optional baseline reference PDF (`--baseline`).
- **Tamper Forensics**: Identifies exact page numbers where text or drawing streams were altered after a signature was placed.
- **Detailed JSON Reporting**: Outputs comprehensive reports containing field names, signer identity, signing timestamp, validity status, and incremental update diagnostics.

### CLI
```bash
# Validate signatures in a signed document
paperpilot pdf-validate-signatures \
  --input tests/e2e_fixtures/signed.pdf \
  --output report.json \
  --json

# Forensic audit comparing a signed document against a pre-signed baseline reference
paperpilot pdf-validate-signatures \
  --input tests/e2e_fixtures/signed_unsigned.pdf \
  --baseline tests/e2e_fixtures/unsigned.pdf \
  --output audit_report.json \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "pdf_validate_signatures",
  "arguments": {
    "input": "tests/e2e_fixtures/signed_unsigned.pdf",
    "output": "audit_report.json",
    "baseline": true
  }
}
```

### REST API
```bash
# Dedicated validation endpoint with baseline comparison
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/validate-signatures \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/signed_unsigned.pdf", "output": "audit_report.json", "baseline": "tests/e2e_fixtures/unsigned.pdf"}'

# Dedicated endpoint (using tool name alias)
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/pdf_validate_signatures \
  -H 'Content-Type: application/json' \
  -d '{"input": "tests/e2e_fixtures/signed_unsigned.pdf", "output": "audit_report.json"}'
```

## 93. `pdf_watermark`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Stamps semi-transparent text watermarks across PDF pages.**  
Applies diagonal, centered, or custom angled branding (e.g. `CONFIDENTIAL`, `DRAFT`, `COPYRIGHT`) with customizable font sizes, opacity, rotation angle, and primary color palettes across all document pages.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_watermark",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "text": "CONFIDENTIAL",
      "angle": 45,
      "opacity": 0.3,
      "color": "gray",
      "output": "tests/e2e_fixtures/out/batch3_test/mcp_watermarked.pdf"
    }
  },
  "id": 1
}
```

### REST API
```bash
N/A
```

## 94. `recipe_run`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

Executes a validated declarative document recipe (`.ppflow`, TOML, JSON) or preset against a local PDF.
Uses the unified multi-step workflow engine and writes a durable terminal receipt (`.paperpilot-receipts/run-*.json`) beside the output file.

- **Unified Runner**: Executes the same underlying pipeline stages as [`document_workflow`](/reference/tools/document_workflow/), supporting runtime conditions, dependency branching, and skip heuristics.
- **Durable Receipts**: On completion, generates a cryptographic execution receipt including `job_id`, `receipt_path`, input/output SHA-256 hashes, and per-stage timings.
- **Dynamic Output Naming**: Supports opt-in pattern templating via `--name-template` / `name_template` (e.g. `{input_stem}-{hash:8}.pdf`). When used, `output` specifies the target directory.
- **Failure Preservation**: Existing output documents are never overwritten or corrupted if recipe execution fails midway. Execution failures include the partial stage report and receipt location in error data.

---

### CLI
```bash
# Execute recipe and output JSON report
paperpilot run \
  ManaulTestingFixtures/document_workflow/Input/audit_pipeline.ppflow \
  input.pdf \
  --output out/result.pdf \
  --json
```

### MCP JSON-RPC
```json
{
  "name": "recipe_run",
  "arguments": {
    "input": "/local/input.pdf",
    "recipe": "preset-legal-discovery",
    "output": "/local/out/result.pdf"
  }
}
```

### REST API
```bash
N/A
```

## 95. `semantic_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs structure-aware semantic chunk search and conceptual relevance ranking across PDF documents.**  
Segments documents into semantic context chunks (respecting document headings, paragraph bounds, and token overlap limits), computes token overlap and exact phrase relevance scores (`0.0` - `1.0`), and ranks the most relevant excerpts.

---

### CLI
```bash
paperpilot semantic-search \
  --dir tests/e2e_fixtures/real \
  --query "revenue" \
  --limit 3 \
  --output /tmp/paperpilot_test/sem_search.json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "semantic_search",
    "arguments": {
      "input": "tests/e2e_fixtures/real",
      "query": "revenue trends",
      "limit": 5
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Semantic Search Route
curl -s -X POST http://127.0.0.1:7823/api/v1/search/semantic_search \
  -H "Content-Type: application/json" \
  -d '{
    "input": "tests/e2e_fixtures/real",
    "query": "revenue trends",
    "limit": 3
  }'
```

## 96. `showcase_gif`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Generates an animated showcase mockup card highlighting document content with elegant transitions.**  
Renders PDF document pages into a polished presentation mockup card featuring clean drop shadows, smooth page transitions, and progressive scanning animations. Designed for product marketing, documentation showcases, portfolio highlights, and software demo reels.

---

### CLI
```bash
paperpilot showcase-gif \
  --input tests/e2e_fixtures/real/single_page.pdf \
  --output tests/e2e_fixtures/out/penta_e2e/showcase.gif \
  --fps 4 \
  --width 1200 \
  --transition-frames 10 \
  --json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "showcase_gif",
    "arguments": {
      "input": "tests/e2e_fixtures/real/single_page.pdf",
      "output": "tests/e2e_fixtures/out/penta_e2e/showcase.gif",
      "fps": 4,
      "width": 1200,
      "transition_frames": 10
    }
  },
  "id": 1
}
```

### REST API
```bash
curl -s -X POST http://127.0.0.1:7823/api/v1/pdf/showcase-gif \
  -H 'Content-Type: application/json' \
  -d '{
    "input": "tests/e2e_fixtures/real/single_page.pdf",
    "output": "tests/e2e_fixtures/out/penta_e2e/showcase.gif",
    "fps": 4,
    "width": 1200
  }'
```

## 97. `watcher_list`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Inspects active folder watchers, runtime lifecycle states, and live processing statistics.**  
Queries the in-memory `WatcherManager` registry to return comprehensive telemetry on all registered watchers, including total items processed, success/failure counts, current lifecycle state (`watching`, `stopping`, `stopped`, `error`), and paths to the most recent output and receipt files.

---

### CLI
```bash
# Live status and file transitions stream to stdout/stderr in the terminal:
paperpilot watch --directory ./inbox --recipe preset-legal-discovery
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "watcher_list",
    "arguments": {}
  },
  "id": 3
}
```

### REST API
```bash
curl -s -X GET http://127.0.0.1:7823/v1/automation/watchers
```

## 98. `watcher_start`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Registers an operating-system folder watcher, processing arriving PDFs with validated recipes and durable audit receipts.**  
Monitors a local filesystem directory for dropped or newly written PDF documents. Arriving files are debounced until file sizes stabilize, executed through declarative multi-step recipes or presets, and recorded with cryptographic SHA-256 receipts.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
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

### REST API
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

## 99. `watcher_stop`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Gracefully cancels a running folder watcher after its active PDF operation completes and unregisters it.**  
Signals the worker thread to stop, waits for any in-flight PDF pipeline step and audit receipt generation to finish safely, and removes the watcher instance from the shared manager registry.

---

### CLI
```bash
N/A
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "watcher_stop",
    "arguments": {
      "watcher_id": "watcher_c9b8f2d87e1445bfbb5bcf112a912803"
    }
  },
  "id": 2
}
```

### REST API
```bash
curl -i -X DELETE http://127.0.0.1:7823/v1/automation/watchers/watcher_c9b8f2d87e1445bfbb5bcf112a912803
```

## 100. `workspace_search`

**Status**: ✅ Verified  |  **Format**: PDF / OpenXML / Data

**Performs recursive, multi-format text discovery and snippet extraction across a workspace directory.**  
Recursively traverses folder trees to find matching search terms across PDF documents, Microsoft Word files (`.docx`), Excel spreadsheets (`.xlsx`), Markdown notes (`.md`), plain text (`.txt`), JSON data, and CSV sheets without requiring third-party indexing engines.

---

### CLI
```bash
paperpilot workspace-search \
  --dir tests/e2e_fixtures/real \
  --query "invoice" \
  --formats "pdf,json" \
  --output /tmp/paperpilot_test/ws_search.json
```

### MCP JSON-RPC
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "workspace_search",
    "arguments": {
      "dir": "tests/e2e_fixtures/real",
      "query": "invoice",
      "formats": "pdf,docx,xlsx,md"
    }
  },
  "id": 1
}
```

### REST API
```bash
# Dedicated Search Route
curl -s -X POST http://127.0.0.1:7823/api/v1/search/workspace_search \
  -H 'Content-Type: application/json' \
  -d '{
    "dir": "tests/e2e_fixtures/real",
    "query": "invoice",
    "formats": "pdf,json"
  }'
```

