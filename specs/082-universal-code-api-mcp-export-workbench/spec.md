# Spec 082: Universal "Export as Code / API / MCP Prompt" Workbench

## 1. Executive Summary & Vision
PaperPilot operates across 5 primary interfaces: Desktop, Embed Web, CLI, MCP Server, and REST Gateway. While users configure actions visually (in `EditableActionCard` and `GlobalCommandBar`) or assemble multi-step workflows (in `FlowSelector`), developers and AI agents frequently need to replicate these exact operations in shell scripts, automation code, or agent workflows.

Spec 082 establishes the **Universal Code / API / MCP Export Workbench**:
Every action configured in PaperPilot—whether an individual tool invocation (e.g. Bates Stamping, Redaction, Compression, Format Conversions) or a compound multi-step pipeline—can be converted into live, ready-to-run code snippets across:
1. **cURL / HTTP REST Gateway**: Multi-part upload and JSON endpoints (`http://localhost:8080/api/v1/...`).
2. **Native CLI Command**: `paperpilot <subcommand>` with accurate flags and options.
3. **AI Agent MCP Prompt & Tool Call**: Natural language instructions + raw JSON RPC tool arguments for Claude Desktop, ChatGPT, or Cursor.
4. **Web Embed & JS API**: `window.PaperPilot.executeTool(...)` and `window.PaperPilot.runPipeline(...)` calls.

---

## 2. Core Architecture: `snippetGenerator.ts`

A centralized, headless snippet generation engine (`apps/desktop/src/lib/utils/snippetGenerator.ts`):

```typescript
export interface SnippetTarget {
  toolId: string; // e.g. 'pdf_bates', 'pdf_compress', 'pdf_rotate', 'document_workflow'
  params: Record<string, unknown>;
  inputFile?: string;
  outputFile?: string;
  inputs?: string[];
}

export interface GeneratedSnippets {
  cli: string;
  curl: string;
  mcpToolCall: string;
  mcpAgentPrompt: string;
  embedJs: string;
}

export function generateSnippets(target: SnippetTarget): GeneratedSnippets;
```

### Parameter Mapping Rules:
- **CLI**: Maps tool ID and parameters to subcommand and kebab-case flags (`--angle 90`, `--prefix "CONF-"`, `--quality 80`).
- **cURL**: Maps parameters to `-F "file=@..."` multipart upload or JSON payload targeting Gateway endpoints (`/api/v1/...`).
- **MCP Tool Call**: Maps parameters into JSON RPC `{ "name": "<tool_name>", "arguments": { ... } }`.
- **MCP Prompt**: Formats an unambiguous prompt instructing the AI agent to execute the tool with given constraints and files.
- **Embed JS**: Formats `await window.PaperPilot.executeTool('<tool_id>', { ... })` or `window.PaperPilot.runPipeline(...)`.

---

## 3. UI Component Integration

### 3.1 Individual Tool Action Cards (`apps/desktop/src/lib/components/EditableActionCard.svelte`)
- Action button: `</> Code / API` (id: `action-export-code-btn`) placed in the action card header/actions bar next to the "Execute" button.
- Updates dynamically as the user modifies sliders, dropdowns, input boxes, or checkboxes in the action card.
- Opens `ExportCodeModal` displaying code for that specific single tool.

### 3.2 Modal Dialog (`apps/desktop/src/lib/components/pipeline/ExportCodeModal.svelte`)
- Extended to support either a multi-step pipeline (`pipelineState`) or a single `SnippetTarget`.
- Clean drawer/modal design with tab switching (`cURL`, `CLI`, `MCP Agent`, `JavaScript`).
- Line-wrapped code preview with 1-click "Copy to Clipboard" button with feedback.
- Accessible dismissal via `Esc`, click outside backdrop, or `Close` button.

---

## 4. Delivery Phases
- **Phase 5.9.69D (Pipeline Code Export - Complete)**: Implemented `ExportCodeModal.svelte` and wired into `FlowSelector.svelte` for visual pipelines.
- **Phase 5.9.71 (Universal Tool Code Workbench - In Progress)**: Implement headless `snippetGenerator.ts`, wire into `EditableActionCard.svelte` for all individual tools, add comprehensive unit tests, and E2E Playwright verification.

---

## 5. Verification & Tests
- Vitest unit tests in `apps/desktop/tests/snippetGenerator.test.ts` testing CLI, cURL, MCP, and JS formatting for all primary tools (Rotate, Bates, Compress, Redact, Watermark, Merge, Split, etc.).
- Playwright E2E test verifying modal opening, tab switching, and clipboard payload verification from an individual action card.
