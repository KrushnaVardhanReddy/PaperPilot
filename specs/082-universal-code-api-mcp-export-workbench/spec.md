# Spec 082: Universal "Export as Code / API / MCP Prompt" Workbench

## 1. Executive Summary & Vision
PaperPilot operates across 6 interfaces: Desktop, Embed, CLI, Agent MCP Server, REST Gateway, and WASM/Edge. While users interact visually with individual tools (in `EditableActionCard` and `GlobalCommandBar`) and multi-step workflows (in `PipelineView`), developers and AI agents frequently need to replicate these exact operations in terminal scripts, backend code, or automated agent pipelines.

Spec 082 establishes the **Universal Code / API / MCP Export Workbench**:
Every action configured in PaperPilot—whether an individual tool invocation (e.g. Bates Stamping, Redaction, Compression) or a compound multi-step pipeline—can be converted into live, ready-to-run code snippets across:
1. **cURL / HTTP REST Gateway**: Multi-part upload and JSON RPC calls.
2. **Native CLI Command**: `paperpilot <subcommand>` with flags.
3. **AI Agent MCP Prompt & Tool Call**: Natural language instructions + raw JSON RPC tool arguments for Claude Desktop, ChatGPT, or Cursor.
4. **Web Embed & JS API**: `<paperpilot-widget>` declarative markup and `window.PaperPilot.runPipeline(...)` calls.

---

## 2. Core Architecture: `snippetGenerator.ts`

A centralized, headless snippet generation engine (`apps/desktop/src/lib/utils/snippetGenerator.ts` and `apps/embed/src/snippetGenerator.ts`):

```typescript
export interface SnippetTarget {
  toolId: string; // e.g. 'pdf_bates', 'pdf_compress', 'document_workflow'
  params: Record<string, unknown>;
  inputFile?: string;
  outputFile?: string;
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
- **CLI**: Maps tool ID and camelCase/snake_case parameters to kebab-case arguments (`--prefix`, `--font-size`, `--opacity`).
- **cURL**: Maps parameters to `-F "key=value"` form fields or JSON payload targeting `http://localhost:8080/api/v1/<endpoint>`.
- **MCP Tool Call**: Maps parameters into JSON RPC `{ "name": "<tool_name>", "arguments": { ... } }`.
- **MCP Prompt**: Formats a human-like, unambiguous prompt instructing the AI agent to execute the tool with given constraints.
- **Embed JS**: Formats `await window.PaperPilot.executeTool('<tool_id>', params)`.

---

## 3. UI Component Integration

### 3.1 Pipeline Builder (`apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`)
- Action button: `</> Code / API` (id: `flow-export-code-btn`).
- Targets compound multi-step pipelines:
  - CLI: `paperpilot workflow run --recipe flow.json --input input.pdf --output output.pdf`
  - REST: `POST /api/v1/workflow/run`
  - MCP: Tool `document_workflow` with inline recipe or preset ID.

### 3.2 Individual Tool Action Cards (`apps/desktop/src/lib/components/EditableActionCard.svelte`)
- Action button: `</> View Code` placed next to the "Execute" button.
- Updates dynamically as the user modifies sliders, dropdowns, input boxes, or checkboxes in the action card.
- Opens `ExportCodeModal` displaying code for that specific single tool.

### 3.3 Modal Dialog (`apps/desktop/src/lib/components/ui/ExportCodeModal.svelte`)
- Clean modal design with tab switching (`cURL`, `CLI`, `MCP Agent`, `JavaScript`).
- Syntax highlighted code preview with line numbers and 1-click "Copy to Clipboard" button.
- Dismissible via `Esc`, click outside, or `Close` button.

---

## 4. Delivery Phases

- **Phase 5.9.69D (Pipeline Code Export)**: Implement `snippetGenerator.ts` for workflows, create `ExportCodeModal.svelte`, and wire into `FlowSelector.svelte`.
- **Phase 5.9.71 (Universal Tool Code Workbench)**: Wire `ExportCodeModal.svelte` and `snippetGenerator.ts` into `EditableActionCard.svelte` and `GlobalCommandBar.svelte` across all 50+ tools.

---

## 5. Verification & Tests
- Unit tests in `snippetGenerator.test.ts` testing CLI, cURL, MCP, and JS formatting for all Wave 1 & Wave 2 tools.
- Playwright E2E test verifying modal opening, tab switching, and clipboard payload verification.
