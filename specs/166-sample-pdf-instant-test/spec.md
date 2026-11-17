# Spec 166: "Try with Sample PDF" Instant Testing in Web Studio

## 1. Executive Summary
Provide a zero-friction, one-click evaluation experience for visitors on `usepaperpilot.com`. Users without a local PDF file immediately at hand can click **"✨ Try with Sample PDF"** to load a representative multi-page test PDF directly into the in-browser WASM playground, allowing instant verification of merge, split, rotate, watermark, compress, and metadata extraction.

---

## 2. Implementation Details

### 2.1 Sample Asset Hosting
Location: `apps/web/public/samples/`
- `sample_document.pdf`: Lightweight (1.4KB) valid multi-page PDF document.
- `sample_form.pdf`: AcroForm field test document.

### 2.2 DropZone Integration
Location: `apps/web/src/components/DropZone.svelte`
- Displays an accessible `actions-row` containing:
  - `Browse Files` file picker button.
  - `or` divider.
  - `✨ Try with Sample PDF` secondary action button.
- On click:
  - Fetches `/samples/sample_document.pdf` asynchronously via standard `fetch()`.
  - Converts response Blob into a browser `File` object (`sample_document.pdf`).
  - Calls `ondrop([sampleFile])`, propagating directly into active tool pipelines (WASM worker).

---

## 3. Verification & Testing Standards
1. Vite build succeeds with `pnpm --dir apps/web build`.
2. Static assets bundled properly into `apps/web/dist/samples/`.
3. Zero network requests outside origin; sample loads cleanly from same-origin `/samples/`.
