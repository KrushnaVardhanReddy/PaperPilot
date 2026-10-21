# Deep Security PDF Sanitizer (pdf_sanitize)

## Architecture
The `pdf_sanitize` tool implements the **"Single Core Runner + 5 Thin Wrappers"** architecture.

### Single Core Runner
The core execution engine lives in `paperpilot-pdf/src/operations/sanitize.rs`. It operates purely in-memory and returns the sanitized byte array. It uses `lopdf` to aggressively strip hidden metadata, active javascript content, embedded file attachments, XML XMP streams, page thumbnails, and private editing data (`/PieceInfo`) to ensure a deeply sanitized, clean single-generation PDF for secure environments.

### 5 Thin Wrappers
1. **CLI Wrapper (`paperpilot-cli`)**: Located in `paperpilot-cli/src/commands/sanitize.rs`. Executes via `paperpilot sanitize --input <in> --output <out>`.
2. **MCP Wrapper (`paperpilot-mcp`)**: Registered in `paperpilot-mcp/src/server.rs` as the `pdf_sanitize` tool, responding to JSON-RPC requests via stdio.
3. **REST Gateway Wrapper (`paperpilot-gateway`)**: Exposed at `POST /api/v1/pdf/sanitize` taking multipart file uploads or JSON pointers.
4. **WASM Engine Wrapper (`paperpilot-wasm`)**: Client-side execution logic defined in `paperpilot-wasm/src/operations.rs` and bound via `#[wasm_bindgen]` in `lib.rs` for `embed.js`.
5. **Cloudflare Edge Worker (`apps/edge`)**: REST endpoint bound to the WASM worker running serverlessly at the CDN edge.

## Security Scope
The sanitizer removes the following by default (configurable via `SanitizeOptions`):
- `/Info` Document Dictionary
- `/Metadata` from Catalog and Pages
- `/EmbeddedFiles` and `/AF` from Catalog `/Names`
- `/JavaScript`, `/JS`, `/OpenAction`, `/AA` (Active Scripts)
- `/Thumb` (Thumbnails)
- `/PieceInfo` (Private editing artifacts)

## Developer Examples

### CLI Execution
```bash
paperpilot sanitize \
    --input confidential.pdf \
    --output sanitized.pdf \
    --remove-metadata true \
    --remove-attachments true \
    --remove-scripts true
```

### WASM Execution (In-Browser Embed)
```javascript
const inputBytes = new Uint8Array(await file.arrayBuffer());
const options = JSON.stringify({
    remove_metadata: true,
    remove_attachments: true,
    remove_scripts: true,
    remove_thumbnails: true,
    remove_private_data: true
});
const sanitizedBytes = await window.PaperPilot.executeTool('pdf_sanitize', inputBytes, options);
```