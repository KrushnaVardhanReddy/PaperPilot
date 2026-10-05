# WebAssembly (WASM) E2E Testing Architecture

PaperPilot's WASM engine guarantees 100% client-side execution using `wasm32-unknown-unknown` inside modern web browsers. To ensure absolute reliability and guard against regressions when integrating Rust into the browser environment, we have implemented an automated E2E testing strategy using **Playwright**.

## Architecture Overview

The testing framework bridges Node.js and the Browser, measuring pure execution of the WASM payloads via a dedicated Web Worker environment, isolated from the UI rendering layer.

### 1. Playwright Web Environment
Tests are driven by Playwright (`apps/web/tests/e2e_wasm_12_tools.spec.ts`) running a headless Chromium browser instance.
The test configuration boots the SvelteKit/Vite development server (`npm run dev`) at `http://localhost:5173`.

### 2. Execution Harness
Rather than testing DOM clicks, the suite tests the **internal WASM APIs directly inside the browser context**.
- **Fixtures:** PDF files (`single_page.pdf` and `multi_page.pdf`) are loaded from disk into memory inside the Node.js test environment.
- **Serialization:** They are converted to `Base64` strings and injected into the headless Chromium browser via `page.evaluate()`.
- **WASM Bridge:** Inside the browser context, the test dynamically imports the WASM client (`/src/lib/wasm/index.ts`).
- **Execution:** The base64 payloads are transformed back into `Uint8Array` arrays, and the pure Rust-WASM methods (e.g. `merge`, `compress`, `encrypt`) are invoked.

### 3. Validation & Reporting
- The resulting byte arrays returned by the WASM module are evaluated.
- The system explicitly verifies the presence of the PDF magic number `%PDF-` (`0x25 0x50 0x44 0x46`) in the output.
- Performance latency (`performance.now()`) is recorded for every tool execution.
- Metrics are compiled into a markdown table written natively to disk as `reports/WASM_12_TOOLS_E2E_SCORECARD.md`.

## 12 Supported Tools
The E2E suite exhaustively validates all 12 core pure-Rust WASM operations:
1. `merge`
2. `split`
3. `rotate`
4. `compress`
5. `encrypt`
6. `watermark`
7. `delete_pages`
8. `extract_pages`
9. `reorder_pages`
10. `crop`
11. `flatten`
12. `set_metadata`

## Execution Commands

### Local Environment
To run the test suite and regenerate the scorecard:

```bash
cd apps/web
pnpm install
npx playwright test tests/e2e_wasm_12_tools.spec.ts
```

The scorecard will be instantly available in the root `reports/` directory.