# Tasks: Spec 156 — Desktop UI Polish, Microcopy Clarity & Actionable Feedback

- [ ] **156.1 — Descriptive Tool Metadata & Explanations**
  - [ ] 156.1.1 Enrich `ToolDefinition` interface in `apps/desktop/src/lib/types/tools.ts` with optional `detailedDescription`, `inputHelp`, `outputHelp`, and `securityNote`.
  - [ ] 156.1.2 Rewrite all 125 tool entries in `apps/desktop/src/lib/types/toolList.ts` replacing brief fragments with clear, descriptive sentences explaining exactly what inputs are expected and what output file is created.

- [ ] **156.2 — Hero Empty State & Document Intake UX**
  - [ ] 156.2.1 Redesign `DropZone.svelte` empty state: add explicit header ("Open or Drag & Drop Documents"), privacy statement ("100% Offline & Private: Documents never leave your device"), and interactive format indicator pills (`.pdf`, `.docx`, `.xlsx`, `.md`, `.html`, `.json`, `.csv`).
  - [ ] 156.2.2 Add tooltips to "Browse Files" and "Import from URL" explaining desktop native picker vs remote HTTP fetch.

- [ ] **156.3 — Operations Dock Parameter Guidance & Warnings**
  - [ ] 156.3.1 In `OperationsPanel.svelte`, add helper captions under common inputs (Page Ranges: "e.g. 1-3, 5, 8-10", Rotation: "Clockwise 90°, 180°, 270°", Password: "256-bit AES protection").
  - [ ] 156.3.2 Add warning banners on destructive operations (`pdf_redact`: "Permanent redaction cannot be undone", `pdf_delete_pages`: "Selected pages will be permanently excised").
  - [ ] 156.3.3 Display input & output format pills prominently on the selected tool header.

- [ ] **156.4 — Actionable Error Formatting & Feedback**
  - [ ] 156.4.1 Expand `errorFormatter.ts` with actionable guidance for password, encryption, font encoding, page bounds, and permission failures.
  - [ ] 156.4.2 Ensure `errorFormatter.test.ts` passes with full coverage.

- [ ] **156.5 — Playwright Verification**
  - [ ] 156.5.1 Run `make test-e2e-all-90` to guarantee zero test regressions across all 90 tools.
