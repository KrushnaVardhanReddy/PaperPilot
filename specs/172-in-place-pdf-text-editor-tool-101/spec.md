# Spec 172: In-Place PDF Text Search and Replacement Engine (`pdf_edit_text` / Tool 101)

## Metadata
- **Spec Number**: 172 (Tool 101)
- **Title**: In-Place PDF Text Search and Replacement Engine (`pdf_edit_text`)
- **Phase**: Phase 5.2 / Document Mutation & Typography Repair
- **Status**: ✅ Completed
- **Impacted Systems**:
  - `paperpilot-pdf`: Core content stream token parser and AST re-serializer (`src/operations/pdf_edit_text.rs`)
  - `paperpilot-cli`: Subcommand `edit-text` / `pdf-edit-text` (`src/commands/pdf_edit_text.rs`)
  - `paperpilot-gateway`: REST endpoints `/api/v1/pdf/edit-text` & `/api/v1/pdf/pdf_edit_text`
  - `paperpilot-mcp`: Tool definition and execution handler `pdf_edit_text` (`paperpilot-mcp/src/server.rs`)
  - `paperpilot-wasm`: WebAssembly export `pdf_edit_text` (`paperpilot-wasm/src/operations.rs`)
  - `apps/desktop`: Svelte 5 Operations view and operationsArgsBuilder
  - `apps/web`: Web studio operations runner
  - `apps/embed`: Embeddable client widget
  - `apps/edge`: Cloudflare Edge Worker microservice
  - `apps/docs`: Astro Starlight reference documentation (`tools-handbook.md`, `reference/tools/pdf_edit_text.md`)

---

## 1. Executive Summary & Problem Statement

Users frequently encounter minor typographical errors, outdated dates, incorrect company names, or erroneous numerical figures in compiled PDF documents where the original editable source (e.g. DOCX, Keynote, LaTeX) is unavailable.

Existing workarounds are problematic:
1. Converting PDF to Word and re-exporting degrades vector art, breaks tabular alignments, and introduces font substitutions.
2. Drawing a white rectangle over existing text and stamping new text on top violates PDF hygiene, fails keyword searches, and exposes redacted text underneath.

**Spec 172** establishes **Tool 101 (`pdf_edit_text`)**: a pure-Rust, in-place text replacement engine that directly rewrites text operators inside uncompressed and Flate-compressed PDF content streams (`/Contents`).

### Key Capabilities
- **Direct Stream Inspection**: Unpacks PDF `/Contents` streams (both direct objects and array streams) and reconstructs font encodings (`WinAnsiEncoding`, `StandardEncoding`, `MacRomanEncoding`, and `Identity-H` two-byte CIDs).
- **In-Place Operator Modification**: Replaces text target bytes within `Tj`, `TJ`, and `'` string operands without disturbing graphics state (`q`/`Q`), matrix transforms (`cm`), or clipping paths.
- **Granular Targeting**: Allows replacing all occurrences or specific 1-indexed matches, scoped to an individual page or document-wide.
- **Penta-Interface Parity**: 100% available via Core Rust, CLI, MCP JSON-RPC, REST API, WASM in browser/mobile, and Cloudflare Workers.

---

## 2. Technical Architecture & Stream Parsing Model

```
[ Input PDF Stream ]
        │
        ├──► 1. Extract Page Tree & Resolve Page `/Contents`
        │      • Direct stream object OR indirect array stream `/Contents [ 4 0 R 5 0 R ]`
        │
        ├──► 2. Font & Encoding Map Extraction
        │      • Inspect `/Resources << /Font << ... >> >>`
        │      • Build glyph-to-unicode and unicode-to-encoded-byte lookup tables
        │
        ├──► 3. Content Stream Tokenization
        │      • Scan operators: `BT`, `ET`, `Tf`, `Tj`, `TJ`, `'`, `"`
        │      • Reconstruct textual runs across kerning offset arrays (`[...] TJ`)
        │
        ├──► 4. Match Evaluation & In-Place Replacement
        │      • Filter by `page` (if specified)
        │      • Evaluate `find` needle against text runs (case-sensitive or insensitive)
        │      • Respect `occurrence` index filter or replace globally
        │      • Encode `replace` text using resolved font encoding or literal UTF-8/WinAnsi
        │
        └──► 5. Stream Compression & PDF AST Serialization
               • Re-compress stream with FlateDecode
               • Update `/Length` entry in stream dictionary
               • Emit clean, standards-compliant PDF bytes
```

### 2.1 Core Rust Data Structures (`paperpilot-pdf`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfEditTextOptions {
    /// Literal text string to search for in page content streams
    pub find: String,
    /// Literal replacement string
    pub replace: String,
    /// Optional 1-indexed page number (if omitted, replaces across all pages)
    #[serde(default)]
    pub page: Option<u32>,
    /// Optional 1-indexed occurrence index to replace (if omitted, replaces all occurrences)
    #[serde(default)]
    pub occurrence: Option<u32>,
    /// Case-sensitive matching flag (defaults to true)
    #[serde(default = "default_true")]
    pub match_case: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfEditTextReport {
    pub success: bool,
    pub total_replacements: u32,
    pub modified_pages: Vec<u32>,
    pub message: String,
}
```

---

## 3. Penta-Interface Specifications

### 3.1 CLI (`paperpilot-cli`)
Subcommand: `paperpilot edit-text` (Alias: `paperpilot pdf-edit-text`)
```bash
paperpilot edit-text \
  --input document.pdf \
  --output corrected.pdf \
  --find "Acme Corp 2025" \
  --replace "Acme Corp 2026" \
  --page 1 \
  --occurrence 1 \
  --json
```

### 3.2 MCP JSON-RPC (`paperpilot-mcp`)
Tool Registration: `pdf_edit_text`
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "pdf_edit_text",
    "arguments": {
      "input": "document.pdf",
      "output": "corrected.pdf",
      "find": "Draft Copy",
      "replace": "Final Release",
      "match_case": false
    }
  },
  "id": 1
}
```

### 3.3 REST API Gateway (`paperpilot-gateway`)
Endpoints:
- `POST /api/v1/pdf/edit-text`
- `POST /api/v1/pdf/pdf_edit_text`

Request Payload:
```json
{
  "input": "document.pdf",
  "output": "corrected.pdf",
  "find": "Old String",
  "replace": "New String",
  "page": 1,
  "occurrence": 1,
  "match_case": true
}
```

### 3.4 WebAssembly & Client Studio (`paperpilot-wasm`, `apps/web`, `apps/embed`)
Export signature:
```typescript
export function pdf_edit_text(
  pdf_bytes: Uint8Array,
  find: string,
  replace: string,
  page?: number,
  occurrence?: number,
  match_case?: boolean
): Uint8Array;
```

---

## 4. Verification & Validation Protocol
1. **Core Unit Tests**: `cargo test -p paperpilot-pdf --lib operations::pdf_edit_text -j 4`.
2. **Penta-Interface E2E Suite**: `tools/penta-interface-e2e/src/cases/pdf_edit_text.rs` validating across CLI, MCP, REST, and WASM.
3. **WebdriverIO Desktop E2E**: `apps/desktop/test/specs/pdf_edit_text.e2e.ts`.
4. **Cloudflare Edge Microservice**: `apps/edge/tests/edge.spec.ts`.
