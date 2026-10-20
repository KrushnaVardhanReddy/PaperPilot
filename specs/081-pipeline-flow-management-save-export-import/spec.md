# Spec 081: Universal Pipeline Flow Management & Execution (All 5 Interfaces + Free vs Enterprise Tiers)

## 1. Context & Business Model Alignment
PaperPilot features a multi-step visual pipeline runner in Desktop (`apps/desktop`) and Web Embed (`apps/embed`). Users chain operations together into custom document automation pipelines (e.g., Legal Discovery Bates Stamping, Financial Redaction & Encryption, Scan Cleanup & Pagination).

To fulfill PaperPilot's strict **Penta-Interface Mandate** (*CLI, MCP, REST Gateway, WASM/Embed, and Desktop*), pipeline flows must not be confined to desktop local memory. A flow created in the Desktop UI or exported to a portable JSON recipe must be 100% executable across **all 5 interfaces** without requiring intermediate disk roundtrips.

To align with PaperPilot's open-core business model (*"All 50+ tools and core desktop/CLI/MCP workflows are 100% free; monetize team collaboration, audit trails, and enterprise infrastructure"*), pipeline flow management is structured into two clean tiers:

### Tier Breakdown

| Capability | Free Community (Local / Self-Hosted) | Enterprise / Team (Cloud) |
| :--- | :---: | :---: |
| **Penta-Interface Execution** | ✅ All 5 interfaces (Desktop, Embed, CLI, MCP, REST) | ✅ All 5 interfaces |
| **Save Custom Flows** | ✅ Unlimited local storage (Webview `localStorage`, CLI config path `~/.paperpilot/flows/`) | ✅ Unlimited cloud workspace storage |
| **Export / Import Flows** | ✅ Standard portable JSON recipe files (`.json`) | ✅ Standard JSON + Team Workspace Sync |
| **Default Industry Presets** | ✅ Built-in presets (Legal Discovery, Clean Scan, Redact & Protect) | ✅ Custom organization-wide enforced presets |
| **Central Flow Library** | ❌ None (Local only) | 👑 **Live Cloud Sync across team members** |
| **Role-Based Access Control** | ❌ None (Single-user) | 👑 **Lock down approved flows / edit permissions** |
| **Compliance Audit Logging** | ❌ None | 👑 **Tamper-evident logs of who executed what flow** |
| **Headless Cloud Execution** | Manual local run / self-hosted daemon | 👑 **Webhook / Cron automation in hosted cloud** |

---

## 2. Universal Data Model & Recipe JSON Schema (`SavedPipeline`)

A single, canonical recipe schema is shared across TypeScript (`apps/desktop`, `apps/embed`) and Rust (`paperpilot-core`, `paperpilot-pdf`, `paperpilot-cli`, `paperpilot-mcp`, `paperpilot-gateway`):

### TypeScript Definition (`SavedPipeline`)
```typescript
export interface PipelineStep {
  id: string;                      // crypto.randomUUID()
  operationId: OperationId;        // e.g. 'pdf_bates', 'pdf_compress', 'pdf_encrypt'
  label: string;                   // User-visible step label
  icon: string;                    // UI icon emoji
  params: Record<string, unknown>; // Operation parameters (e.g. prefix, level, password)
}

export interface SavedPipeline {
  id: string;               // e.g. "preset-legal-discovery" or UUID
  name: string;             // User-visible flow title
  description?: string;     // Purpose / notes
  steps: PipelineStep[];    // Sequential operations
  updatedAt: string;        // ISO 8601 timestamp
  isPreset?: boolean;       // System default vs user-created
  teamId?: string;          // (Enterprise) Org/Team ID for cloud-synced flows
  accessLevel?: 'viewer' | 'editor'; // (Enterprise) RBAC indicator
}
```

