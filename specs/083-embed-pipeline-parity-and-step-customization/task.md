# Task Checklist: Spec 083 — Embed Pipeline Parity (Browse Button & Interactive Step Selection)

## 1. Dropzone UX Polish (`apps/embed/src/Widget.svelte`)
- [ ] Add explicit, styled `<button type="button" class="pp-btn-browse">Browse Files</button>` inside the pipeline dropzone
- [ ] Add selected file summary chip displaying file name and size once uploaded
- [ ] Add "Change File / Clear" button to swap files before execution
- [ ] Ensure clicking "Browse Files" triggers native file dialog without event bubbling conflicts

## 2. Interactive Pipeline & Step Selection (`apps/embed/src/Widget.svelte` & `apps/embed/src/api.ts`)
- [ ] Add `allowCustomization` prop and `data-allow-customization` dataset parser
- [ ] Add `autoExecute` prop (defaults to true for fixed pipelines, false when customizable)
- [ ] Initialize mutable `$state` `pipelineSteps` from `parsedPipeline`
- [ ] Render step chips with delete button (`×`) when customization is enabled
- [ ] Add `[+ Add Step]` button at the end of the stepper
- [ ] Add compact popover tool picker displaying available tools (`Rotate`, `Watermark`, `Compress`, `Extract`, `Split`, `Merge`)
- [ ] Add explicit `<button class="pp-btn pp-btn-primary pp-btn-execute">⚡ Run Pipeline</button>` when `autoExecute === false`

## 3. Demo & Testing Verification
- [ ] Update `apps/web/public/embed-test.html` Demo 3 with `data-allow-customization="true"` demonstration
- [ ] Add Playwright test in `apps/embed/tests/pipeline.spec.ts` asserting:
  - [ ] Visible "Browse Files" button in dropzone
  - [ ] Adding a new step dynamically via `+ Add Step`
  - [ ] Removing a step
  - [ ] Executing customized pipeline
- [ ] Run `npm --prefix apps/embed run build` and ensure zero errors
