# Spec 164: PaperPilot Browser Companion Extension (Phase 11)

## 1. Executive Summary
The PaperPilot Browser Companion Extension brings the document intelligence and manipulation power of PaperPilot directly into Google Chrome, Microsoft Edge, Brave, and Mozilla Firefox.
Built on **Manifest V3**, the extension features a **WASM-First, Hybrid Architecture**:
1. **Instant Standalone Mode (Zero Install)**: Uses client-side `paperpilot-wasm` so any user who installs the extension can immediately merge, split, rotate, watermark, extract text, and inspect metadata without needing the desktop app running.
2. **Desktop Power Mode**: Automatically detects the local PaperPilot daemon (`http://127.0.0.1:7823`). When active, it unlocks heavy native operations (neural PP-OCRv4, large-file memory mapping, full Office conversions, and semantic chunking via `pdf_chunk`).
3. **In-Tab Context Menus**: Users right-click any web PDF (e.g. arXiv preprints, Stripe receipts, SEC filings) to extract text, summarize, or chunk for AI context.
4. **Clean Web Article Exporter**: Captures article web pages, strips advertisements and clutter, and exports them to clean PDF/A or structured Markdown.

---

## 2. Technical Architecture & Directory Layout

Location: `apps/extension`

```
apps/extension/
├── manifest.json            # Manifest V3 specification
├── package.json             # Build scripts (Vite + Svelte 5 / TypeScript)
├── vite.config.ts           # Bundles background worker, content script, and popup UI
├── src/
│   ├── background/
│   │   ├── index.ts         # Service worker handling context menus & gateway bridge
│   │   └── gatewayClient.ts # Detects & proxies requests to http://127.0.0.1:7823
│   ├── content/
│   │   ├── index.ts         # Injects clean-article capture & in-tab PDF detectors
│   │   └── articleReader.ts # Clutter-free DOM extraction to markdown / sanitized HTML
│   ├── popup/
│   │   ├── App.svelte       # Fast Svelte 5 popup UI
│   │   ├── main.ts          # Mounts Svelte popup
│   │   └── popup.html       # Extension toolbar action popup
│   └── wasm/
│       └── wasmBridge.ts    # Loads paperpilot-wasm for client-side processing
└── tests/
    └── extension.spec.ts    # Headless Playwright / Puppeteer Chrome extension test
```

---

## 3. Core Features & Capabilities

### 3.1 Dual Execution Engine
- **Client WASM Engine**:
  - Leverages `paperpilot-wasm` bindings.
  - Supports: `pdf_split`, `pdf_merge`, `pdf_rotate`, `pdf_watermark`, `pdf_extract_text`, `pdf_metadata`, and `pdf_hash`.
  - Runs in-memory inside the browser tab or extension worker; documents never leave the browser.
- **Local Gateway Bridge (`http://127.0.0.1:7823`)**:
  - Pings `GET http://127.0.0.1:7823/health` on popup mount.
  - If 200 OK: Lights up green indicator `[⚡ Desktop Connected]`.
  - Routes heavy requests (`pdf_chunk`, `pdf_ocr`, `pdf_to_docx`, `pdf_extract_tables`) to `/api/v1/...`.

### 3.2 Context Menu Actions
When a user right-clicks on a PDF link or an open PDF tab:
- *"PaperPilot: Extract Clean Markdown"*
- *"PaperPilot: Semantic Chunk for AI"*
- *"PaperPilot: OCR & Extract Text"*
- *"PaperPilot: Open in Desktop App"*

### 3.3 Clean Web Article to PDF / Markdown
- Injects a lightweight reader script to strip navbars, ads, trackers, and popups.
- Converts main content to formatted Markdown with preserved headers, or renders directly to clean PDF.

---

## 4. Verification & Testing Standards
1. Extension build succeeds with `pnpm --dir apps/extension build` producing a valid `dist/` directory with `manifest.json`.
2. Playwright extension test verifying extension loading in headless Chromium and popup initialization.
3. Zero regressions across existing workspace crates (`cargo check --workspace --all-targets -j 4`).

