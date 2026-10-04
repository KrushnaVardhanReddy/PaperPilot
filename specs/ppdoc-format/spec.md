# `.ppdoc` — PaperPilot Native Document Format Specification

> **Status:** Draft v0.1 — Not yet implemented. Tracked under Phase 8 of PHASES.md.
> **Authors:** PaperPilot Core Team
> **License:** Apache 2.0 (format spec itself is open)

---

## 1. Why a New Format?

PDF was designed in 1993 as a **print-faithful, device-independent page description language**.
It solved the problem of the era: "make it look the same everywhere."

The modern reality is different:

| Modern Need | PDF's Answer | Why It Fails |
|---|---|---|
| Mobile / responsive reading | ❌ None | Fixed pixel coordinates break on small screens |
| Semantic structure for AI/LLMs | ❌ Heuristic-only | Text stored in render order, not reading order |
| Real-time collaboration | ❌ Bolt-on (annotations layer) | Comments tied to coordinates, not content IDs |
| Differential sync / versioning | ❌ None | Full-file replacement only |
| Interactive compute (formulas, charts) | ❌ Proprietary JS (Acrobat-only) | Non-portable, security nightmare |
| Accessible by default | ❌ Tagged PDF (optional, ignored) | Rarely implemented correctly |
| Queryable / structured data | ❌ None | Tables are just positioned text glyphs |
| Streaming / progressive load | ❌ Linearized PDF (rare) | Complex, rarely produced correctly |

`.ppdoc` is designed to fix all of the above while:
- Remaining **100% offline** (no cloud dependency)
- Being **exportable to PDF** for legacy compatibility
- Staying **open and royalty-free**

---

## 2. Core Design Principles

1. **Semantic First** — Every element carries meaning, not just appearance.
2. **Layout is a View** — Content and layout are separate concerns, like HTML + CSS.
3. **AI-Native** — Embedding vectors, chunk hints, and named entities are first-class fields.
4. **Diffable by Design** — The format is structured so `git diff` and delta-sync work naturally.
5. **Portable Compute** — Interactive elements use sandboxed WASM modules, not JavaScript.
6. **Crypto-Auditable** — Per-section Merkle hashing allows partial verification and signing.
7. **Export Agnostic** — One source, many targets: PDF, EPUB, HTML, Markdown, DOCX.

---

## 3. File Structure

A `.ppdoc` file is a **ZIP archive** (like `.docx`, `.epub`, `.apk`) with a defined internal layout:

```
document.ppdoc/          (ZIP container)
├── manifest.json        # File registry, format version, checksums
├── content/
│   ├── document.json    # The semantic content tree (primary source of truth)
│   └── sections/        # Optional: large docs split per-section for streaming
│       ├── sec_001.json
│       └── sec_002.json
├── layout/
│   ├── default.pplayout # Default layout rules (JSON — like CSS for documents)
│   └── print.pplayout   # Print-specific layout overrides
├── assets/
│   ├── images/          # Original-resolution images (WebP preferred)
│   ├── fonts/           # Only subset glyphs actually used
│   └── wasm/            # Sandboxed compute modules (.wasm)
├── metadata/
│   ├── meta.json        # Title, author, language, creation date, tags
│   ├── embeddings.json  # AI embeddings per section (optional, for RAG)
│   └── signatures.json  # Cryptographic signatures (optional)
└── history/
    └── changes.ndjson   # Append-only newline-delimited change log (optional)
```

---

## 4. Content Model (`content/document.json`)

The content tree is a **JSON document** following a recursive node model.

### 4.1 Top-Level Schema

```json
{
  "ppdoc_version": "0.1",
  "root": {
    "type": "document",
    "id": "doc-root",
    "lang": "en-US",
    "reading_direction": "ltr",
    "children": []
  }
}
```

### 4.2 Node Types

Every node has a **required** `type` and `id` field. `id` values are stable UUIDs — they survive edits and re-exports.

| Node Type | Description | Key Fields |
|---|---|---|
| `document` | Root container | `lang`, `reading_direction` |
| `section` | Logical document section | `heading`, `level` (1–6), `label` |
| `paragraph` | Text block | `inline[]`, `align`, `indent` |
| `heading` | Section title | `level` (1–6), `text`, `anchor` |
| `table` | Structured grid | `headers[]`, `rows[][]`, `caption` |
| `figure` | Image or diagram | `src` (asset ref), `alt`, `caption` |
| `list` | Ordered or unordered | `ordered` (bool), `items[]` |
| `code_block` | Preformatted code | `language`, `content` |
| `formula` | Math expression | `latex`, `rendered_svg` (cached) |
| `callout` | Note/warning/tip | `variant` (note/warning/tip/caution) |
| `interactive` | WASM compute widget | `wasm_ref` (asset ref), `fallback_img` |
| `page_break` | Forced page break (print only) | — |
| `form_field` | Fillable field | `field_type`, `name`, `default`, `required` |

