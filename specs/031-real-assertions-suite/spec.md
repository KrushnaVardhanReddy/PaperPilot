# Spec 031: Rust-Native Penta-Interface Real Semantic Assertions Suite (`tools/penta-interface-e2e`)

## Status: APPROVED / READY FOR IMPLEMENTATION

## 1. Problem Statement
The previous test scripts relied on partial interface coverage or shallow verification:
- Checks only if the process exits with code 0.
- Lacked complete coverage across the full 5-interface deployment topology.
- Test harness bugs (output identity collisions, lazy `pass` blocks, path resolution issues).
- Incomplete coverage across complexity tiers (Simple, Medium, Complex, Negative).

## 2. Objective & Architecture
Build a **100% pure-Rust, zero-Python modular E2E crate** in `tools/penta-interface-e2e/`.

The suite verifies all **44 tools** across all **5 PaperPilot Interfaces**:
1. **💻 CLI**: Invoking `target/debug/paperpilot-cli` (or release) subprocess.
2. **🤖 MCP**: Stdio JSON-RPC 2.0 communication with `target/debug/paperpilot-mcp`.
3. **🌐 REST API**: Spawning `paperpilot serve --port 7823` in background with `/health` polling.
4. **⚡ WASM (Browser)**: `WasmPdfEngine.<op>` parity schema and semantic assertion.
5. **☁️ Cloudflare Edge**: `POST /api/v1/<op>` endpoint parity and semantic assertion.

### 4-Tier Test Matrix per Tool:
Every tool must be tested across:
- **Tier 1 (Simple)**: Minimal valid input (1-2 pages, simple ASCII text).
- **Tier 2 (Medium)**: Multi-page document with structured layout (5 pages, custom ranges, rotations).
- **Tier 3 (Complex / Hard)**: Stress test documents (tables, 3-way merges, non-contiguous ranges, unicode).
- **Negative Case**: Intentionally invalid input asserting non-zero exit code or structured JSON error.

**Total Scope**: 44 tools × 5 interfaces × 4 tiers = **880 verified test executions**.

---

## 3. Four-Phase Sequential Split (Zero Merge Conflicts)

To prevent session timeouts or context truncation in Jules, the implementation is split into 4 sequential subtasks:

| Subtask | Scope & Tools | Tests Count | Deliverable |
|---|---|---|---|
| **5.9.4A** | **Crate Foundation & Page Operations (9 Tools)**: `pdf_merge`, `pdf_split`, `pdf_extract_pages`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_rotate`, `pdf_crop`, `pdf_burst`, `pdf_remove_blank` | 180 tests | Crate foundation (`tools/penta-interface-e2e`), runner architecture, fixtures generator, lopdf assertions, 9 page operations cases, and `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4A.md` |
| **5.9.4B** | **Security, Stamping & Forms (14 Tools)**: `pdf_encrypt`, `pdf_decrypt`, `pdf_redact`, `pdf_sign`, `pdf_hash`, `pdf_watermark`, `pdf_header_footer`, `pdf_bates`, `pdf_page_numbers`, `pdf_annotate`, `pdf_read_form`, `pdf_fill_form`, `pdf_flatten`, `pdf_create_form_field` | 280 tests | `src/cases/security_stamping_forms.rs` and `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4B.md` |
| **5.9.4C** | **Extraction, Analysis & Optimization (13 Tools)**: `pdf_compress`, `pdf_repair`, `pdf_linearize`, `pdf_extract_text`, `pdf_extract_images`, `pdf_search`, `pdf_render`, `pdf_compare`, `pdf_metadata`, `pdf_bookmarks`, `pdf_classify_type`, `pdf_validate`, `pdf_ocr` | 260 tests | `src/cases/analysis_optimization.rs` and `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT_5_9_4C.md` |
| **5.9.4D** | **Conversions & Unified Scorecard Report (8 Tools)**: `pdf_images_to_pdf`, `pdf_to_pdf_a`, `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`, `pdf_convert_html`, `pdf_convert_markdown`, `pdf_convert_excel` | 160 tests | `src/cases/conversions.rs` + Unified master 880-test Markdown scorecard in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md` |

---

## 4. Crate Architecture (`tools/penta-interface-e2e`)

