# EMBED WIDGET REPORT

## Architecture Summary
The Embedded Web Distribution layer (`apps/embed`) enables dropping the PaperPilot WASM engine directly into any website with a single `<script>` tag and a `div` element.

It comprises:
1.  **Universal CDN Loader (`embed.js`)**: A lightweight script built via Rollup (using Vite with Svelte). It automatically searches for `[data-paperpilot-portal]` or `#paperpilot-portal` nodes. Upon finding them, it parses configuration from `data-*` attributes.
2.  **Shadow DOM Containerisation**: To ensure styles do not bleed from or into host pages (like WordPress sites with strong global CSS resets), the widget is appended to an open Shadow Root on the host element.
3.  **Svelte 5 Widget (`Widget.svelte`)**: Designed using Svelte 5 runes (`$props`, `$derived`). It takes arguments to display a drag-and-drop zone and selected tool cards, firing out custom DOM events (`paperpilot:ready`, `paperpilot:process-start`).
4.  **JavaScript Programmatic API**: `window.PaperPilot` exposes `.mount()`, `.unmount()`, and `.on()` for developers working with SPA frameworks to inject and coordinate widget lifecycles independently of automatic parsing.

## Build Status
- [x] `npm run build` exits with code 0 with no TypeScript errors or warnings.
- [x] `dist/embed.js` raw size: **37 KB**, gzip size: **14.3 KB**.
- [x] `npx playwright test` — all 5 tests green:
  1. 1. Shadow DOM Encapsulation
  2. 2. Tool Filtering
  3. 3. Badge Visibility (on by default)
  4. 4. Badge Hidden
  5. 5. Programmatic Mount/Unmount
- [x] `window.PaperPilot.mount()` / `unmount()` / `on()` are accessible on the global object in a plain HTML page.
- [x] Shadow DOM confirmed: host page style `div { color: red !important; }` does NOT leak into widget internals.
- [x] `data-hide-badge="true"` correctly removes the `⚡ Powered by PaperPilot` badge.