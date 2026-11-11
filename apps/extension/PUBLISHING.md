# PaperPilot Browser Companion Extension — Store Publishing Guide

> **Target Platforms**: Google Chrome Web Store, Microsoft Edge Add-ons, Mozilla Firefox AMO  
> **Package Architecture**: Manifest V3, WASM-First client processing, optional PaperPilot Desktop Gateway bridge

---

## 1. Automated Packaging

To generate a clean production package ready for store upload, run from the repository root:

```bash
pnpm run pack:extension
```

This compiles all TypeScript, Svelte 5 popup components, and WebAssembly bundles, and creates:
📁 **`apps/extension/paperpilot-extension.zip`**

### Package Inspection
The root of the `.zip` archive contains:
- `manifest.json` (Manifest V3)
- `background.js` (Service Worker)
- `content.js` (Clean web article reader & context script)
- `popup.html` (Toolbar popup entry)
- `paperpilot_wasm_bg.wasm` (Compiled Rust WebAssembly engine)
- `icons/` (`icon-16.png`, `icon-48.png`, `icon-128.png`)
- `assets/` (Bundled chunks & stylesheets)

---

## 2. Google Chrome Web Store Submission

### 2.1 Developer Registration
1. Navigate to the [Chrome Developer Dashboard](https://chrome.google.com/webstore/devconsole).
2. Sign in with your Google account.
3. Pay the one-time registration fee ($5 USD) if publishing for the first time.

### 2.2 Item Upload
1. Click the **"+ New Item"** button.
2. Drag and drop **`apps/extension/paperpilot-extension.zip`**.
3. Once validated, you will be taken to the store listing editor.

### 2.3 Store Listing Details
- **Title**: `PaperPilot: Universal Document & AI Companion`
- **Short Summary (max 132 chars)**:  
  `Document intelligence, PDF manipulation, and clean web article capture powered by WebAssembly.`
- **Detailed Description**:
  ```markdown
  PaperPilot Companion brings local document intelligence and PDF processing directly into your browser tabs.

  Key Capabilities:
  • 100% Private In-Browser Processing: Split, merge, rotate, watermark, and extract text using client-side WebAssembly without sending files to external servers.
  • Clean Article Reader: Turn ad-heavy web pages and articles into sanitized, ad-free Markdown or PDF with one click.
  • AI Semantic Chunking: Structure-aware chunking for AI LLM prompts and RAG embeddings.
  • PaperPilot Desktop Bridge: Seamlessly connects to your local PaperPilot desktop daemon on port 7823 for heavy neural OCR and large-file memory mapping.
  ```
- **Category**: `Productivity` or `Developer Tools`.
- **Language**: `English`.
- **Icons**: Pre-configured inside `icons/` in the package.
- **Screenshots**: Upload 1280x800 or 640x400 PNG/JPEG screenshots showcasing:
  1. Quick Actions & Web Article to Markdown extraction.
  2. PDF Tools (Split, Rotate, Watermark).
  3. AI Semantic Chunking view.

### 2.4 Privacy Practices Tab (Crucial for Quick Approval)
- **Single Purpose**:  
  `Local document manipulation (PDF split, merge, rotate, watermark) and clean text/markdown extraction from web pages.`
- **Permission Justifications**:
  | Permission | Justification |
  | :--- | :--- |
  | `activeTab` | Allows the user to extract clean markdown text from the current active tab upon clicking the toolbar action. |
  | `contextMenus` | Provides right-click shortcuts on web PDFs and links ("PaperPilot: Extract Text", "PaperPilot: Semantic Chunk"). |
  | `storage` | Caches recent user-extracted snippets locally on device without remote transmission. |
  | `scripting` | Injects the clean-article DOM sanitizer to strip advertisements, navbars, and tracking widgets. |
- **Host Permissions (`http://127.0.0.1:7823/*`, `http://localhost:7823/*`)**:  
  `Enables the optional local bridge to the user's running PaperPilot desktop daemon on localhost for offline neural OCR processing.`
- **Data Usage Declaration**:
  - Check: **"I do not sell or transfer user data to third parties."**
  - Check: **"I do not use or transfer user data for purposes that are unrelated to the item's core functionality."**
  - Check: **"I do not use or transfer user data to determine creditworthiness or for lending purposes."**

### 2.5 Submit for Review
Click **"Submit for Review"**. Manifest V3 extensions with pure local WASM typically receive approval within **24–48 hours**.

---

## 3. Microsoft Edge Add-ons Submission

Microsoft Edge shares the same Manifest V3 format with zero registration fee:

1. Navigate to the [Microsoft Edge Partner Center](https://partner.microsoft.com/dashboard/microsoftedge).
2. Sign in with a Microsoft account.
3. Click **"Create new extension"**.
4. Upload `apps/extension/paperpilot-extension.zip`.
5. Mirror the descriptions, category, and screenshots used for Chrome.
6. Submit for review (typically approved in 1–2 business days).

---

## 4. Mozilla Firefox (Add-ons for Firefox / AMO)

Firefox Manifest V3 compatibility:
1. Navigate to the [Firefox Add-on Developer Hub (AMO)](https://addons.mozilla.org/developers/).
2. Click **"Submit a New Add-on"**.
3. Choose **"On this site"** (for public distribution) or **"On your own"** (self-distributed unsigned/signed XPI).
4. Upload `apps/extension/paperpilot-extension.zip`.
5. Enter description, select categories (`Productivity`, `Document Management`), and submit.

---

## 5. Direct Enterprise / Local Developer Distribution

To distribute the extension internally or test without store review:

1. Share `apps/extension/paperpilot-extension.zip` or the unpacked `apps/extension/dist` directory.
2. Instruct users to:
   - Navigate to `chrome://extensions` or `edge://extensions`.
   - Enable **Developer mode** (top-right toggle).
   - Click **"Load unpacked"** and select the `dist` directory.
   - Click the extension icon in the toolbar, or click the **↗ (Full Tab)** button to run in permanent tab mode.
