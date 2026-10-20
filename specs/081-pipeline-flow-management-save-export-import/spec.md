# Spec 081: Pipeline Flow Management (Community Free vs Enterprise Team Workspaces)

## 1. Context & Business Model Alignment
PaperPilot features a multi-step visual pipeline runner in Desktop (`apps/desktop`) and Web Embed (`apps/embed`). Users construct custom document automation pipelines (e.g., Legal Discovery Bates Stamping, Financial Redaction & Encryption, Scan Cleanup & Pagination).

Currently, pipelines only exist in memory during a session and cannot be saved, exported, imported, or selected from presets.

To align with PaperPilot's open-core business model (*"All 50+ tools and core desktop workflows are 100% free; monetize team collaboration, audit trails, and enterprise infrastructure"*), pipeline flow management is structured into two clean tiers:

### Tier Breakdown

| Capability | Free Community (Local) | Enterprise / Team (Cloud) |
| :--- | :---: | :---: |
| **Save Custom Flows** | ✅ Unlimited (Stored in browser / webview `localStorage`) | ✅ Included |
| **Export / Import Flows** | ✅ Manual file-based (`.json` file export & import) | ✅ Included |
| **Default Industry Presets** | ✅ Out-of-the-box standard presets (Legal, Scan, Security) | ✅ Custom company-managed presets |
| **Central Flow Library** | ❌ None | 👑 **Live Cloud Sync across team members** |
| **Role-Based Access Control** | ❌ None | 👑 **Lock down approved flows / edit permissions** |
| **Compliance Audit Logging** | ❌ None | 👑 **Tamper-evident logs of who executed what flow** |
| **Headless Cloud Execution** | Manual local run | 👑 **Webhook / Cron automation in hosted cloud** |

---

## 2. Technical Architecture & Data Model

### Pipeline Flow JSON Schema (`SavedPipeline`)
```typescript
export interface SavedPipeline {
  id: string;               // e.g. "preset-legal-bates" or crypto.randomUUID()
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
1. **Legal Discovery & Bates Stamping (`preset-legal-discovery`)**:
   - `pdf_bates` (Prefix: `CASE-2026-`, start: 1)
   - `pdf_watermark` (Text: `CONFIDENTIAL`, opacity: 0.3, font_size: 48)
   - `pdf_linearize` (Optimize for fast web viewing)
   - *Use Case*: Litigation discovery prep and court document stamping.

2. **Scan Cleanup & Pagination (`preset-clean-scan`)**:
   - `pdf_remove_blank` (Sensitivity: 95)
   - `pdf_page_numbers` (Format: `Page {n} of {total}`, Position: `bottom-center`)
   - `pdf_compress` (Mode: `medium`)
   - *Use Case*: Clean up scanned double-sided documents, paginate, and optimize for email.

3. **Redaction & Password Protection (`preset-redact-protect`)**:
   - `pdf_redact` (Words: `SSN, confidential, invoice`)
   - `pdf_flatten` (Bake annotations and form fields permanently)
   - `pdf_encrypt` (User password: `changeme`, 256-bit AES)
   - *Use Case*: Scrub sensitive data, lock down forms, and encrypt before sharing.

---

## 3. UI Architecture (`FlowSelector.svelte` & `SaveFlowModal.svelte`)

### FlowSelector Toolbar Layout:
```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  Flow: [ Legal Discovery & Bates Stamping (3 steps) ▼ ]   [💾 Save]  [⬇️ Export]  [⬆️ Import]  [✨ New] │
└────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```
1. **Flow Dropdown**:
   - Organized into `<optgroup label="Standard Presets">` and `<optgroup label="My Saved Flows">`.
   - Selecting a preset or saved flow deep-clones steps into the canvas.
   - Shows "— Custom / Unsaved Pipeline —" when user edits steps.
2. **Action Buttons**:
   - `💾 Save Flow`: Disabled if 0 steps. Opens `SaveFlowModal.svelte` to enter Name/Description.
   - `⬇️ Export JSON`: Disabled if 0 steps. Downloads `<flow-name>.json` file directly to browser.
   - `⬆️ Import JSON`: File picker for `.json` recipe files. Validates schema, generates unique ID, saves locally, and loads.
   - `🗑️ Delete Flow`: Appears only for user-created custom flows (system presets are protected).
   - `✨ New Pipeline`: Clears current canvas to build from scratch.

---

## 4. Delivery Phases (Split Strategy)
- **Part 1 (Task 5.9.69A)**: Data model, presets, `localStorage` persistence, JSON serialization, and Vitest unit tests in `apps/desktop/src/lib/state/pipeline.svelte.ts`.
- **Part 2 (Task 5.9.69B)**: Interactive `FlowSelector.svelte` toolbar, `SaveFlowModal.svelte`, `PipelineView.svelte` integration, and Playwright E2E tests.
