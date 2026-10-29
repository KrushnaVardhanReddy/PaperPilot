# Task 3.4.15: Universal Document Workspace & Consumer-Friendly Studio Migration

## Objectives
Implement Spec 143 (`specs/143-desktop-universal-doc-workspace-migration/spec.md`) in `apps/desktop`, migrating the desktop UI into a modern, consumer-friendly Universal Document Workspace connected 100% to real app data and all 75 tools.

## Subtasks
- [ ] **Task 143.1: Universal Drop Zone & Empty State**
  - Create `apps/desktop/src/lib/components/workspace/UniversalHomeDashboard.svelte`.
  - Provide a friendly drop zone supporting PDF, Word, Excel, CSV, Markdown, and Images.
  - Implement the 8 Everyday Universal Action Cards linked to real file selection and `appState.activeTool`.
- [ ] **Task 143.2: Real Smart Format Filter Pills**
  - Update `apps/desktop/src/lib/components/ui/DocumentList.svelte` with reactive format filter pills (`All`, `PDF`, `Sheets`, `Word`, `Scanned`).
  - Calculate real document counts dynamically from `appState.documents`.
- [ ] **Task 143.3: Top Header Mode Switcher & Command Search**
  - Add slim top header with mode switcher (`[ 📄 Studio ] [ 🔗 Automations ]`).
  - Wire up global search input to `CommandPalette.svelte` / `GlobalCommandBar.svelte` with `Ctrl + K` hotkey.
- [ ] **Task 143.4: Contextual Active Workspace & Floating Pill**
  - Ensure `+page.svelte` cleanly switches between `PdfViewer.svelte` and `SpreadsheetViewer.svelte`.
  - Add a sleek floating action pill at the bottom center of the active viewer for quick common operations.
  - Add a collapsible toggle for the right Operations Panel.
- [ ] **Task 143.5: Tests & Zero-Mock Verification**
  - Ensure all 117 desktop tests pass via `pnpm --dir apps/desktop test`.
  - Verify that no static mock data or hardcoded arrays exist in the updated components.
