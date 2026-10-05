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

### 5. AI Chat Panel & Assistant Drawer (`PdfChatPanel.svelte`)
- **Widget**: Integrated via zero-dependency [`quikchat`](https://github.com/deftio/quikchat) to avoid wheel reinvention and keep Tauri bundle overhead negligible.
- **Placement**: Collapsible right or bottom drawer toggleable via top toolbar / shortcut.
- **Capabilities**:
  - Multi-turn conversational Q&A against active document text (offline via `bert-hash-nano-embeddings` or Pro via local Ollama / cloud LLM).
  - Rich Markdown formatting with syntax highlighting, lists, and table rendering.
  - Per-document conversation history persistence across tab switching.
  - Interactive Action Confirmation cards embedded directly in the message flow (e.g. "Run Merge", "Apply Redaction").

### 6. Developer Mode: Tri-Interface Action Inspector & CodeGen (`ActionInspector.svelte`)
- **Trigger**: Toggleable via `Developer Mode` switch in settings, top toolbar badge (`🛠️ Dev Mode`), or hotkey `Ctrl+Shift+I`.
- **Purpose**: Bridge visual GUI actions to repeatable automation code (CLI, cURL REST API, MCP tool call, and Python/Bash scripts) with one-click copying.
- **UI Architecture**:
  - Embedded collapsible drawer or tab inside `OperationsPanel.svelte`.
  - **Live Reactive CodeGen**: As users adjust sliders (e.g., opacity, page ranges, passwords, crop boxes) or pick files in the GUI, the generated snippet updates instantly in real time.
- **Multi-Tab CodeGen Formats**:
  1. **CLI Tab**:
     - Formats exact bash command with flags matching active inputs:
       ```bash
       paperpilot watermark --input contract.pdf --output out.pdf --text "CONFIDENTIAL" --pages 1-5
       ```
     - Includes a toggle: `Wrap in batch loop (all *.pdf in folder)` generating a 3-line bash `for file in *.pdf; do ... done` snippet.
  2. **REST API (cURL) Tab**:
     - Formats full `curl` command with method, endpoint, headers, and JSON body targeting PaperPilot's local/gateway HTTP API:
       ```bash
       curl -X POST http://localhost:8080/api/v1/pdf/tools/pdf_watermark \
         -H "Content-Type: application/json" \
         -d '{"input": "contract.pdf", "output": "out.pdf", "text": "CONFIDENTIAL", "pages": [1,2,3,4,5]}'
       ```
  3. **MCP Tool Call (JSON) Tab**:
     - Produces native Model Context Protocol JSON payload ready to paste into Claude Desktop, Cursor, Antigravity, or any MCP client:
       ```json
       {
         "name": "pdf_watermark",
         "arguments": {
           "input": "contract.pdf",
           "output": "out.pdf",
           "text": "CONFIDENTIAL",
           "pages": [1, 2, 3, 4, 5]
         }
       }
       ```
  4. **Code / Scripting Tab**:
     - Generates copyable scripts in **Python** (`requests` or `subprocess`), **Node.js/TypeScript**, and **Rust** (`paperpilot-core` crate call).
- **Audit & Workflow History ("Action Log")**:
  - Retains a chronological list of recent operations executed during the desktop session.
  - "Export Workflow as Pipeline": Allows exporting multi-step user actions (e.g. *Decrypt -> Extract Pages -> Watermark*) as an executable Bash script or a multi-step MCP execution plan.

### 7. Interactive Page Organizer & Thumbnail Ergonomics (`PdfThumbnails.svelte`)
To ensure effortless page manipulation directly in the document viewport without opening separate menus:
- **Visual Drag & Drop Reordering**:
  - Drag grip indicator (`⋮⋮`) with `cursor: grab` / `cursor: grabbing`.
  - Blue animated insertion drop-line showing exact target position before release.
- **Persistent Quick-Action Controls**:
  - High-contrast action buttons on every page thumbnail:
    - Rotate single page (`🔄` 90° clockwise in-place).
    - Delete single page (`🗑️` with instant viewer refresh and undo toast).
- **Right-Click Context Menu**:
  - Right-clicking any page thumbnail opens a contextual action menu:
    - `Rotate Clockwise (90°)`
    - `Rotate Counter-Clockwise (270°)`
    - `Rotate 180°`
    - `Duplicate Page`
    - `Delete Page`
    - `Extract This Page As New Document`
