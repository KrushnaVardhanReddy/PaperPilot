# Spec 087: Mobile Touch-First WASM Engine & Embed Demonstration

## Status: Approved / Draft
**Phase / Task**: 5.9.67 (Mobile Architecture Refinement)  
**Parent Epic**: Phase 5 — Native Engine & Platform Expansion  
**Dependencies**: `paperpilot-wasm`, `apps/embed` (`Widget.svelte`), `apps/desktop` (Mobile Touch UI)

---

## 1. Executive Summary & Objective

Mobile users engage with document software for **emergency, high-utility, on-the-go micro-tasks** (e.g. camera-scanning a receipt to PDF, emergency signing a contract, merging two attachments, deleting a bad page, compressing a file before WhatsApp/email upload). Heavy enterprise batch processing and terminal CLI pipelines are neither practical nor desired on phones.

Previously, mobile builds required complex cross-compilation of heavy native desktop crates via Tauri 2 (`paperpilot-gateway`, `paperpilot-mcp`, native file system bridges), resulting in APK bloat, build fragility, and Android scoped storage (`content://`) permission friction.

This specification formalizes **WASM-First Mobile Architecture**:
1. Powering Mobile through the pure in-memory `paperpilot-wasm` engine (running 100% locally in the mobile WebView / PWA).
2. Providing a mobile-optimized **PaperPilot Embed & Standalone Mobile Touch Widget**:
   - Zero native compilation dependencies for core document processing.
   - Touch-first UI with ≥48px touch targets, bottom-sheet parameter drawers, safe-area inset protection, and single-tap actions.
   - Built-in emergency toolset: Scan to PDF, Sign, Merge, Split/Delete, Rotate, Clean/Remove Blank, Compress.
   - Seamless integration with native mobile capabilities: HTML5 camera capture, file picker, and native `navigator.share()`.

---

## 2. Emergency Mobile Toolset Taxonomy

Mobile focuses exclusively on high-frequency, on-the-go tools:

| Category | Tool | Mobile Action / Value |
| :--- | :--- | :--- |
| **Capture** | `images_to_pdf` / Camera Scan | Sequential camera snapshot to deskewed, clean PDF |
| **Security & Legal** | `pdf_sign` | Draw touch signature or tap saved signature onto document |
| **Security & Privacy**| `pdf_sanitize` / `document_pii_scan` | Strip metadata & audit sensitive personal info before sharing |
| **Organization** | `pdf_merge` | Combine 2 or more files into a single bundle |
| **Page Surgery** | `pdf_rotate`, `pdf_delete_pages`, `pdf_reorder_pages` | Fix upside-down orientation, remove stray/blank pages |
| **Optimization** | `pdf_compress` | Shrink document size (<2MB) for instant email/chat dispatch |

---

## 3. Architecture & Data Flow

```
┌────────────────────────────────────────────────────────┐
│           Mobile Device (Android / iOS / PWA)          │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │    PaperPilot Mobile Touch UI (Svelte 5 / Embed)  │  │
│  │   • Touch-friendly Bottom Sheet                  │  │
│  │   • Camera Snap Intake (<input capture="camera">)│  │
│  │   • Finger Draw Signature Canvas                 │  │
│  │   • Native Share Button (navigator.share)        │  │
│  └──────────────────────────┬───────────────────────┘  │
│                             │ In-Memory Uint8Array     │
│                             ▼                          │
│  ┌──────────────────────────────────────────────────┐  │
│  │           PaperPilot In-Memory WASM Engine        │  │
│  │               (paperpilot-wasm.wasm)              │  │
│  │   • Pure In-Memory Lopdf & Image Processing       │  │
│  │   • Zero Network Overheads (100% Offline)         │  │
│  │   • Zero Native Cross-Compilation Friction        │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

---

## 4. Technical Requirements

### 4.1 Mobile-Compatible Embed Demonstration (`apps/embed`)
- **Responsive Viewport Support**: Ensure `Widget.svelte` adapts gracefully from embedded iframe mode down to mobile viewports (`max-width: 480px`).
- **Touch Target Minimums**: All interactive elements (tool selection chips, file upload buttons, slider thumbs, action triggers) must satisfy ≥48px minimum touch dimensions.
- **Swipeable Bottom Drawer**: When a tool is selected, its configuration parameters slide up from the bottom (sheet modal) with a tactile drag pill, rather than awkward sidebars.
- **Native Sharing**: Add a dedicated `Share` action that invokes `navigator.share({ files: [resultFile] })` on compatible mobile platforms, with instant fallback to standard download.

### 4.2 In-Memory WASM Execution in `apps/desktop`
- Update `OperationsPanel.svelte`: When running in mobile mode (`appState.isMobile`), bypass `process_mobile_buffer` and execute directly via `WasmPdfEngine` using in-memory `Uint8Array` buffers.
- Eliminates Android content URI / file path conversion failures.

### 4.3 Automated Verification
- Vitest unit tests for mobile WASM runner buffer handoffs.
- Playwright mobile emulation tests (`viewport: { width: 375, height: 667 }`, touch enabled) validating:
  1. File intake & camera scan button display.
  2. In-memory WASM execution of emergency tools (Rotate, Compress, Merge, Sanitize).
  3. Result download and `navigator.share` trigger.
