# Task Checklist: Spec 083 — Embed Pipeline 1-to-1 Desktop Parity

## 1. Dropzone UX Polish (`apps/embed/src/Widget.svelte`)
- [ ] Add explicit `<button type="button" class="pp-btn-browse">📁 Browse Files</button>` inside the pipeline dropzone
- [ ] Add selected file card showing document icon, file name, formatted size, and `✕ Change File` button
- [ ] Ensure click handlers stop propagation cleanly so clicking "Browse Files" or "Clear" triggers the expected action

## 2. Interactive Stepper & Step Picker (`apps/embed/src/Widget.svelte`)
- [ ] Add mutable `$state` `customSteps` initialized from `parsedPipeline`
- [ ] Display step chips with remove button (`✕`) when customization is active
- [ ] Add `[+ Add Step]` button at the end of the stepper
- [ ] Implement inline tool picker modal/drawer with available in-browser operations:
  - `rotate` (default angle 90)
  - `watermark` (default text 'CONFIDENTIAL', opacity 0.3)
  - `compress`
  - `extract` (default pages '1')
  - `split` (default pages '1-2')
- [ ] Appending a tool updates `customSteps` and refreshes the visual stepper

## 3. Flow Management Toolbar (Save, Presets, Export, Import)
- [ ] Implement Flow Toolbar inside `Widget.svelte`:
  - [ ] `<select>` dropdown grouping `<optgroup label="Standard Presets">` and `<optgroup label="My Saved Flows">`
  - [ ] `💾 Save Flow` button opening a modal to name & save the recipe to `localStorage['paperpilot_saved_flows']`
  - [ ] `⬇️ Export JSON` button generating and downloading `<name>.json`
  - [ ] `⬆️ Import JSON` file input allowing users to upload a `flow.json` and load its steps
- [ ] Dispatch custom DOM events (`paperpilot:flow-saved`, `paperpilot:flow-exported`) for parent host app integration

## 4. Execution Controls & Props
- [ ] Add `allowCustomization` prop (mapped from `data-allow-customization="true"`)
- [ ] Add `autoExecute` prop (defaults to true for fixed pipelines, false when customizable)
- [ ] Add explicit `<button class="pp-btn pp-btn-primary pp-btn-run">⚡ Run Pipeline</button>` when `autoExecute === false`
- [ ] Progress bar and result cards with download link upon completion

## 5. Demo & Automated Playwright Tests
- [ ] Update `apps/web/public/embed-test.html` Demo 3 showing interactive customizable mode with flow toolbar
- [ ] Add Playwright test in `apps/embed/tests/pipeline.spec.ts` asserting:
  - [ ] `"Browse Files"` button is visible and clickable
  - [ ] Flow selector dropdown switches between presets
  - [ ] Adding and deleting steps dynamically
  - [ ] Saving a custom flow to `localStorage`
  - [ ] Running the customized pipeline end-to-end
- [ ] Build verification: `npm --prefix apps/embed run build` passes with 0 errors
