# Task 166: Implement "Try with Sample PDF" on Web Studio

## Checklist
- [x] Create `apps/web/public/samples/` directory
- [x] Add lightweight sample PDFs (`sample_document.pdf`, `sample_form.pdf`)
- [x] Update `.gitignore` to allow `!apps/web/public/samples/*.pdf`
- [x] Enhance `apps/web/src/components/DropZone.svelte` with `loadSamplePdf()` handler, `✨ Try with Sample PDF` button, and `👁️ View Sample` direct inspection link
- [x] Generate rich 3-page `sample_document.pdf` with cover overview, tabular invoice data, and mock PII records for end-to-end tool testing
- [x] Add styling for `.actions-row`, `.sample-btn`, `.view-sample-link`, and `.or-divider` with responsive layout
- [x] Test `pnpm --dir apps/web build` and verify output assets in `apps/web/dist/`
- [x] Refresh knowledge graph with `graphify-rs build --update`
- [x] Commit and push changes to `feat/adoption-and-ux-polish`
