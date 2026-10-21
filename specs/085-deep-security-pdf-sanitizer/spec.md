# Spec 085: Privacy & Deep Security Sanitizer (`pdf_sanitize`)

## 1. Executive Summary
PDF files created by modern software (Microsoft Word, Google Docs, Adobe Acrobat, macOS Preview) frequently retain hidden forensic data, unreferenced object streams, revision histories, sensitive metadata, embedded file attachments, and active executable scripts.

While basic redaction masks visible pixels, it does **not** protect against forensic data recovery or malicious embedded payloads. **Spec 085** implements `pdf_sanitize`, an uncompromised, pure-Rust deep security scrubbing engine following the **Single Core Runner + 5 Thin Wrappers** architecture.

---

## 2. Sanitization Capabilities (What gets scrubbed)

A sanitized document has the following elements stripped or neutralized without degrading visible text or vector rendering:

1. **Document Info & Metadata**:
   - Strips Document Information Dictionary (`/Title`, `/Author`, `/Subject`, `/Keywords`, `/Creator`, `/Producer`, `/CreationDate`, `/ModDate`).
   - Strips all `/Metadata` XMP streams.
2. **Revision History & Incremental Updates**:
   - Rebuilds cross-reference tables and rewrites the entire PDF as a clean, single-generation serialized file, purging deleted objects, historical xref tables, and slack space.
3. **Embedded Files & Attachments**:
   - Removes `/EmbeddedFiles` in document `/Names` dictionary and `/AF` (Associated Files) arrays.
4. **Active Scripts & Executable Actions**:
   - Neutralizes `/JavaScript`, `/JS`, `/OpenAction`, `/AA` (Additional Actions), and `/Launch` actions to prevent exploit payloads or telemetry phoning home.
5. **Private Application Data & Thumbnails**:
   - Removes `/PieceInfo` (Adobe Illustrator / Photoshop private editing layers).
   - Removes `/Thumb` (page thumbnail preview caches that might expose unredacted prior content).

---

## 3. Architecture Pattern: Single Core Runner + 5 Thin Wrappers

```
                     ┌────────────────────────────────────────────────────────┐
                     │   Single Core Runner (paperpilot-pdf)                   │
                     │   fn sanitize_pdf(bytes, &options) -> Result<Vec<u8>>   │
                     └──────────────────────────┬─────────────────────────────┘
                                                │
         ┌──────────────────┬───────────────────┼───────────────────┬──────────────────┐
         │                  │                   │                   │                  │
         ▼                  ▼                   ▼                   ▼                  ▼
    1. Native CLI      2. Agent MCP       3. REST Gateway       4. WASM Engine     5. Edge Worker
  paperpilot sanitize   pdf_sanitize     POST /api/v1/pdf/     WasmPdfEngine.     POST /api/v1/
                         tool call           sanitize             sanitize          sanitize
```

### 3.1 Single Core Runner (`paperpilot-pdf/src/operations/cleanup.rs` or `sanitize.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeOptions {
    pub remove_metadata: bool,
    pub remove_attachments: bool,
    pub remove_scripts: bool,
    pub remove_thumbnails: bool,
    pub remove_private_data: bool,
}

pub fn sanitize_pdf(
    input_bytes: &[u8],
    options: &SanitizeOptions,
) -> Result<(Vec<u8>, SanitizeReport), anyhow::Error>;
```

### 3.2 The 5 Thin Wrappers
1. **CLI (`paperpilot-cli/src/commands/group_b.rs` or `sanitize.rs`)**:
   `paperpilot sanitize --input in.pdf --output out.pdf [--strip-metadata] [--strip-scripts]`
2. **MCP (`paperpilot-mcp/src/server.rs`)**:
   Tool `pdf_sanitize`: `{ "input": "...", "output": "...", "remove_metadata": true }`
3. **REST Gateway (`paperpilot-gateway/src/handlers.rs` & `server.rs`)**:
   `POST /api/v1/pdf/sanitize` (supporting both multipart file upload and JSON body)
4. **Browser WASM (`paperpilot-wasm/src/operations.rs` & `lib.rs`)**:
   `WasmPdfEngine.sanitize(inputBytes, optionsJson) -> Uint8Array`
5. **Cloudflare Edge (`apps/edge/src/index.ts`)**:
   `POST /api/v1/sanitize` -> calls `WasmPdfEngine.sanitize`

---

## 4. Acceptance Criteria & Merge Gate
- [ ] 100% unit test coverage in `paperpilot-pdf` confirming XMP metadata, `/Names /EmbeddedFiles`, and `/JS` are removed.
- [ ] Unmocked Penta-Interface E2E suite passes across all 5 surfaces.
- [ ] Integration in Desktop Operations Panel (`OperationsPanel.svelte`) and Visual Pipeline (`pipeline.svelte.ts`).
