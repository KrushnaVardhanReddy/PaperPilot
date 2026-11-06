# Task 3.4.17: Interactive Visual Signature Creation & Placement System

## Parent Spec
`specs/152-interactive-signature-stamp-modal/spec.md`

## Objectives
Implement an interactive, industry-standard visual signature and initial placement system in `apps/desktop`. When the user clicks the floating `✍ Sign / Annotate` button, provide a signature creator modal (Type / Draw / Upload) and intuitive click-to-place stamping onto the PDF viewer canvas.

## Subtasks

- [ ] **Task 152.1: Signature Creation Dialog (`SignatureModal.svelte`)**
  - Create `apps/desktop/src/lib/components/signature/SignatureModal.svelte`.
  - Include tabs:
    1. **Type**: Input text with font styling presets (cursive handwriting scripts) and color choices (Black, Blue, Navy).
    2. **Draw**: Freehand signature canvas pad with Clear and Undo controls.
    3. **Upload**: File input for image files with automatic white background removal.
  - Support full name and initials modes.
  - Store recent signatures in `localStorage` under `paperpilot:saved_signatures`.

- [ ] **Task 152.2: Interactive Stamp & Placement Layer (`SignatureStampLayer.svelte`)**
  - Mount stamp overlay in `PdfViewer.svelte`.
  - When in placement mode, display a semi-transparent ghost preview following the cursor.
  - On mouse click, place signature as an editable stamp object with bounding box handles (move, resize, delete).
  - Map viewport screen pixels to document 72-dpi PDF point coordinate system accurately across zoom levels.

- [ ] **Task 152.3: Wire Floating Action Pill & Page Integration**
  - Update `apps/desktop/src/routes/+page.svelte` so clicking `✍ Sign / Annotate` triggers `SignatureModal` if no signature is armed, or enters stamp placement mode if one is ready.
  - Provide a quick popover menu on the pill: `Add Signature`, `Add Initials`, `Freehand Pen`, `Clear Annotations`.

- [ ] **Task 152.4: Baking / Persistence with Engine (`pdf_stamp`)**
  - Add an "Apply / Save Signed PDF" action button.
  - Convert signature SVG/PNG raster to temp image and invoke `pdf_stamp` via the engine to bake the signature cleanly into the output document.

- [ ] **Task 152.5: Tests & E2E Validation**
  - Add component unit test `SignatureModal.test.ts`.
  - Add Playwright E2E test verifying modal popup, signature creation, and page stamp placement.
