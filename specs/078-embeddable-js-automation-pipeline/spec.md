# Spec 078: Client-Side Declarative Workflow & Headless Automation Pipeline for Embeddable JS

## 1. Context & Motivation
PaperPilot's embedded distribution layer (`apps/embed`, `embed.js`) currently renders interactive single-tool UI cards. However, web SaaS platforms, agencies, and enterprise portals (e.g., insurance claim intake, HR onboarding, legal contract portals) need **automated, multi-step document pipelines** that run client-side in the browser:
- **Zero Server Cost**: Compute executes 100% inside the user's browser.
- **Strict Client-Side Privacy**: Files are sanitized, stripped of PII, compressed, and watermarked *before* being uploaded to company servers.
- **Dual Mode**:
  1. **Visual Pipeline Widget**: Drop-in container with `data-pipeline='[...]'` that auto-executes on file drop and renders step progress.
  2. **Headless JavaScript API**: Programmatic `PaperPilot.runPipeline({ file, steps })` returning processed `Uint8Array` bytes.

---

## 2. Technical Architecture & Design

### 2.1 Declarative Pipeline Specification
A pipeline is an array of operation steps defined as JSON:
```json
[
  { "tool": "rotate", "angle": 90, "pages": "all" },
  { "tool": "compress", "quality": "medium" },
  { "tool": "watermark", "text": "INTERNAL AUDIT", "opacity": 0.2, "angle": 45 },
  { "tool": "extract_pages", "pages": "1-5" }
]
```

Supported pipeline operations in in-browser WASM / PDF engine:
- `merge`: Combine multiple files
- `split`: Extract specific ranges
- `rotate`: Rotate pages by 90, 180, 270 degrees
- `compress`: Stream compression and object pruning
- `watermark`: Text watermark stamping
- `extract_pages`: Extract designated pages
- `delete_pages`: Remove specified pages
- `reorder_pages`: Reverse or custom index ordering
- `crop`: Page margin cropping
- `flatten`: Flatten annotations & form fields

### 2.2 Headless API in `apps/embed/src/api.ts`
Export `runPipeline`:
```typescript
export interface PipelineStep {
  tool: string;
  [key: string]: any;
}

export interface PipelineOptions {
  file: File | Uint8Array;
  steps: PipelineStep[];
  onProgress?: (stepIndex: number, step: PipelineStep, progressPercent: number) => void;
}

export interface PipelineResult {
  success: boolean;
  data: Uint8Array;
  filename: string;
  stepsExecuted: number;
  durationMs: number;
}

export async function runPipeline(options: PipelineOptions): Promise<PipelineResult>;
```

### 2.3 Visual Drop-in Widget in `apps/embed/src/Widget.svelte`
Support `data-pipeline` and `pipeline` prop:
- When `pipeline` prop is provided:
  - Rather than displaying a grid of individual tool cards, display the **Pipeline Status View**:
    - Stepper progress bar: `[1] Rotate ➔ [2] Compress ➔ [3] Watermark`
    - Live step indicators: Pending (⚪), In Progress (🔄), Done (✅), Error (❌).
    - When file is dropped, immediately trigger the sequential pipeline.
    - Provide 1-click Download button or trigger `paperpilot:pipeline-complete` CustomEvent with `{ data, filename, durationMs }`.

### 2.4 Auto-Init Attribute Support in `apps/embed/src/embed.ts`
```html
<div 
  data-paperpilot-portal
  data-pipeline='[{"tool":"compress"},{"tool":"watermark","text":"APPROVED"}]'
  data-auto-download="true">
</div>
```

---

## 3. Acceptance Criteria & Test Plan
1. **Headless Execution**:
   - Calling `PaperPilot.runPipeline(...)` with 2+ chained operations produces valid PDF bytes matching sequential transformations.
2. **Visual Pipeline Stepper**:
   - Supplying `data-pipeline` renders a multi-step stepper UI.
   - Dropping a PDF triggers progress events and emits `paperpilot:pipeline-complete`.
3. **Automated Tests**:
   - Playwright test in `apps/embed/tests/` validating pipeline execution and event dispatching.
