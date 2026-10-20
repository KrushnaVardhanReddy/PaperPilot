# Task 5.9.67: Embeddable JS Client-Side Automation Pipeline & Headless Pipeline API

## Overview
Implement the client-side automation pipeline in `apps/embed` (Spec 078). This enables web SaaS platforms and embedding developers to run declarative, multi-step document workflows (e.g. `rotate ➔ compress ➔ watermark ➔ extract`) entirely in the user's browser with 0 server costs, via both a visual stepper widget and a headless JavaScript API (`PaperPilot.runPipeline`).

## Subtasks
- [ ] In `apps/embed/src/api.ts`:
  - Define `PipelineStep`, `PipelineOptions`, and `PipelineResult` interfaces.
  - Implement `runPipeline(options: PipelineOptions): Promise<PipelineResult>`.
  - Sequential execution: pass resulting Uint8Array from step N into step N+1.
- [ ] In `apps/embed/src/embed.ts`:
  - Expose `PaperPilot.runPipeline = runPipeline` on `window.PaperPilot`.
  - Parse `data-pipeline` attribute (JSON string) and `data-auto-download` in `autoInit()`.
- [ ] In `apps/embed/src/Widget.svelte`:
  - Add `pipeline?: string | PipelineStep[]` prop.
  - Render a visual multi-step pipeline stepper (`Step 1 ➔ Step 2 ➔ Step 3`) with live progress and status icons.
  - Dispatch custom event `paperpilot:pipeline-complete`.
- [ ] In `apps/embed/tests/pipeline.spec.ts`:
  - Add unit/E2E test verifying headless `runPipeline` execution and visual pipeline rendering.
- [ ] Build verification: `npm run build` in `apps/embed` passes with 0 errors.
