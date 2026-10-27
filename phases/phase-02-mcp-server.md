## Phase 2 — MCP Server

**Goal:** PaperPilot exposes all PDF operations as strongly typed MCP tools that AI agents can call.

**Exit condition:** An external AI agent (Claude, GPT, etc.) can use PaperPilot via MCP to merge, split, compress, and extract text from PDFs without any custom integration code.

**[Phase 1 exit condition: ✅ MET — all operations implemented, tested, and CLI working]**

---

### 2.1 — MCP Foundation `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 2.1.1 | Add `rmcp` crate dependency ✅ | Merged (Session 8555100062784875086) |
| 2.1.2 | Create `paperpilot-mcp` crate ✅ | Merged (Session 8555100062784875086) |
| 2.1.3 | Design tool schema conventions ✅ | Merged (Session 8555100062784875086) |

---

### 2.2 — Tool Implementations `[PARALLEL within group, BLOCKED BY 2.1]`

Each MCP tool wraps the corresponding Phase 1 Rust operation.

| # | Task | Notes |
|---|---|---|
| 2.2.1 | `pdf_merge` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.2 | `pdf_split` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.3 | `pdf_extract_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.4 | `pdf_delete_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.5 | `pdf_reorder_pages` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.6 | `pdf_rotate` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.7 | `pdf_crop` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.8 | `pdf_burst` tool ✅ | Merged (Session 2435568152568408465) |
| 2.2.9 | `pdf_compress` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.10 | `pdf_extract_text` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.11 | `pdf_extract_images` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.12 | `pdf_search` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.13 | `pdf_sign` tool ✅ | Merged |
| 2.2.14 | `pdf_watermark` tool ✅ | Merged (Session 5942899190760144396) |
| 2.2.15 | `pdf_encrypt` / `pdf_decrypt` tools ✅ | Merged (Session 5942899190760144396) |
| 2.2.16 | `pdf_metadata` tool ✅ | Merged (Session 5942899190760144396) |

---

### 2.4 — MCP Tool Backfill `[PARALLEL with 2.2, BLOCKED BY respective Phase 1 ops]`

> These tools were added during Phase 1 sessions **after** 2.2 was originally written.
> They are split into groups matching who will implement the MCP wiring.

#### 2.4a — Group B Missing Tools ✅
| # | Task | Notes |
|---|---|---|
| 2.4.1 | `pdf_repair` tool ✅ | Merged |
| 2.4.2 | `pdf_linearize` tool ✅ | Merged |
| 2.4.3 | `pdf_redact` tool ✅ | Merged |
| 2.4.4 | `pdf_flatten` tool ✅ | Merged |
| 2.4.5 | `pdf_to_pdf_a` tool ✅ | Merged |
| 2.4.6 | `pdf_header_footer` tool ✅ | Merged |
| 2.4.7 | `pdf_bates` tool ✅ | Merged |
| 2.4.8 | `pdf_integrity_hash` tool ✅ | Merged |

#### 2.4b — Group C Missing Tools ✅
| # | Task | Notes |
|---|---|---|
| 2.4.9 | `pdf_render` tool ✅ | Merged |
| 2.4.10 | `pdf_images_to_pdf` tool ✅ | Merged |
| 2.4.11 | `pdf_compare` tool ✅ | Merged |
| 2.4.12 | `pdf_bookmarks` tool ✅ | Merged |
| 2.4.13 | `pdf_ocr` tool ✅ | Merged |

#### 2.4c — Group F (PDF Creation) Missing Tools (needs new Jules session)
| # | Task | Notes |
|---|---|---|
| 2.4.14 | `pdf_markdown_to_pdf` tool | Wraps `MarkdownToPdf` from 1.3.40 |
| 2.4.15 | `pdf_html_to_pdf` tool | Wraps `HtmlToPdf` from 1.3.41 |
| 2.4.16 | `pdf_create_from_template` tool | Wraps template engine from 1.3.42/1.3.43 |

#### 2.4d — Group D Conversion Tools
| # | Task | Notes |
|---|---|---|
| 2.4.17 | `pdf_to_html` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.18 | `pdf_to_markdown` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.19 | `pdf_to_json` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.20 | `pdf_to_llm_export` tool 🚧 | Already registered in PR #37 (Session 10906031722199501492) |
| 2.4.21 | `pdf_to_docx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.22 | `pdf_to_xlsx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.23 | `pdf_to_pptx` tool ✅ | Merged (Session 8203167166999327475) |
| 2.4.24 | `pdf_classify_type` tool ✅ | Merged (Session 8203167166999327475) |

#### 2.4e — Group E Forms Tools (wired by active Jules session 996394326205831326)
| # | Task | Notes |
|---|---|---|
| 2.4.25 | `pdf_read_form` tool ✅ | Merged (Session 996394326205831326) |
| 2.4.26 | `pdf_fill_form` tool ✅ | Merged (Session 996394326205831326) |
| 2.4.27 | `pdf_create_form_field` tool ✅ | Merged (Session 996394326205831326) |

---

### 2.3 — MCP Server Infrastructure `[PARALLEL with 2.2]`

| # | Task | Notes |
|---|---|---|
| 2.3.1 | Input validation layer ✅ | Merged (Session 9931121574944478405) |
| 2.3.2 | Error response format ✅ | Merged (Session 9931121574944478405) |
| 2.3.3 | File path / temp file handling ✅ | Merged (Session 9931121574944478405) |
| 2.3.4 | MCP server binary ✅ | Merged (Session 9931121574944478405) |
| 2.3.5 | MCP schema tests ✅ | Merged (Session 9931121574944478405) |
| 2.3.6 | End-to-end MCP tests ✅ | Merged (Session 9931121574944478405) |
| 2.3.7 | E2E MCP tests (Strict) ✅ | Merged (Session 9931121574944478405) |
| 2.3.8 | SDK Architecture Lock (`rmcp` vs `pmcp`) ✅ | **Architectural Decision (Locked):** Retain official Anthropic `rmcp` 3.5.0 SDK for guaranteed host compatibility (Claude Desktop, Cursor, Goose). Evaluated `pmcp` (zero-cost alternative); deferred to Phase 7.1 Cloud gateway where high-concurrency microsecond serialization matters. |

---

### 2.5 — OpenAPI 3.1 & Embedded Swagger UI for Gateway

> **Stirling-PDF Parity & Developer Integration**: Exposes interactive Swagger UI at `/swagger-ui` and the OpenAPI 3.1 JSON specification at `/api-docs/openapi.json` for all REST endpoints and MCP dispatchers.

| # | Task | Notes |
|---|---|---|
| 2.5.1 | `utoipa` & `utoipa-swagger-ui` Integration | Add dependencies to `paperpilot-gateway` and define root `ApiDoc` |
| 2.5.2 | REST & MCP Endpoint Annotations | Annotate all 14+ endpoints (`/health`, `/merge`, `/split`, `/compress`, `/tools/{tool_name}`) with `#[utoipa::path]` |
| 2.5.3 | Schema Models Definition | Add `#[derive(ToSchema)]` to all request/response payload structs |
| 2.5.4 | Embedded Swagger UI Route | Mount `/swagger-ui` and `/api-docs/openapi.json` in Axum router |
| 2.5.5 | OpenAPI JSON Spec Export | Export static `docs/api/openapi.json` for external client SDK generation |

---