### 4.3 Inline Runs (Inside Paragraphs)

```json
{
  "type": "paragraph",
  "id": "p-001",
  "inline": [
    { "text": "This is ", "bold": false, "italic": false },
    { "text": "important", "bold": true, "italic": false, "color": "#c0392b" },
    { "text": " text.", "bold": false, "italic": false }
  ]
}
```

### 4.4 Full Section Example

```json
{
  "type": "section",
  "id": "sec-financial-summary",
  "heading": { "level": 2, "text": "Financial Summary", "anchor": "financial-summary" },
  "children": [
    {
      "type": "paragraph",
      "id": "p-fin-001",
      "inline": [
        { "text": "Revenue for Q3 was " },
        { "text": "$4.2M", "bold": true },
        { "text": ", up 18% YoY." }
      ]
    },
    {
      "type": "table",
      "id": "tbl-fin-001",
      "caption": "Q3 Revenue Breakdown",
      "headers": ["Product", "Revenue", "Growth"],
      "rows": [
        ["PaperPilot Pro", "$2.1M", "+22%"],
        ["PaperPilot Cloud", "$2.1M", "+14%"]
      ]
    }
  ]
}
```

---

## 5. Layout Model (`layout/default.pplayout`)

Layout is **separate from content** — the same content tree can be rendered with different layouts.

```json
{
  "pplayout_version": "0.1",
  "page": {
    "size": "A4",
    "orientation": "portrait",
    "margin": { "top": "25mm", "right": "20mm", "bottom": "25mm", "left": "20mm" }
  },
  "typography": {
    "base_font": "Inter",
    "base_size": "11pt",
    "line_height": 1.6,
    "heading_scale": [2.0, 1.6, 1.3, 1.1, 1.0, 0.9]
  },
  "flow": "reflowable",
  "color_scheme": "light",
  "breakpoints": {
    "mobile": { "flow": "reflowable", "base_size": "14pt" },
    "print":  { "flow": "fixed", "color_scheme": "print" }
  }
}
```

`flow` values:
- `reflowable` — like EPUB; adapts to screen size
- `fixed` — like PDF; pixel-perfect positioning

---

## 6. Metadata (`metadata/meta.json`)

```json
{
  "title": "Q3 Financial Report 2026",
  "authors": ["Jane Smith", "John Doe"],
  "created_at": "2026-10-04T12:00:00Z",
  "modified_at": "2026-10-04T14:30:00Z",
  "language": "en-US",
  "tags": ["finance", "quarterly", "confidential"],
  "subject": "Internal financial summary for Q3 FY2026",
  "keywords": ["revenue", "growth", "product"],
  "custom": {
    "department": "Finance",
    "document_id": "FIN-2026-Q3-001"
  }
}
```

---

## 7. AI Embeddings (`metadata/embeddings.json`)

Embeddings are **optional but first-class**. When present, they turn any `.ppdoc` into a
RAG-ready knowledge artifact — no re-processing needed.

```json
{
  "model": "neuml/bert-hash-nano-embeddings",
  "model_version": "1.0",
  "dimensions": 128,
  "generated_at": "2026-10-04T14:30:00Z",
  "sections": [
    {
      "section_id": "sec-financial-summary",
      "chunk_text": "Revenue for Q3 was $4.2M, up 18% YoY...",
      "vector": [0.021, -0.134, 0.087],
      "named_entities": [
        { "text": "$4.2M", "type": "MONEY" },
        { "text": "Q3", "type": "DATE" }
      ]
    }
  ]
}
```

When PaperPilot opens a `.ppdoc`, it can instantly answer semantic queries without re-indexing.

---

## 8. Cryptographic Signatures (`metadata/signatures.json`)

Unlike PDF's whole-document signing, `.ppdoc` supports **per-section Merkle signing**.

