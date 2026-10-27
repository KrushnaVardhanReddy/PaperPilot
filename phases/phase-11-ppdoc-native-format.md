## Phase 8 — `.ppdoc` Native Document Format

**Goal:** Design and implement PaperPilot's own next-generation document format that supersedes PDF as an *authoring* format while keeping PDF as a *render/export target*. This turns PaperPilot from a "PDF tool" into a "document platform."

**Spec:** [`specs/ppdoc-format/spec.md`](specs/ppdoc-format/spec.md)

**Strategic rationale:** The same move that made Figma beat Sketch (`.fig` as source → export anything), LaTeX dominate academia (`.tex` → PDF), and Pandoc indispensable (Markdown → 40 formats). PaperPilot would be the first tool to offer this with native PDF import, offline AI embeddings, and Rust-speed processing.

**Exit condition:** A `.ppdoc` file can be created, edited, round-tripped to JSON, and exported to PDF and HTML with full content fidelity. `pdf2ppdoc` can import a real-world PDF with ≥ 80% structural accuracy.

---

### 8.1 — Format Core (`ppdoc-rs` crate) `[PARALLEL]`

| # | Task | Notes |
|---|---|---|
| 8.1.1 | Define `PpdocNode` enum in Rust | All node types: `Document`, `Section`, `Paragraph`, `Table`, `Figure`, `List`, `CodeBlock`, `Formula`, `Callout`, `Interactive`, `FormField`, `PageBreak` |
| 8.1.2 | Implement ZIP container read/write | Use `zip` crate — manifest, content, layout, assets, metadata, history directories |
| 8.1.3 | `manifest.json` validation | Format version check, file registry, SHA-256 checksums |
| 8.1.4 | JSON serialization via `serde` | `#[derive(Serialize, Deserialize)]` on all node types; stable UUID `id` generation |
| 8.1.5 | `PpdocDocument::query(selector)` | XPath-style node lookup by `id`, `type`, `section`, or custom attributes |
| 8.1.6 | Round-trip test suite | Parse → serialize → parse — content tree must be byte-identical |

---

### 8.2 — Layout Engine (`pplayout`)

| # | Task | Notes |
|---|---|---|
| 8.2.1 | `PplayoutRules` struct | Page size, margins, typography scale, flow mode (`Reflowable` / `Fixed`) |
| 8.2.2 | Breakpoint resolver | Select correct layout rules for `screen` / `mobile` / `print` contexts |
| 8.2.3 | Default layout pack | Ship `default.pplayout` and `print.pplayout` as embedded defaults |
| 8.2.4 | Custom layout loading | Load user-provided `.pplayout` from the document archive |

---

### 8.3 — Export Pipeline

| # | Task | Notes |
|---|---|---|
| 8.3.1 | `ppdoc → PDF` exporter | Walk content tree, emit `lopdf` objects using layout rules; reuse Phase 1 PDF engine |
| 8.3.2 | `ppdoc → HTML` exporter | Emit semantic HTML5 with ARIA roles; inline CSS from layout rules |
| 8.3.3 | `ppdoc → EPUB 3` exporter | Package HTML export into EPUB container; WASM widgets → static fallback images |
| 8.3.4 | `ppdoc → Markdown` exporter | Best-effort flat text; preserve tables (GFM), images (relative paths), headings |
| 8.3.5 | `ppdoc → Plain Text` exporter | Reading-order extraction with clean whitespace |
| 8.3.6 | Export CLI subcommand | `paperpilot ppdoc export --format pdf --input doc.ppdoc --output doc.pdf` |
| 8.3.7 | Hybrid PDF Export (`--embed-source`) | Inspired by LibreOffice Hybrid PDF but **visible**: attach the `.ppdoc` ZIP bundle into the PDF `/EmbeddedFiles` dictionary; inject structured XMP metadata (`ppdoc_version`, `generator`); render a small tasteful "PaperPilot Document" badge in the PDF margin (opt-out flag `--no-badge`). When any resulting `.pdf` is dropped back into PaperPilot, the embedded `.ppdoc` is extracted and opened with full edit history, AI embeddings, and vector source intact. **Viral distribution mechanism** — every shared PDF is a soft funnel for PaperPilot adoption. See `INSPIRATION.md §4`. |

