# Task 116A: Bi-Directional Citation Canvas & Actionable RAG Workspace

## Objective
Implement bidirectional linkage between `PdfChatPanel.svelte` citations and the PDF canvas in `PdfViewer.svelte`.

## Requirements
1. **`PdfViewer.svelte`**:
   - Add listener for `paperpilot:jump-to-citation` event (`{ page: number, bbox?: { x, y, width, height } }`).
   - On receipt:
     - Set `pageNum = event.detail.page`.
     - Render glowing pulse box overlay in `.citation-highlights-layer` using `convertToViewportPoint`.
     - Auto-clear pulse after 2.5s.
2. **`PdfChatPanel.svelte`**:
   - Parse citation tokens in assistant messages (`[Page N]`, `[p. N]`, `[p. N:M]`).
   - Render interactive citation pill buttons with smooth hover and click handlers.
   - Dispatch `paperpilot:jump-to-citation`.
3. **Tests & Build**:
   - Ensure `npm run build` / `pnpm build` in `apps/desktop` compiles with 0 errors.
   - Add Vitest or Playwright verification.