```json
{
  "algorithm": "Ed25519",
  "tree": [
    {
      "node_id": "sec-financial-summary",
      "hash": "sha256:abc123...",
      "signature": "ed25519:xyz789...",
      "signer": "jane.smith@acme.com",
      "signed_at": "2026-10-04T15:00:00Z"
    }
  ],
  "root_hash": "sha256:def456..."
}
```

**Key benefit:** You can **prove section 3 is signed without revealing section 5** — impossible with PDF today.

---

## 9. Change History (`history/changes.ndjson`)

Append-only NDJSON log — like a Git commit log inside the file.

```ndjson
{"ts":"2026-10-04T12:00:00Z","op":"create","author":"jane@acme.com","node_id":"doc-root"}
{"ts":"2026-10-04T13:10:00Z","op":"edit","author":"john@acme.com","node_id":"p-fin-001","prev":"Revenue for Q3...","next":"Revenue for Q3 was $4.2M..."}
{"ts":"2026-10-04T14:30:00Z","op":"sign","author":"jane@acme.com","node_id":"sec-financial-summary"}
```

---

## 10. Interactive Widgets (WASM Compute)

Nodes of type `interactive` embed sandboxed `.wasm` modules. These can render:
- Live formula fields (like Excel)
- Interactive charts (no server needed)
- Custom calculators

```json
{
  "type": "interactive",
  "id": "widget-revenue-chart",
  "wasm_ref": "assets/wasm/revenue_chart.wasm",
  "fallback_img": "assets/images/revenue_chart_static.webp",
  "props": {
    "data_source": "tbl-fin-001",
    "chart_type": "bar"
  }
}
```

The sandbox runs WASM with **no network access, no filesystem access** — pure function: `props → render`.

---

## 11. Export Targets

From one `.ppdoc` source, PaperPilot produces:

| Target | Fidelity | Notes |
|---|---|---|
| **PDF** | ✅ High | Fixed layout via `pdfium`/`lopdf` render pass |
| **HTML** | ✅ High | Semantic, accessible, embeddable |
| **EPUB 3** | ✅ High | Reflowable reading; WASM widgets converted to static |
| **Markdown** | ⚠️ Medium | Flat text; tables and images preserved, layout lost |
| **DOCX** | ⚠️ Medium | Via OOXML bridge; complex layouts approximate only |
| **Plain Text** | ✅ Good | Reading-order extraction, clean whitespace |
| **JSON** | ✅ Perfect | The content tree itself — lossless round-trip |

---

## 12. Tooling Roadmap

| Tool | Description |
|---|---|
| `ppdoc-rs` | Rust crate — parse, validate, query `.ppdoc` files |
| `ppdoc-render` | Render `.ppdoc` → PDF / HTML / EPUB using layout rules |
| `ppdoc-diff` | Structural diff between two `.ppdoc` files |
| `ppdoc-sign` | Sign/verify per-section Merkle signatures |
| `ppdoc-embed` | Generate AI embeddings for all sections → `embeddings.json` |
| `pdf2ppdoc` | Import existing PDFs into `.ppdoc` (best-effort semantic extraction) |
| `md2ppdoc` | Convert Markdown → `.ppdoc` (lossless for flat docs) |

---

## 13. Competitive Positioning

`.ppdoc` is the **compile source**, PDF is the **deploy artifact**.

```
Author in .ppdoc
      │
      ├──► PDF        (for legal, print, archival)
      ├──► EPUB       (for reading apps, e-readers)
      ├──► HTML       (for web publishing)
      ├──► Markdown   (for developer docs)
      └──► DOCX       (for MS Office users)
```

This mirrors how:
- **Figma** beat Sketch — `.fig` is source, export to anything
- **LaTeX** dominates academia — `.tex` is source, export to PDF
- **Pandoc** is indispensable — Markdown is source, 40+ output formats

PaperPilot would be the first tool to offer this paradigm with **native PDF import**,
**offline AI embeddings**, and **Rust-speed processing**.

---

## 14. Open Questions

- [ ] Should `document.json` use MessagePack instead of JSON for large documents? (faster parse, smaller size)
- [ ] Should the ZIP container be replaced with a custom framing format for streaming? (like Matroska for video)
- [ ] WASM sandbox: use WASI Preview 2 (WASI 0.2) for the widget runtime?
- [ ] Should embeddings be stored per-paragraph or per-section? (tradeoff: precision vs. file size)
- [ ] Interop with W3C Web Annotations spec for the comment/annotation layer?

---

*Last updated: 2026-10-04 | Spec Version: 0.1-draft*
