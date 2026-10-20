# Spec 081: Pipeline Flow Management (Community Free vs Enterprise Team Workspaces)

## 1. Context & Business Model Alignment
PaperPilot features a multi-step visual pipeline runner in Desktop (`apps/desktop`) and Web Embed (`apps/embed`). Users construct complex custom document automation pipelines (e.g., Legal Discovery Bates Stamping, Financial Redaction & Encryption, Split & Fan-Out Watermarking).

To align with PaperPilot's open-core business model ("*All 50+ tools and core desktop workflows are 100% free; monetize team collaboration, audit trails, and enterprise infrastructure*"):

### Tier Breakdown

| Capability | Free Community (Local) | Enterprise / Team (Cloud) |
| :--- | :---: | :---: |
| **Save Custom Flows** | ✅ Unlimited (Stored in local disk/browser `localStorage`) | ✅ Included |
| **Export / Import Flows** | ✅ Manual file-based (`.json` file export & import) | ✅ Included |
| **Default Industry Presets** | ✅ Out-of-the-box standard presets | ✅ Custom company-managed presets |
| **Central Flow Library** | ❌ None | 👑 **Live Cloud Sync across team members** |
| **Role-Based Access Control** | ❌ None | 👑 **Lock down approved flows / edit permissions** |
| **Compliance Audit Logging** | ❌ None | 👑 **Tamper-evident logs of who executed what flow** |
| **Headless Cloud Execution** | Manual local run | 👑 **Webhook / Cron automation in hosted cloud** |

---

## 2. Technical Architecture & Data Model

### Pipeline Flow JSON Schema (`SavedPipeline`)
```typescript
export interface SavedPipeline {
  id: string;               // e.g. "flow-uuid" or "preset-legal-bates"
  name: string;             // User-visible flow title
  description?: string;     // Purpose / notes
  steps: PipelineStep[];    // Array of configured steps with tool operationId & params
  updatedAt: string;        // ISO 8601 timestamp
  isPreset?: boolean;       // System default vs user-created
  teamId?: string;          // (Enterprise) Org/Team ID for cloud-synced flows
  accessLevel?: 'viewer' | 'editor'; // (Enterprise) RBAC indicator
}
```

### Community Free Presets (Out-of-the-Box)
1. **Legal Discovery & Bates Stamping**:
   - `pdf_bates` (Prefix: `CASE-2024-`, start: 1)
   - `pdf_watermark` (Text: `CONFIDENTIAL`, opacity: 0.3)
   - `pdf_linearize` (Optimize for fast web viewing)
2. **Redact & Password Protect**:
   - `pdf_redact` (Terms: `SSN, confidential, invoice`)
   - `pdf_flatten` (Flatten interactive form fields & annotations)
   - `pdf_encrypt` (Password encryption)
3. **Split & Fan-Out Watermark**:
   - `pdf_split` (Split by page intervals)
   - `pdf_watermark` (Watermark each split output)

---

## 3. UI Implementation (`apps/desktop/src/lib/components/pipeline/FlowSelector.svelte`)

### Community Free Features
1. **Flow Dropdown Selector**:
   - Displays presets and custom saved flows with step counts (e.g., `Legal Discovery (3 steps)`).
   - Selecting a flow immediately populates the canvas and config panels.
2. **Local Management Toolbar**:
   - `💾 Save Flow`: Opens modal prompting for Flow Name & Description. Saves to `localStorage`.
   - `📤 Export JSON`: Downloads active flow as `<flow-name>.json` for sharing with colleagues.
   - `📥 Import JSON`: File picker allowing user to upload any `.json` flow. Validates schema and activates imported flow.
   - `🗑️ Delete Flow`: Deletes user-saved flow (presets protected).

### Enterprise Integration Hooks (Extensible for Gateway/SSO)
- Flow selector shows an unobtrusive **"Team Workspaces"** badge/dropdown option:
  - If signed in with Enterprise Gateway license: auto-fetches `/api/v1/team/pipelines` from the connected Gateway/Cloud instance.
  - If on Free Community: clicking "Team Workspaces" shows an informational modal explaining shared cloud flow libraries and SSO.

---

## 4. Verification & Testing
- Unit test for `PipelineState` flow persistence, serialization, and import validation.
- End-to-end Playwright test verifying:
  1. Saving a 3-step pipeline locally.
  2. Exporting to a `.json` file.
  3. Resetting/clearing state.
  4. Importing the `.json` file back and verifying all parameters are intact.
  5. Running the imported pipeline against a test PDF.
