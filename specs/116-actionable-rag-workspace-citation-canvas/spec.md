# Spec 116: Actionable RAG Workspace & Bi-Directional Citation Canvas (Task 4.3.9)

## 1. Overview
Bridges document question-answering with the interactive PDF canvas viewer. When users chat with their document in `PdfChatPanel.svelte`, responses contain actionable citation pills (e.g. `[Page 3]` or `[Page 4: Section 2]`). Clicking a citation pill automatically:
1. Navigates the canvas to the cited page (`pageNum`).
2. Computes the target bounding box (or page center if bbox not provided).
3. Renders a temporary glowing highlight pulse (`.citation-pulse`) around the cited area on the canvas.
4. Allows chat prompts to propose actionable execution plans directly from retrieved context.

---

## 2. Technical Architecture

### 2.1 Citation Payload Structure
```typescript
export interface DocumentCitation {
  page: number; // 1-indexed page number
  snippet: string; // Quoted excerpt
  bbox?: { x: number; y: number; width: number; height: number }; // Optional PDF coordinate box
}
```

### 2.2 Event Contract
- **Event Name**: `paperpilot:jump-to-citation`
- **Dispatched By**: `PdfChatPanel.svelte` (or `CitationBadge.svelte`)
- **Payload (`CustomEventDetail`)**:
  ```typescript
  {
    page: number;
    bbox?: { x: number; y: number; width: number; height: number };
    label?: string;
  }
  ```
- **Handled By**: `PdfViewer.svelte`
  - Sets `pageNum = event.detail.page`.
  - If `bbox` provided, adds a transient item to `citationHighlights` with a CSS keyframe animation pulse (duration ~2.5s).

### 2.3 Citation Rendering in Chat Stream
- Markdown parser or regex inside `PdfChatPanel.svelte` identifies `[Page X]` or `[Page X:Y]` patterns and replaces them with interactive `<button class="citation-pill">`.
- Clicking the button fires `window.dispatchEvent(new CustomEvent('paperpilot:jump-to-citation', ...))`.

---

## 3. Visual & Aesthetic Standards
- Dark mode glassmorphism: `#181825` container with `#4f46e5` / `#6366f1` accent.
- Citation pills: Subtle blue/violet rounded badge with a small document icon (`📄 p. 4`).
- Highlight Pulse: Semi-transparent neon highlight (`rgba(99, 102, 241, 0.35)`) with an expanding pulsing border (`@keyframes pulse-ring`) that fades out after 2.5 seconds.
