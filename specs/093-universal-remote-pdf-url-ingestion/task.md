# Task 5.9.78: Universal Remote PDF URL Ingestion

## Status
- Spec: [Spec 093](file:///home/krushna/Project/PaperPilot/specs/093-universal-remote-pdf-url-ingestion/spec.md)
- Target: Wave 3 (`v0.7.0`) Advanced Ingestion & Integration
- Status: Ready for Implementation

---

## Objectives & Work Items

### 1. Cloudflare Edge URL Parameter Ingestion (`apps/edge`)
- In `apps/edge/src/index.ts`, add helper `resolveInputBytes(request: Request, url: URL): Promise<Uint8Array>`.
- If `url.searchParams.has('url')` or body JSON has `url`, fetch remote URL via `fetch()`.
- Validate `%PDF` magic bytes and restrict internal IP ranges (SSRF protection).
- Support all existing endpoints (`/rotate`, `/compress`, `/watermark`, `/split`, etc.) with `?url=...`.

### 2. Desktop & Web UI URL Input (`apps/desktop` & `apps/embed`)
- In `DropZone.svelte`:
  - Add "🔗 Import URL" action button next to file picker.
  - Add modal or inline input for pasting a URL.
  - Fetch URL as `Blob`, derive filename, and emit `ondrop([file])`.
  - Add loading state and human-friendly error toast on invalid URLs.

### 3. CLI & MCP Remote URL Resolution
- In `paperpilot-cli/src/commands/`:
  - Intercept `--input` values beginning with `http://` or `https://` and download via `reqwest`.
- In `paperpilot-mcp/src/server.rs`:
  - Resolve remote `input` URLs automatically before executing operations.

### 4. Verification & Testing
- Unit tests in `apps/edge/tests/edge.spec.ts` testing URL fetching.
- Playwright test verifying URL paste into `DropZone.svelte`.
- Live test against `https://paperpilot-edge.kpakkiragari.workers.dev`.
