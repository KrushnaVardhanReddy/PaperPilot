# Web Portal Navigation & Mega-Menu Architecture

## Overview

The PaperPilot Web Portal (`apps/web` / `usepaperpilot.com`) delivers client-side WebAssembly execution of all PDF operations. As PaperPilot reached **101 pure-Rust tools** (culminating with Tool 101: `pdf_edit_text`), the portal navigation was overhauled to provide a clean, modern SaaS navigation experience inspired by Stirling PDF while highlighting PaperPilot's key architectural advantage: **100% Client-Side WebAssembly execution with Zero Cloud Uploads**.

---

## 1. Top Navigation & Header (`Navbar.svelte`)

The navigation bar is implemented as a sticky glassmorphic header (`Navbar.svelte`) containing:

1. **Brand Identity & Trust Pill**:
   - PaperPilot logo and "Web" badge.
   - Live "100% Client-Side WASM — Zero Uploads" pulsing status pill.

2. **Tools Mega-Menu Dropdown (`MegaMenu.svelte`)**:
   - Triggers from the "Tools ▾" navigation item.
   - Organizes all 101 tools into 5 primary categories:
     - 📑 **Organize & Pages**: Merge, Split, Reorder, Rotate, Delete, Booklet, N-Up, Scale, Auto-Upright, Scanned Book Split, Alternate Merge, Posterize, Burst, Compare, Remove Blank.
     - ✏️ **Edit & Protect**: Edit PDF Text (Tool 101 `pdf_edit_text`), Watermark, Add Stamp, Letterhead Overlay/Underlay, Sign PDF, Encrypt, Decrypt, Sanitize, Redact, Metadata, Header & Footer, Page Numbers, Flatten, Attachments, Permissions, Bates Numbering.
     - 🔄 **Convert**: PDF to Word (DOCX), Excel (XLSX), PPTX, Images, Markdown, HTML, JSON, EPUB, Animated GIF, CBZ Comic Archive, Showcase Demo, Office formats.
     - ⚡ **Optimize & Security**: High-Ratio Compression, Stream Deduplication, Repair & Self-Healing, Linearize, Cryptographic Hash, Validate Signatures, Strip Signatures, Timestamp, Deskew, Invert Colors.
     - 🤖 **Intelligence & Automation**: Pure-Rust OCR, Sensitive PII Scan, Table Extraction, Pattern/Regex Extraction, LLM Semantic Chunking, RAG Embeddings, Semantic Search, Batch Fill, CLI Ask, Workflows.
   - Category filtering tabs and live search query input.

3. **Features Dropdown (`FeaturesDropdown.svelte`)**:
   - Client-Side WASM Engine (~30-95ms execution).
   - 100% Privacy & Zero Uploads.
   - Hexagonal Storage Architecture.
   - CLI & AI Agent MCP Server.

4. **Global Tool Quick-Finder (`ToolQuickFinderModal.svelte`)**:
   - Keyboard accessible via `Cmd+K` or `Ctrl+K`.
   - Fuzzy search over all 101 tools, titles, tags, and descriptions.
   - Arrow keys (`↑`/`↓`), `Enter` to select, `ESC` to close.

5. **Desktop App Modal (`DesktopDownloadModal.svelte`)**:
   - One-click modal offering native desktop installers for Windows (.msi/.exe), macOS (.dmg universal), and Linux (.AppImage/.deb).

6. **Documentation, GitHub & Feedback**:
   - Direct link to Astro Starlight documentation portal.
   - GitHub link with star badge.
   - Feedback & Bug reporting trigger connected to `FeedbackModal.svelte`.
   - High-contrast gradient "Download Desktop" primary CTA button.

---

## 2. Hero Section & Quick Launch Row (`HeroPlayground.svelte`)

To provide immediate utility without overwhelming the user, the hero area contains:

1. **Privacy Trust Banner (`PrivacyTrustBanner.svelte`)**:
   - Emphasizes the zero-upload guarantee directly under the top header.
2. **Quick Tool Launch Row (`QuickToolLaunchRow.svelte`)**:
   - Direct pill access to the 6 most frequently used operations:
     - `[ 📑 Merge ]`
     - `[ ✂️ Split ]`
     - `[ 🗜️ Compress ]`
     - `[ ✏️ Edit Text ]`
     - `[ 🔍 OCR ]`
     - `[ 🔄 PDF to Word ]`
3. **Seamless WASM Activation**:
   - Clicking any tool anywhere in the portal dispatches a `select-tool` event with the target `wasmToolId` and smoothly scrolls to the `OperationsView.svelte` playground.

---

## 3. Tool Catalog (`toolsCatalog.ts`)

All 101 tools are cataloged with strict typing and metadata:

```typescript
export interface CatalogTool {
  id: string;
  wasmToolId: string;
  name: string;
  icon: string;
  category: 'organize' | 'edit' | 'convert' | 'optimize' | 'intelligence';
  categoryLabel: string;
  description: string;
  isPopular?: boolean;
  handbookNum: number;
}
```

The catalog is synchronized with `apps/docs/src/content/docs/reference/tools-handbook.md`.

---

## 4. Verification & Testing

- `npm run build` inside `apps/web`: Produces optimized Vite production bundle with zero syntax errors.
- Event integration: Tested tool activation across both `select-tool` custom event and `activeTool` reactive state bindings.
