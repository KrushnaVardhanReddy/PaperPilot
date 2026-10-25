# Task 108A: Implement PDF Attachments & Portfolio Engine (`pdf_attachments`)

## Metadata
- **Spec**: [Spec 108: Manage PDF Attachments & Portfolios (`pdf_attachments`)](file:///home/krushna/Project/PaperPilot/specs/108-pdf-attachments/spec.md)
- **Phase**: Phase 5, Task 5.3.12
- **Primary Crate**: `paperpilot-pdf`
- **Impacted Interfaces**:
  - `paperpilot-pdf` (core engine & unit tests)
  - `paperpilot-cli` (CLI commands: list, extract, add, remove)
  - `paperpilot-mcp` (MCP tool schema & tool dispatch)
  - `paperpilot-gateway` (REST endpoints & OpenAPI)
  - `paperpilot-wasm` (In-memory WebAssembly engine)
  - `apps/desktop` (OperationsPanel UI controls & AuditReportViewer integration)
  - `tools/penta-interface-e2e` (E2E test suite)

---

## Detailed Requirements

### 1. Core Engine: `paperpilot-pdf/src/operations/pdf_attachments.rs`
1. **Attachment Extraction & Listing**:
   - Inspect Document Catalog `/Root` for `/Names` -> `/EmbeddedFiles`.
   - Parse `/Names` array or `/Kids` intermediate nodes (ISO 32000-1 Name Tree).
   - Resolve `/Filespec` objects: extract `/F`, `/UF`, `/Desc`, and `/EF` dictionary.
   - For listing: extract metadata (`AttachmentInfo`: name, size, mime_type, dates) and serialize to JSON.
   - For extraction: decode embedded stream (`/EmbeddedFile`) using FlateDecode.
2. **Attachment Injection**:
   - Given a file name, byte buffer, and optional description/MIME:
   - Create an `/EmbeddedFile` stream object compressed with FlateDecode.
   - Create a `/Filespec` dictionary referencing the stream under `/EF << /F ... >>`.
   - Insert or update into the `/EmbeddedFiles` Name Tree under `/Root /Names`.
   - Save modified PDF.
3. **Attachment Removal**:
   - Search for the specified file name in `/EmbeddedFiles /Names` array and remove key/value pair.
4. **Unit Tests**:
   - `test_pdf_attachments_list_empty`: Returns empty list on standard PDF.
   - `test_pdf_attachments_add_and_list`: Adds attachment, lists it, verifies name and byte size match.
   - `test_pdf_attachments_extract`: Injects text/csv file, extracts it, asserts byte content equality.
   - `test_pdf_attachments_remove`: Adds attachment, removes it, asserts count drops to 0.

---

### 2. Penta-Interface Integration
1. **CLI (`paperpilot-cli`)**:
   - Ensure `paperpilot attachments` supports `--action <list|extract|add|remove>`, `--file-name <name>`, `--file <path>`, `--output-dir <dir>`.
2. **MCP (`paperpilot-mcp`)**:
   - Update `schema_pdf_attachments` with typed action, file_name, file_data_base64 parameters.
3. **REST Gateway (`paperpilot-gateway`)**:
   - `GET /api/v1/pdf/attachments` or `POST /api/v1/pdf/attachments` accepting multipart PDF and parameters.
4. **WASM (`paperpilot-wasm`)**:
   - Expose `wasm_pdf_attachments(pdf_bytes, options_json)`.
5. **Desktop UI (`apps/desktop/src/lib/components/layout/OperationsPanel.svelte`)**:
   - Add custom UI controls when `activeTool.id === 'pdf_attachments'`:
     - Action selector buttons (`📋 List Attachments`, `📥 Extract`, `➕ Add Attachment`, `🗑️ Remove`).
     - Display attached file list or file upload picker for adding new attachments.
     - Integration with `AuditReportViewer.svelte` to display JSON attachment report cleanly.

---

### 3. Verification & Acceptance Criteria
1. `cargo test -p paperpilot-pdf operations::pdf_attachments -j 2` passes 100%.
2. `cargo check --workspace -j 2` passes with zero errors.
3. Desktop UI shows interactive attachment controls when selecting the tool.
