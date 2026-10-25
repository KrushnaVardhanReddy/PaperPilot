# Task 113A: Implement Web Portal Mobile Handoff QR Code & Share Modal

## Metadata
- **Spec**: [Spec 113: Web Portal Instant Mobile Handoff & Share Modal](file:///home/krushna/Project/PaperPilot/specs/113-web-share-qr-modal/spec.md)
- **Phase**: Phase 5.9 / Task 5.9.82
- **Primary Language**: Svelte 5 / TypeScript / CSS
- **Impacted Area**: `apps/web/`

---

## Detailed Requirements

### 1. Verified Static QR Code Asset (`apps/web/public/qr_code.svg`)
- Zero-external-dependency, standard crisp SVG QR code for mobile handoff.

### 2. Svelte 5 Share Modal Component (`apps/web/src/components/ShareModal.svelte`)
- State management using Svelte 5 runes (`$state`).
- QR code preview rendered centered with dark-mode contrast padding.
- Input box displaying current link + "Copy" button.
- "Share via..." button for `navigator.share` (conditionally shown if API exists).
- Esc key listener and outside click dismiss handler.

### 3. Header Integration (`apps/web/src/App.svelte`)
- Place Share button inside `.header-content` in top navigation.
- Maintain responsive layout on mobile/tablet viewports.

### 4. Verification
- `npm run check` or `npm run build` inside `apps/web/` to confirm zero TypeScript or bundling errors.
