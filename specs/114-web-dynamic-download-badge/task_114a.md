# Task 114A: Implement Dynamic GitHub Releases Download Counter Badge in Web Portal

## Metadata
- **Spec**: [Spec 114: Web Portal Dynamic GitHub Releases Download Counter Badge](file:///home/krushna/Project/PaperPilot/specs/114-web-dynamic-download-badge/spec.md)
- **Phase**: Phase 5.9 / Task 5.9.83
- **Primary Language**: Svelte 5 / TypeScript / CSS
- **Impacted Area**: `apps/web/`

---

## Detailed Requirements

### 1. Dynamic Badge Component (`apps/web/src/components/DownloadBadge.svelte`)
- Uses Svelte 5 runes (`$state`, `$props`).
- Fetches all releases from GitHub Releases API asynchronously on mount.
- Aggregates `download_count` across all release assets.
- Implements `sessionStorage` caching (15-minute TTL) to preserve rate-limit budget.
- Pill badge styling adhering to PaperPilot dark theme and micro-animation standards.

### 2. Portal Header & Footer Mounting (`apps/web/src/App.svelte`)
- Mount `<DownloadBadge />` in `.header-actions` next to `<ShareModal />` and download CTA button.
- Clean responsive layout for desktop, tablet, and mobile.

### 3. Verification
- Run `npm run build` in `apps/web` to guarantee zero bundling or compiler warnings.