### Rust Struct Definition (`paperpilot_core::workflow`)
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStep {
    pub id: String,
    pub operation_id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavedWorkflow {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub steps: Vec<WorkflowStep>,
    pub updated_at: String,
    #[serde(default)]
    pub is_preset: bool,
    #[serde(default)]
    pub team_id: Option<String>,
}
```

---

## 3. Standard Industry Default Presets

Every interface includes or recognizes 3 standard, production-grade industry recipes:

1. **Legal Discovery & Bates Stamping (`preset-legal-discovery`)**:
   - `pdf_bates` (Prefix: `CASE-2026-`, start: 1, padding: 6)
   - `pdf_watermark` (Text: `CONFIDENTIAL`, opacity: 0.3, font_size: 48)
   - `pdf_linearize` (Optimize for fast web viewing)
   - *Use Case*: Litigation discovery prep and court document stamping.

2. **Scan Cleanup & Pagination (`preset-clean-scan`)**:
   - `pdf_remove_blank` (Sensitivity: 95)
   - `pdf_page_numbers` (Format: `Page {n} of {total}`, Position: `bottom-center`)
   - `pdf_compress` (Mode: `medium`)
   - *Use Case*: Clean up double-sided scanned documents, paginate, and optimize for distribution.

3. **Redaction & Password Protection (`preset-redact-protect`)**:
   - `pdf_redact` (Words: `SSN, confidential, invoice`)
   - `pdf_flatten` (Bake annotations and form fields permanently)
   - `pdf_encrypt` (User password: `changeme`, 256-bit AES)
   - *Use Case*: Scrub sensitive keywords, flatten interactive layers, and encrypt before sharing.

---

## 4. Penta-Interface Specification

### Interface 1: Desktop Application (`apps/desktop`)
- **Flow Selector Toolbar (`FlowSelector.svelte`)**:
  - Placed under pipeline header. Dropdown groups `<optgroup label="Standard Presets">` and `<optgroup label="My Saved Flows">`.
  - Action buttons: `💾 Save Flow`, `⬇️ Export JSON`, `⬆️ Import JSON`, `🗑️ Delete Flow` (custom only), `✨ New Pipeline`.
- **Save Flow Modal (`SaveFlowModal.svelte`)**:
  - Modal with Name, Description, and step sequence preview badges. Saves to `localStorage['paperpilot_saved_flows']`.
- **Export/Import**:
  - Native file export/import as `<flow-name>.json` validating recipe schema.

### Interface 2: Web Embed & WASM (`apps/embed`)
- **Declarative Recipe Loading via Web Component / Embed API**:
  ```html
  <paperpilot-widget pipeline="preset-legal-discovery"></paperpilot-widget>
  ```
  or via JS API:
  ```javascript
  const flow = await fetch('/recipes/clean_scan.json').then(r => r.json());
  window.PaperPilot.runPipeline({
    file: myPdfFile,
    recipe: flow, // or pipeline: flow.steps
    onProgress: (stepIdx, step, percent) => { ... }
  });
  ```
- Preset resolver built into embed library so host sites can embed automated workflows directly.

### Interface 3: Native CLI (`paperpilot-cli`)
- **Workflow Command**:
  ```bash
  # Execute via preset ID
  paperpilot workflow --preset legal-discovery --input input.pdf --output output.pdf

  # Execute via external JSON recipe file
  paperpilot workflow --recipe ./my_flow.json --input input.pdf --output output.pdf

  # Manage saved CLI recipes
  paperpilot workflow list
  paperpilot workflow export legal-discovery --output ./legal.json
  paperpilot workflow import ./custom.json
  ```
- **Directory Watch Integration**:
  ```bash
  paperpilot watch --dir ./inbox --out ./outbox --recipe ./my_flow.json
  ```

### Interface 4: Agent MCP Server (`paperpilot-mcp`)
- **MCP Tool `document_workflow`**:
  - Tool definition with schema:
    ```json
    {
      "name": "document_workflow",
      "description": "Execute a declarative multi-step pipeline workflow (preset or JSON recipe) against a PDF",
      "parameters": {
        "type": "object",
        "properties": {
          "input": { "type": "string", "description": "Path to input PDF" },
          "output": { "type": "string", "description": "Path to output PDF" },
          "preset": { "type": "string", "enum": ["legal-discovery", "clean-scan", "redact-protect"], "description": "Named industry workflow preset" },
          "recipe": { "type": "object", "description": "Custom SavedPipeline JSON object with steps and parameters" }
        },
        "required": ["input", "output"]
      }
    }
    ```
  - Enables AI agents to run compound operations (e.g. Bates + Watermark + Linearize) in a single RPC roundtrip.

### Interface 5: HTTP REST Gateway & Edge (`paperpilot-gateway` / `apps/edge`)
- **REST Endpoints**:
  - `POST /api/v1/workflow/run`: Accepts `multipart/form-data` with `file` and either `preset` string or `recipe` JSON string.
  - `POST /api/v1/workflow/run-json`: Accepts JSON `{ "input": "/path/in.pdf", "output": "/path/out.pdf", "preset": "legal-discovery" }`.
  - `GET /api/v1/workflow/presets`: Returns available built-in presets and step configurations.

---

## 5. Phased Delivery Strategy

- **Phase 5.9.69A (Desktop State & Presets)**: ✅ **COMPLETED** (PR #220, commit `460ec1d`).
- **Phase 5.9.69B (Desktop & Embed UI Flow Selector)**: `FlowSelector.svelte`, `SaveFlowModal.svelte`, Embed preset prop, and Playwright E2E tests.
- **Phase 5.9.69C (Core Workflow Engine & CLI/MCP/REST Parity)**:
  - In-memory workflow engine in `paperpilot-pdf` (`run_workflow`).
  - CLI subcommand `paperpilot workflow`.
  - MCP tool `document_workflow`.
  - REST Gateway endpoint `/api/v1/workflow/run`.
  - Penta-interface E2E real assertions in `tools/penta-interface-e2e`.
