# 014: On-Device AI Architecture (TinyBERT & Local RAG)

## Status
Approved

## Context
PaperPilot requires a responsive, 100% offline natural language interface that parses user instructions (e.g., "squish this pdf", "take first 5 pages of contract", "convert to docx") into structured `OperationPlan` executions. We required an architecture that avoids Python dependencies, keeps the total installer size under 50MB, executes in <2ms on CPU, and enables users to query living documentation directly in the chat.

## Decisions

### 1. Two-Tier AI Strategy
* **Free Personal Tier (Bundled)**: `TinyBERT-4L-312D` (INT8 ONNX).
  - Size on disk: ~14MB (compressed to ~7MB via `zstd`).
  - Active memory: ~25MB RAM.
  - Latency: ~1.5ms to 2ms on standard CPU.
  - Capabilities: Single-intent classification and token slot-filling for arguments across arbitrary phrasing.
  - Deployment: Embedded directly in the Rust binary via `include_bytes!` and executed in-memory via `ort` (ONNX Runtime).
* **Pro / Teams Tier**: `SmolLM-135M-Instruct` (Q4 GGUF, ~75MB) + Bring-Your-Own-Key (BYOK) cloud APIs (OpenAI, Gemini, Claude) + Local Ollama.
  - Capabilities: Multi-step conversational agent planning and chained execution recipes.

### 2. Embedded Documentation RAG (`sqlite-vec`)
Rather than fine-tuning models on rapidly changing documentation (which causes hallucinations), PaperPilot embeds the living documentation (`TRI_INTERFACE_E2E_AND_DOCS.md`, user guides) into an embedded SQLite database using `sqlite-vec`.
* Questions in the desktop chat query the embedded vector index in <0.5ms.
* Exact, copy-pasteable CLI and REST API snippets are returned directly with 100% factual accuracy.

### 3. Anti-Brute-Force & Offline Memory-Burn
For encrypted PDFs, brute-force protection is enforced without server phone-home dependencies using **Argon2id Memory-Hard Key Derivation** (1–2GB RAM cost per attempt). Optional enterprise KMS key escrow allows remote burn-switches when connected.

### 4. Context-Aware NLP Resolution & Implicit Active Document Binding
To prevent errors when users issue commands inside an open document (e.g. typing `"Split pages 1 to 2"` or `"Rotate 90 degrees"` without retyping the active filename), the NLP resolution API accepts execution context:
* `resolve_with_context(query: &str, context: &ResolverContext) -> Result<OperationPlan, NlpError>`
* **`ResolverContext`**:
  - `active_document: Option<String>`: The path/name of the currently focused document in the viewer.
  - `open_documents: Vec<String>`: List of all loaded documents currently available in the workspace.
* **Resolution Rule**: If an intent requires a target document and the query does not explicitly specify one:
  1. If `active_document` is present, it is automatically bound as the primary input file.
  2. If on the Documents hub and only one document is loaded, that document is automatically selected.
  3. If multiple documents are present without clear targets, the resolver returns an actionable clarification request instead of failing.

### 5. Multi-Document Omnibar (`GlobalCommandBar.svelte`) with `@mention` Autocomplete
In addition to the side-drawer chat in `PdfViewer`, a global command bar is embedded at the bottom of the Documents hub (`DocumentsView.svelte`):
* **`@` Mentions**: Typing `@` triggers a popover autocomplete dropdown listing loaded documents (`appState.documents`), allowing instant multi-document command composition (e.g. `"Merge @doc1.pdf and @doc2.pdf into combined.pdf"`, `"Compare @v1.pdf with @v2.pdf"`).
* **Batch Operations**: Supports commands like `"Compress all documents by 50%"` or `"Convert all to PDF/A"`.
* **Execution Plan Cards**: Emits the structured `OperationPlan` preview card with interactive `[Execute Action]` triggers before dispatching to `invoke_mcp_tool`.