```
tools/penta-interface-e2e/
├── Cargo.toml
└── src/
    ├── main.rs                   # CLI runner & Markdown report generator
    ├── runner/
    │   ├── mod.rs                # Runner traits & types (ExecutionResult, InterfaceType)
    │   ├── cli.rs                # Executes paperpilot-cli binary with args
    │   ├── mcp.rs                # Spawns paperpilot-mcp stdio, sends JSON-RPC 2.0 tools/call
    │   ├── api.rs                # Spawns Gateway, polls /health, executes reqwest calls
    │   └── targets.rs            # WASM & Cloudflare Edge schema & parity metadata
    ├── fixtures/
    │   ├── mod.rs                # Programmatic fixture manager & generator
    │   ├── pdf.rs                # Generates minimal, multi-page, formatted PDFs (lopdf / raw bytes)
    │   └── office.rs             # Generates test CSV, HTML, Markdown, image fixtures
    ├── assertions/
    │   ├── mod.rs                # AssertionEngine trait & dispatcher
    │   ├── pdf.rs                # Deep PDF assertions: page count, extract text per page, rotation, bates, encrypt
    │   ├── image.rs              # Image dimensions, format magic bytes
    │   └── json.rs               # Schema, field existence, hash comparison
    └── cases/
        ├── mod.rs                # Registry of all 44 tools (ToolTestCase trait)
        ├── page_ops.rs           # (Part A) 9 tools
        ├── security_forms.rs     # (Part B) 14 tools
        ├── analysis.rs           # (Part C) 13 tools
        └── conversions.rs        # (Part D) 8 tools
```

---

## 6. Phase 5.9.5 — Penta-Interface Edge Cases & Deep Boundary Verification Suite

### 6.1 Objective
Extend the test verification in `tools/penta-interface-e2e` to stress-test deep boundary conditions and hostile inputs across all **44 tools** and all **5 interfaces**. This guarantees that the engine handles extreme inputs deterministically without panicking, crashing, or corrupting state.

### 6.2 Tool Category Edge Case Matrix (All 44 Tools)

#### Group 1: Page Operations (9 Tools)
- `pdf_merge`: Empty input array (`[]`), single input file, duplicate identical input files in list, merging with zero-byte corrupted PDF.
- `pdf_split`: Chunk size larger than total page count, chunk size 1 (equivalent to burst), negative/zero chunk size.
- `pdf_extract_pages`: Discontinuous range (`"1,3,5"`), reversed range (`"5,4,3,2,1"`), single-page range (`"3"`), out-of-bounds page (`"99"` on 5-page PDF -> structured error).
- `pdf_delete_pages`: Deleting page 1, deleting all pages except 1, attempting to delete ALL pages (`"1-5"` -> structured error: cannot produce 0-page PDF).
- `pdf_reorder_pages`: Identity permutation (`"1,2,3,4,5"`), inverted reverse order (`"5,4,3,2,1"`), single page permutation, invalid page index in order.
- `pdf_rotate`: 360°/720° rotations (full circle invariant), negative degrees (`-90°` -> 270°), rotating 1 page in a 10-page document.
- `pdf_crop`: Microscopic bounding box (`x=10, y=10, w=1, h=1`), full-bleed crop equal to MediaBox (`0,0,612,792`), crop outside page boundaries.
- `pdf_burst`: 1-page document burst (creates exactly 1 output file), burst on 10-page document verifying all 10 files.
- `pdf_remove_blank`: Document with 0 blank pages (identity output), document with all blank pages, threshold sensitivity boundary (`0.001` vs `0.999`).