---

### 8.4 — Import Pipeline

| # | Task | Notes |
|---|---|---|
| 8.4.1 | `pdf2ppdoc` converter | Use `lopdf` + heuristics to reconstruct semantic tree from PDF content streams |
| 8.4.2 | Heading detection | Font-size clustering → assign `heading` level 1–6 |
| 8.4.3 | Table detection | Adjacent text-block alignment → `table` node reconstruction |
| 8.4.4 | Figure extraction | Raster image objects → `figure` nodes with extracted alt-text via OCR |
| 8.4.5 | `md2ppdoc` converter | Markdown AST (via `pulldown-cmark`) → `.ppdoc` content tree (lossless for flat docs) |
| 8.4.6 | Import accuracy benchmark | Target: ≥ 80% structural accuracy on a corpus of 100 real-world PDFs |

---

### 8.5 — AI Integration

| # | Task | Notes |
|---|---|---|
| 8.5.1 | `ppdoc-embed` tool | Walk all `section` nodes → run `neuml/bert-hash-nano-embeddings` → write `metadata/embeddings.json` |
| 8.5.2 | Named entity extraction | Run NER pass during embedding → store `named_entities[]` per section |
| 8.5.3 | Instant RAG on open | When PaperPilot opens a `.ppdoc` with `embeddings.json`, load vectors into in-memory index immediately — no re-indexing |
| 8.5.4 | MCP tool: `ppdoc_query` | `ppdoc_query(file, question)` → semantic search over embedded sections → return top-K chunks |

---

### 8.6 — Cryptographic Signatures

| # | Task | Notes |
|---|---|---|
| 8.6.1 | Per-section SHA-256 hashing | Hash canonical JSON of each `section` node → store in `signatures.json` |
| 8.6.2 | Merkle tree construction | Build root hash from section hashes → enables partial verification |
| 8.6.3 | Ed25519 signing | `ppdoc-sign sign --section sec-id --key private.pem` |
| 8.6.4 | Signature verification CLI | `ppdoc-sign verify --input doc.ppdoc` → reports which sections are signed and by whom |
| 8.6.5 | Selective disclosure proof | Verify a single section's signature without reading the full document |

---

### 8.7 — Change History

| # | Task | Notes |
|---|---|---|
| 8.7.1 | NDJSON change log writer | Append `{"ts","op","author","node_id","prev","next"}` entries on every edit |
| 8.7.2 | `ppdoc-diff` CLI | `ppdoc-diff doc_v1.ppdoc doc_v2.ppdoc` → structural diff output (human-readable) |
| 8.7.3 | History replay | Reconstruct any past version of the document from the change log |
| 8.7.4 | History viewer in Desktop app | Timeline UI showing who changed what and when (Phase 4 integration) |

---

### 8.8 — WASM Interactive Widgets

| # | Task | Notes |
|---|---|---|
| 8.8.1 | Widget sandbox spec | WASI Preview 2 (`wasi:io`, `wasi:filesystem` blocked, `wasi:random` allowed) |
| 8.8.2 | Widget host in desktop app | Load `.wasm` from `assets/wasm/`, call `render(props_json) → svg_string` |
| 8.8.3 | Reference widget: bar chart | Pure-Rust WASM widget that renders a bar chart from `table` node data |
| 8.8.4 | Static fallback generation | On export (PDF/EPUB), call widget → capture SVG → embed as static image |
| 8.8.5 | Widget SDK docs | Document the `render` ABI so third parties can build `.ppdoc` widgets |

---

