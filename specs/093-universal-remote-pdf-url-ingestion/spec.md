# Spec 093: Universal Remote PDF URL Ingestion Across Penta-Interface

## 1. Background & Executive Summary
Currently, PaperPilot processes PDF documents supplied either as local filesystem paths (`paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`) or as in-memory file buffers (`File` / `Uint8Array` in `apps/desktop`, `apps/embed`, and `apps/edge`).

However, in many modern consumer and B2B workflows, documents originate from remote URLs:
- Public web links: `https://www.princexml.com/samples/newsletter/drylab.pdf`
- Cloud storage links: Amazon S3 presigned URLs, Google Cloud Storage, Azure Blob.
- Enterprise API webhooks: Stripe invoice PDFs, DocuSign links, research papers on arXiv.

Requiring users or API clients to manually download the file first and upload raw bytes adds unnecessary latency, bandwidth waste, and friction.

**Spec 093** implements **Universal Remote PDF URL Ingestion** across all 5 PaperPilot surfaces.

---

## 2. Technical Architecture & Requirements

### 2.1 Unified URL Fetcher with Security Controls
A unified URL ingestion utility must:
1. **Support HTTP/HTTPS Streams**: Fetch remote PDF streams directly into memory (`Vec<u8>`).
2. **Magic Byte & Content-Type Validation**:
   - Check `Content-Type: application/pdf` or inspect the first 4 bytes of the stream for `%PDF`.
   - Reject non-PDF payloads gracefully with `PdfError::InvalidFormat("URL does not return a valid PDF document")`.
3. **SSRF (Server-Side Request Forgery) Protection**:
   - In Cloudflare Edge, REST Gateway, and MCP environments, block private and loopback IP ranges:
     - `127.0.0.0/8`
     - `10.0.0.0/8`
     - `172.16.0.0/12`
     - `192.168.0.0/16`
     - `169.254.169.254` (cloud metadata endpoints)
4. **Max File Size Limit**: Enforce configurable maximum download buffer size (default: 100 MB) to prevent denial-of-service memory exhaustion.

---

## 3. Surface-by-Surface Implementation

### 3.1 Desktop & Embed Web UI (`apps/desktop` & `apps/embed`)
- In `DropZone.svelte`:
  - Add an inline secondary input / modal button: **"🔗 Import from URL"**.
  - User pastes a URL (e.g. `https://example.com/sample.pdf`) and presses Enter.
  - Svelte fetches the URL as a `Blob`, extracts the filename from the URL path or `Content-Disposition`, converts it into a `File` object, and dispatches it into `ondrop([file])`.
  - Shows an inline loading spinner: *"Fetching remote PDF..."*.

### 3.2 Cloudflare Edge Microservice (`apps/edge`)
- In `src/index.ts`:
  - In addition to accepting raw binary bodies and `multipart/form-data`, every endpoint checks for `?url=...` query parameter or JSON body `{"url": "https://..."}`:
    ```ts
    let inputBytes: Uint8Array;
    const remoteUrl = url.searchParams.get('url');
    if (remoteUrl) {
      assertSafeUrl(remoteUrl);
      const res = await fetch(remoteUrl);
      if (!res.ok) return new Response('Failed to fetch remote PDF', { status: 400 });
      inputBytes = new Uint8Array(await res.arrayBuffer());
    } else {
      inputBytes = new Uint8Array(await request.arrayBuffer());
    }
    ```
  - Enables zero-upload cURL automation:
    ```bash
    curl "https://paperpilot-edge.kpakkiragari.workers.dev/api/v1/rotate?angle=90&url=https://www.princexml.com/samples/newsletter/drylab.pdf" \
      --output rotated.pdf
    ```

### 3.3 Native CLI (`paperpilot-cli`)
- In `paperpilot-cli`:
  - When `--input` starts with `http://` or `https://`:
    - Automatically use `reqwest` to download the remote stream into a temporary in-memory buffer before passing to the operation runner.

### 3.4 MCP Server (`paperpilot-mcp`)
- In `paperpilot-mcp/src/server.rs`:
  - When an AI agent passes a URL in the `input` field (`"input": "https://example.com/invoice.pdf"`), the MCP tool downloads the remote document transparently.

### 3.5 REST Gateway (`paperpilot-gateway`)
- Endpoints accept JSON payload `{"url": "https://...", ...}` or query param `?url=...`.

---

## 4. Verification & Testing
1. **Unit Tests**:
   - `test_url_fetch_valid_pdf`: Asserts successful download and parsing of a remote PDF fixture.
   - `test_url_fetch_invalid_magic_bytes`: Asserts error when URL returns HTML or image instead of PDF.
   - `test_ssrf_blocking`: Asserts rejection of `http://127.0.0.1:8080` and `http://169.254.169.254`.
2. **Cloudflare Edge Live Verification**:
   - Run cURL against live Edge worker with `?url=https://www.princexml.com/samples/newsletter/drylab.pdf`.
3. **Playwright E2E UI Test**:
   - Emulate entering a URL in `DropZone.svelte` and verify the document loads into the workspace viewer.
