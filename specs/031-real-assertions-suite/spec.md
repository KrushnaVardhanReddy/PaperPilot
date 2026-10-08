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
| **5.9.4A** | **Crate Foundation & Page Operations (9 Tools)**: `pdf_merge`, `pdf_split`, `pdf_extract_pages`, `pdf_delete_pages`, `pdf_reorder_pages`, `pdf_rotate`, `pdf_crop`, `pdf_burst`, `pdf_remove_blank` | 180 tests | Crate foundation (`tools/penta-interface-e2e`), runner architecture, fixtures generator, lopdf assertions, and 9 page operations cases |
| **5.9.4B** | **Security, Stamping & Forms (14 Tools)**: `pdf_encrypt`, `pdf_decrypt`, `pdf_redact`, `pdf_sign`, `pdf_hash`, `pdf_watermark`, `pdf_header_footer`, `pdf_bates`, `pdf_page_numbers`, `pdf_annotate`, `pdf_read_form`, `pdf_fill_form`, `pdf_flatten`, `pdf_create_form_field` | 280 tests | `src/cases/security_stamping_forms.rs` |
| **5.9.4C** | **Extraction, Analysis & Optimization (13 Tools)**: `pdf_compress`, `pdf_repair`, `pdf_linearize`, `pdf_extract_text`, `pdf_extract_images`, `pdf_search`, `pdf_render`, `pdf_compare`, `pdf_metadata`, `pdf_bookmarks`, `pdf_classify_type`, `pdf_validate`, `pdf_ocr` | 260 tests | `src/cases/analysis_optimization.rs` |
| **5.9.4D** | **Conversions & Unified Scorecard Report (8 Tools)**: `pdf_images_to_pdf`, `pdf_to_pdf_a`, `pdf_to_docx`, `pdf_to_xlsx`, `pdf_to_pptx`, `pdf_convert_html`, `pdf_convert_markdown`, `pdf_convert_excel` | 160 tests | `src/cases/conversions.rs` + Unified 880-test Markdown scorecard in `reports/PENTA_INTERFACE_E2E_REAL_ASSERTIONS_REPORT.md` |

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

## 5. Strict Verification Rules
1. **Test-Only Rule**: Never edit `paperpilot-*` engine crates.
2. **Input Immutability**: All inputs must retain identical SHA-256 after tool runs.
3. **Output Non-Identity**: Output SHA-256 must not match input SHA-256 (never hash output path before running).
4. **No Lazy Bypasses**: Validate exact extracted text and attributes per page; zero `pass` blocks.