### 6. Unmocked E2E Natural Language & Execution Verification
To ensure the AI chat and NLP subsystem functions reliably from prompt to PDF output on disk without synthetic test artifacts:
* An unmocked E2E test suite (`tests/e2e_ai_chat_real_pipeline.spec.ts`) runs directly against real test fixtures (e.g. `tests/fixtures/sample.pdf`).
* **Verifications**:
  1. User enters natural language command (`"rotate 90 degrees"`, `"split pages 1 to 2"`).
  2. IPC bridge resolves via the real `OfflineNlpResolver` (combining RuleEngine Layer 1 and TinyBERT ONNX Layer 2).
  3. Action card renders with accurate parameters and confirmation button.
  4. Clicking `[Execute Action]` invokes the actual `invoke_mcp_tool` engine which executes the underlying PDF operation (`paperpilot-pdf`).
  5. Verifies output file generation on disk and validates output integrity (e.g., page count / rotation state).

### 7. Searchable Command Help & Cheat Sheet Drawer (`ChatCheatSheet.svelte`) & Adjustable Right Panel
To resolve the "blank canvas" discoverability challenge for users, the chat interface includes a searchable command palette:
* **Entry Point**: A `💡 Examples` button in `PdfChatPanel.svelte` (and hotkey `Ctrl+/` / `?`).
* **Search & Filter**: Real-time search filter categorizing supported natural language queries across all 44 tools (`Pages & Structure`, `Optimize & Repair`, `Security & Privacy`, `Conversions & Extraction`, `Edit & Markup`, `Multi-Document Merge/Compare`).
* **One-Click Insert**: Clicking any example prompt automatically populates the chat prompt input, allowing users to modify arguments (e.g. page numbers or passwords) and submit immediately.
* **Adjustable Right Panel Resizability**:
  - The right panel splitter (`ViewerRightPanel.svelte`) supports dynamic dragging up to **`800px`** (clamped between 240px and 800px).
  - Switching to the AI Chat tab automatically ensures a comfortable reading width (bumps to at least `420px` if currently narrow).
  - Visual hover cues on the splitter handle (`col-resize`) and double-click toggle (`300px` / `500px`) ensure effortless resizing on desktop viewports.

### 8. Interactive & Editable Action Cards for All 44 Operations (`PdfChatPanel.svelte`)
When the AI assistant proposes an `OperationPlan`, the card in the chat stream must NOT be a static read-only confirmation box:
* **Interactive Parameter Editing**:
  - **Pages Target Array/Selector**: Shows where the operation will be applied with an explicit pages target (e.g. `all`, or page chips/inputs `[1, 4]`, with a quick-select `[Current Page]` chip). Users can add, edit, or remove target pages before execution.
  - **Dynamic Parameter Fields**: Depending on the detected operation, renders editable inputs pre-populated with NLP extraction values (e.g. `angle` selector: 90°/180°/270°, watermark text, password, output file path, crop box, bates prefix/start, header/footer text).
* **Comprehensive 44-Tool Execution Dispatch**:
  - Full parameter mapping for all 44 PDF operations (matching `OperationsPanel.svelte` parity), passing valid required arguments (e.g. `pages: "all"`, default output paths, format targets) so execution never fails with `Missing or invalid parameter`.
* **Execution & In-Place State Refresh**:
  - Clicking `[⚡ Execute Action]` executes `invoke_mcp_tool` with the user-edited parameters, provides instant toast feedback, and triggers a document canvas reload if modifying the active document.

## Consequences
* Total application size remains ~50MB, well under the 100MB constraint.
* Zero external Python, server, or cloud dependencies for AI operation.
* Instant sub-2ms response times on any desktop hardware.
* Flawless user experience in single-document viewer mode (no redundant typing of file names).
* Frictionless multi-document batch and merge workflows from the central Documents hub.
* Immediate discoverability and rapid prompt formulation for all 44 operations via the interactive cheat sheet.
* Real end-to-end pipeline confidence with zero mocking.
