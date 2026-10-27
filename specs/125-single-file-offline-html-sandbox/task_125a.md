# Task 125A: Single-File Offline Self-Contained HTML Sandbox (`paperpilot-portable.html`)

## Metadata
- **Task ID**: `Task 125A`
- **Spec**: `[Spec 125]` (`specs/125-single-file-offline-html-sandbox/spec.md`)
- **Phase**: `Phase 5.9.94`
- **Target Branch**: `feature/dev`
- **Origin**: Requested by `spwa4` on Hacker News

---

## 1. Goal
Produce an air-gapped, zero-dependency, self-contained single-file `paperpilot-portable.html` distribution containing the Web UI and inline base64 WebAssembly runtime that executes client-side PDF operations without a web server or internet connection.

---

## 2. Implementation Steps

### Step 1: Ensure Client-Side WASM Build
Compile `paperpilot-wasm` with `wasm-pack`:
```bash
wasm-pack build --target web paperpilot-wasm
```

### Step 2: Implement Portable Inlining Generator (`scripts/build_portable_html.js`)
- Read compiled `paperpilot-wasm/pkg/paperpilot_wasm_bg.wasm` and convert to base64 string.
- Read JS glue `paperpilot-wasm/pkg/paperpilot_wasm.js` and modify `init()` to instantiate from `Uint8Array.from(atob(inlineWasmBase64))` rather than `fetch()`.
- Package a clean, responsive single-file UI embedding:
  - Drag & drop PDF file inputs.
  - Core tool selector (Merge, Split, Rotate, Delete, Compress, Extract Text).
  - Client-side download trigger via `URL.createObjectURL(blob)`.
  - Strict air-gap Content Security Policy (`<meta http-equiv="Content-Security-Policy" ...>`).
- Output to `dist-portable/paperpilot-portable.html`.

### Step 3: Add Automation & Makefile Target
Add target `make build-portable` in root `Makefile`.

### Step 4: Add Automated Offline Verification Test
Create a Playwright test `tests/portable_html.spec.ts` that:
- Loads `file://${PWD}/dist-portable/paperpilot-portable.html`.
- Monitors network events and asserts `requestsCount === 0`.
- Executes a PDF operation and verifies output download blob.

---

## 3. Exit Criteria
- `dist-portable/paperpilot-portable.html` generated and under 5 MB total size.
- 0 network requests emitted.
- All unit and Playwright assertions passing.
- `PHASES.md` and `HANDOFF.md` updated.
