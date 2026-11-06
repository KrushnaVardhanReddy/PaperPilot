# Spec 156: Desktop UI Polish, Microcopy Clarity & Actionable Feedback

## 1. Executive Summary & Problem Statement
User feedback and external reviews highlighted that while the PaperPilot backend engine and 90-tool execution pipelines are robust, the **Desktop UI presentation and microcopy are currently underspecified, abrupt, and lacking actionable descriptions**:
1. **Cryptic Tool Descriptions**: Tools in `toolList.ts` and `OperationsPanel.svelte` currently display brief 3–5 word snippets (e.g., *"Divide document into separate pages"*, *"Recover and rebuild corrupted or unreadable documents"*, *"Extract tabular data into spreadsheets"*). Users cannot tell what inputs are accepted, what output structure is produced, or what specific flags/modes are applied.
2. **Empty States & Onboarding**: The initial hero drop-zone (`DropZone.svelte`) states *"Drop files here"* and *"PDF, Office, Markdown, HTML, JSON · or click to browse"*, but lacks clear visual affordances, explanations of air-gapped local processing, and supported file extension badges.
3. **Action Parameter Cards**: In `OperationsPanel.svelte` and `EditableActionCard.svelte`, form inputs lack contextual placeholder help, parameter units (e.g., degrees for rotation, DPI for rasterization, padding length for Bates stamping), and guidance on destructive operations (redaction, flattening, deletion).
4. **Error & Status Clarity**: While `errorFormatter.ts` handles basic classification, many toast notifications and status bar elements report raw error strings or generic headlines without step-by-step remediation suggestions.

Spec 156 refactors the frontend text, tool metadata, empty states, and feedback mechanisms across `apps/desktop` to deliver clear, professional, enterprise-grade UX copy and visual clarity.

---

## 2. Key Areas of Improvement

### 2.1 Rich, Descriptive Tool Metadata (`apps/desktop/src/lib/types/toolList.ts`)
Upgrade every tool definition with:
- **`detailedDescription`**: A 2-sentence explanation of what the tool accomplishes and how it behaves under the hood.
- **`inputHelp`**: Clear guidance on required inputs (e.g., *"Requires 2 or more PDFs"*, *"Single PDF with interactive AcroForm fields"*).
- **`outputHelp`**: Clear expectation of the resulting file (e.g., *"Produces a single linearized PDF optimized for instant streaming"*, *"Outputs an RFC 4180 compliant CSV file"*).
- **`securityNote`**: Clarifying privacy guarantees (e.g., *"100% offline pure-Rust transformation — zero network transmission"*).

### 2.2 Hero Empty State & Document Intake (`apps/desktop/src/lib/components/ui/DropZone.svelte`)
- Replace the bare *"Drop files here"* text with an informative, welcoming intake banner:
  - Header: **"Open or Drag & Drop Documents"**
  - Descriptive Subtitle: *"Fast, privacy-first document transformation. All processing occurs locally on your machine with zero cloud uploads."*
  - Supported format pills: `[📄 PDF]` `[📝 Word .docx]` `[📊 Excel .xlsx]` `[📑 Markdown]` `[🌐 HTML]`
  - Secondary action buttons with tooltips for URL ingestion and directory loading.

### 2.3 Operations Dock UX & Parameter Forms (`apps/desktop/src/lib/components/layout/OperationsPanel.svelte`)
- Display input/output format badges on every tool card.
- Add informative helper tooltips / subtitles underneath each form field in the operations dock:
  - *Angle*: "Rotate clockwise in 90° increments (90°, 180°, 270°)"
  - *Page Ranges*: "Specify discrete pages or intervals (e.g. 1-3, 5, 8-10)"
  - *Password*: "Used to encrypt output with 256-bit AES encryption"
  - *DPI / Scale*: "Higher resolution increases file size; standard printing is 300 DPI"
- Show an alert callout for irreversible operations like `pdf_redact` (*"Permanent excision: underlying text glyphs and vectors will be removed from the PDF stream"*).

### 2.4 User-Friendly Error Formatting (`apps/desktop/src/lib/utils/errorFormatter.ts`)
- Expand error headlines and action hints with concrete next steps:
  - Password errors: *"Protected PDF — Enter the document open password in the security panel to unlock editing."*
  - Missing fonts/render: *"Complex Font Encodings — This document uses non-standard embedded CID fonts; try running OCR or Flatten Annotations first."*

---

## 3. Deliverables & Scope
1. Update `apps/desktop/src/lib/types/toolList.ts` with descriptive copy and tooltips.
2. Polish `DropZone.svelte` with format badges, privacy assurances, and clear instructions.
3. Polish `OperationsPanel.svelte` with field-level helper hints and input/output expectations.
4. Enhance `errorFormatter.ts` with expanded guidance.
5. Verify Playwright test suite passes cleanly with updated DOM selectors and text assertions.