#### Group 2: Security, Stamping & Forms (14 Tools)
- `pdf_encrypt`: Password containing shell escapes and unicode symbols (`P@$$w0rd!#%^&*()'"\ 🚀`), empty password.
- `pdf_decrypt`: Decrypting already unencrypted PDF (noop or error), wrong password on encrypted file (graceful authentication error).
- `pdf_redact`: Zero-dimension rect, full-page redaction (`0,0,612,792`), redaction on non-existent page number.
- `pdf_sign`: Signature timestamping without network (offline fallback), signing already signed PDF (incremental signature check).
- `pdf_hash`: Verifying deterministic SHA-256 calculation for identical bytes, hash of empty / 1-page PDF.
- `pdf_watermark`: Unicode / CJK / Emoji watermark text (`"CONFIDENTIAL 🔒 机密 Éléphant"`), 0 opacity vs 1.0 opacity, 0° vs 45° rotation.
- `pdf_header_footer`: Long header text exceeding page width, multi-byte UTF-8, formatting tokens (`{page} of {total}`).
- `pdf_bates`: High start number (`start: 999999`), prefix with symbols (`"LEGAL-CONF-$#@"`), padding 10.
- `pdf_page_numbers`: Position boundaries (`top-left`, `top-center`, `top-right`, `bottom-left`, `bottom-center`, `bottom-right`), 1-page PDF numbering.
- `pdf_annotate`: Multiple overlapping annotations, annotation on page outside range, empty content.
- `pdf_read_form`: PDF with zero form fields (returns empty object `{}` cleanly, not null/crash), corrupted AcroForm dictionary.
- `pdf_fill_form`: Filling field that does not exist in document (ignored or error, no panic), empty values map `{}`.
- `pdf_flatten`: Flattening PDF without interactive form fields / annotations (returns valid PDF identical in appearance).
- `pdf_create_form_field`: Field at margin boundary (x=0, y=0), duplicate field name creation.

#### Group 3: Extraction, Analysis & Optimization (13 Tools)
- `pdf_compress`: Already-compressed PDF (assert output size <= input size without corruption), quality=1 vs quality=100.
- `pdf_repair`: Clean PDF (repair should keep document intact), truncated PDF missing `%%EOF` (recovers valid streams).
- `pdf_linearize`: Linearizing already-linearized PDF, linearizing encrypted PDF.
- `pdf_extract_text`: Extracting text from image-only / empty PDF (returns empty string cleanly, not crash), extraction with RTL or ligatures.
- `pdf_extract_images`: PDF with 0 embedded images (returns empty list / creates 0 images), extracting from multi-image page.
- `pdf_search`: Searching for term not in document (returns empty matches list `[]`), regex / case-sensitivity flags, unicode search term.
- `pdf_render`: Rendering page 1 at extreme DPI (72 DPI vs 300 DPI), rendering out-of-bounds page (fails cleanly).
- `pdf_compare`: Comparing identical file against itself (similarity score 1.0 / 0 diffs), comparing completely different files.
- `pdf_metadata`: Setting and reading unicode metadata with emojis in Title/Author, reading metadata from PDF with empty Info dictionary.
- `pdf_bookmarks`: Reading bookmarks on PDF without outlines (returns empty list), adding nested 3-level bookmark outline.
- `pdf_classify_type`: Classifying scanned vs digital vs hybrid PDF, classifying 1-page vs 10-page document.
- `pdf_validate`: Validating valid PDF (passes), validating file with invalid magic bytes (`not a pdf`) -> structured error report.
- `pdf_ocr`: OCR on digitally generated text PDF, OCR on blank white page.

#### Group 4: Conversions (8 Tools)
- `pdf_images_to_pdf`: Multiple images with different aspect ratios/orientations, 1 single image.
- `pdf_to_pdf_a`: Converting PDF with embedded forms and transparencies to PDF/A-1b.
- `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`: Exporting text with tables to office formats.
- `pdf_convert_html`: HTML with embedded CSS, flexbox layout, and unicode characters.
- `pdf_convert_markdown`: Markdown with deep headers, tables, code blocks, and blockquotes.
- `pdf_convert_excel`: Multi-row tabular data conversion to PDF.

### 6.3 Test Invariants & Safety Guardrails
1. **Zero Panic Rule**: No process invocation or API request may exit with panic, `SIGSEGV`, or unhandled Rust unwrap.
2. **Deterministic Status Codes**:
   - Out-of-bounds or malformed requests must return structured error exit codes (CLI code `1` or `2`, REST `400 Bad Request` or `422`).
3. **No Mock Overrides**:
   - Every edge case must assert the real exit code or JSON error payload.
4. **Execution & Report Output**:
   - `cargo run -p penta-interface-e2e -- --group edge_cases` will run the edge verification suite and generate `reports/PENTA_INTERFACE_E2E_EDGE_CASES_REPORT.md`.
