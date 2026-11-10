# Session 15 QA Scorecard — Tools 71–75 (Extraction, Conversion & OCR Intelligence)

**Branch:** `feature/TESTER_1_FIX`  
**Base Commit:** `317a93c` (Session 14)  
**Session:** 15 of PaperPilot QA Protocol  
**Execution Date:** 2026-11-10  
**Cargo Check:** ✅ PASS (0 errors, 0 warnings for workspace crates)

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Tools in scope | 5 (Tools 71–75: `pdf_to_markdown`, `pdf_to_json`, `pdf_extract_text`, `pdf_extract_tables`, `pdf_ocr`) |
| Tools fully wired | 5 |
| `cargo check` result | ✅ Zero errors |
| UI controls added / updated | 4 new dedicated UI blocks + `tableFormat` state + OCR dual-alias support |
| Rust test stubs added (wave7 MCP) | 9 new interface tests (3 tools × 3 interfaces: CLI, MCP, API) |
| Rust test stubs added (wave5 E2E) | 5 new tool arms in `test_wave5_penta_interface.rs` |
| CLI format expansion | Added `"md" \| "markdown"` and `"html" \| "htm"` normalization in `paperpilot-cli` |
| Playwright suite (`e2e_suite_3`) | Fully verified with existing assertions |

---

## Per-Tool Status

### Tool 71 — `pdf_to_markdown`
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Wired | `input`, `output` (`.md`), with `pdf_convert_markdown` alias |
| `OperationsPanel.svelte` UI | ✅ Added (Session 15) | Dedicated config block: CommonMark format, heading hierarchy, code blocks, tables note |
| Playwright `e2e_suite_3` | ✅ Verified | Validates `# ` heading markdown tokens and document text |
| `wave7_conversions_mcp_tests.rs` | ✅ Already present | Tested as tool #2 with CLI/MCP/API validations |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 15) | Match arm with `convert --format markdown`, `requires_output=false` |
| `paperpilot-cli` | ✅ Fixed (Session 15) | Added `"markdown"` format alias to `group_d::handle_convert` alongside `"md"` |

### Tool 72 — `pdf_to_json`
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Already wired | `input`, `output` (`.json`) |
| `OperationsPanel.svelte` UI | ✅ Added (Session 15) | Dedicated config block: structured JSON hierarchy, text blocks, coordinates, font metadata |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts valid JSON structure and parsed layout tree |
| `wave7_conversions_mcp_tests.rs` | ✅ Already present | Tested as tool #3 with CLI/MCP/API validations |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 15) | Match arm with `convert --format json`, `requires_output=false` |

### Tool 73 — `pdf_extract_text`
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Added (Session 15) | Added `case 'pdf_extract_text':` alias alongside `extract_text` |
| `OperationsPanel.svelte` UI | ✅ Added (Session 15) | Dedicated config block: raw reading-order extraction, unicode font cmap normalization, `.txt` output |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts extracted plain text matches document keyword invariants |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 15) | Added CLI (`extract-text`), MCP (`pdf_extract_text`), and API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 15) | Match arm with `extract-text --input ... --output ... --json` |

### Tool 74 — `pdf_extract_tables`
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Already wired | `input`, `output` (`.json`/`.csv`), `format` from `context.tableFormat` |
| `OperationsPanel.svelte` UI | ✅ Added (Session 15) | Dedicated config block with format selector (`json`/`csv`), table grid detection note, dynamic extensions/filters |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts tabular matrix dimensions (rows > 0, cols > 0) and financial fields |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 15) | Added CLI (`extract-tables`), MCP (`pdf_extract_tables`), and API stubs |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 15) | Match arm with `extract-tables --format csv --pages all` |

### Tool 75 — `pdf_ocr`
| Layer | Status | Notes |
|-------|--------|-------|
| `operationArgsBuilder.ts` | ✅ Added (Session 15) | Added `case 'pdf_ocr':` alias alongside `ocr` |
| `OperationsPanel.svelte` UI | ✅ Updated (Session 15) | Dual-alias support (`ocr` / `pdf_ocr`) across destination picker, clipboard copy, model download banner, run button |
| Playwright `e2e_suite_3` | ✅ Verified | Asserts searchable text layer injection and document text inspection |
| `wave7_conversions_mcp_tests.rs` | ✅ Added (Session 15) | Added CLI (`ocr`), MCP (`pdf_ocr`), and API stubs with `%PDF-` validation |
| `test_wave5_penta_interface.rs` | ✅ Added (Session 15) | Match arm with `ocr --input ... --output ...`, `requires_output=true` validating `%PDF-` magic header |

---

## Files Modified

| File | Change Description |
|------|--------------------|
| `apps/desktop/src/lib/services/operationArgsBuilder.ts` | Added `pdf_extract_text`, `pdf_ocr`, and `pdf_convert_markdown` aliases |
| `apps/desktop/src/lib/components/layout/OperationsPanel.svelte` | Added dedicated UI config blocks for `pdf_to_markdown`, `pdf_to_json`, `pdf_extract_text`, `pdf_extract_tables`; added `tableFormat` state; wired `pdf_ocr` alias in model download check, clipboard copy, output path generator, and run buttons |
| `paperpilot-cli/src/commands/group_d.rs` | Normalized format parsing to accept `"markdown"` alongside `"md"`, and `"htm"` alongside `"html"` |
| `paperpilot-mcp/tests/wave7_conversions_mcp_tests.rs` | Added 9 new test blocks covering CLI, MCP, and API interfaces for Tools 73–75 (`pdf_extract_text`, `pdf_extract_tables`, `pdf_ocr`) |
| `tests/e2e/test_wave5_penta_interface.rs` | Added Tools 71–75 to test suite vector and implemented 5 dedicated penta-interface match arms |
| `reports/WAVE_15_SESSION_15_SCORECARD.md` | This scorecard |

---

## `cargo check` Result

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
```

Zero errors. All workspace crates compile cleanly.

---

## Interface Coverage Matrix

| Tool | CLI Command | MCP Payload | REST API Endpoint | Desktop UI | Playwright E2E |
|------|-------------|-------------|-------------------|------------|----------------|
| `pdf_to_markdown` | `convert --format markdown` | `{input, output}` | `/api/v1/pdf/tools/pdf_to_markdown` | ✅ Dedicated block | ✅ `# ` heading tokens |
| `pdf_to_json` | `convert --format json` | `{input, output}` | `/api/v1/pdf/tools/pdf_to_json` | ✅ Dedicated block | ✅ Valid JSON layout |
| `pdf_extract_text` | `extract-text` | `{input, output}` | `/api/v1/pdf/tools/pdf_extract_text` | ✅ Dedicated block | ✅ Plain text reading order |
| `pdf_extract_tables` | `extract-tables --format csv` | `{input, output, format, pages}` | `/api/v1/pdf/tools/pdf_extract_tables` | ✅ Format selector (`json`/`csv`) | ✅ Rows/cols & financial keys |
| `pdf_ocr` | `ocr` | `{input, output}` | `/api/v1/pdf/tools/pdf_ocr` | ✅ Destination selector + model downloader | ✅ Searchable text layer |
