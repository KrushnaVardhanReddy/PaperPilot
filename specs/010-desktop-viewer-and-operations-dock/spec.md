# Spec: Phase 4.F — Advanced Desktop Viewer & Operations Dock

## Objective
Provide a unified, modern desktop workstation for viewing, navigating, annotating, and executing PDF operations with instant visual feedback and clear destination controls.

## Core Components

### 1. Unified Canvas Viewer (`PdfViewer.svelte`)
- Renders PDF pages using `pdfjs-dist` inside an HTML5 `<canvas>` element.
- Supports zoom levels (50% to 300%), fit-to-width, and high-DPI retina display scaling.
- Interactive form layer (`PdfFormLayer.svelte`) renders interactive AcroForm widgets (`/Tx`, `/Btn`) directly on the page coordinates.

### 2. In-Canvas Search Bar (`PdfSearchBar.svelte`)
- Global `Ctrl+F` / `Cmd+F` opens a floating search bar in the top-right corner of the viewer canvas.
- Extracts text asynchronously across all document pages and highlights matching indices.
- Navigation buttons (`‹` / `›`) jump the active page directly to match occurrences.

### 3. Visual Split-Diff Slider (`PdfVisualDiff.svelte`)
- Opened via `Ctrl+D` or by clicking **Compare Documents** in the operations panel.
- Renders Page N of Document A and Document B side-by-side with an interactive draggable split slider for pixel-perfect comparison of revisions.

### 4. Right-Docked Operations Panel (`OperationsPanel.svelte`)
- **Stirling-PDF Parity Directory**: Displays all 44 tools organized by category pills (`⚡ Quick`, `📑 Pages`, `✍️ Edit`, `🗜️ Optimize`, `🔒 Security`, `🔄 Convert`, `🧠 AI`).
- **Interactive Inspector Mode**: Clicking a tool enters parameter configuration.
- **Native Save-As Dialog for Merge & Split**:
  - `merge` and `split` invoke `@tauri-apps/plugin-dialog` `save()` to let users pick the exact target directory and filename before running.
  - Cancelling the dialog safely aborts execution.
- **Editable Output Path for Single-File Tools**:
  - Auto-suggests the output file path in the source document's directory with the correct extension (`.docx`, `.xlsx`, `.md`, `.txt`, `.pdf`).
  - Pre-filled in an editable text input before the Run button.
- **Creation Tools Unblocked**: `md_to_pdf`, `html_to_pdf`, and `img_to_pdf` open native system file dialogs for input files without requiring an already opened PDF.
