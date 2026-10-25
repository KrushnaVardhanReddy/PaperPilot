# Spec 114: Web Portal Dynamic GitHub Releases Download Counter Badge

## Status: APPROVED
**Target Phase**: Phase 5.9 (Task 5.9.83)  
**Components**: `apps/web/` (`DownloadBadge.svelte`, `App.svelte`)  

---

## 1. Problem Statement
Users navigating the PaperPilot Web portal have no immediate visibility into community adoption and total desktop/CLI binary downloads. GitHub Releases hosts multi-platform assets (`macos`, `ubuntu`, `windows`, `android`), but users shouldn't have to navigate away to GitHub or manually tally download metrics.

---

## 2. Goals & Success Criteria
1. **Live Download Aggregation**: Fetch total releases metadata dynamically from the GitHub REST API (`https://api.github.com/repos/KrushnaVardhanReddy/PaperPilot/releases`) in real time.
2. **Aggregated Asset Downloads**: Accurately sum `download_count` across all release tags and binary assets.
3. **Session Cache**: Prevent GitHub API rate limits (60 req/hr unauthenticated) by caching results in `sessionStorage` for 15 minutes.
4. **Graceful Fallbacks**: Display latest version badge or loading skeleton if offline or rate-limited.
5. **Seamless UI Integration**: Mount sleek, responsive badge into the header actions and footer beside the privacy indicators.

---

## 3. Architecture & Data Flow
```
User visits Web Portal -> DownloadBadge.svelte
   |
   +--> Check sessionStorage (`gh_dls_KrushnaVardhanReddy/PaperPilot`)
   |       |
   |       +-- (Valid < 15m) -> Render cached count immediately
   |       +-- (Expired/None) -> Fetch https://api.github.com/repos/.../releases
   |                                 |
   |                                 +-- Sum all asset `download_count`s
   |                                 +-- Store in sessionStorage
   |                                 +-- Render formatted total pill (e.g., "0", "1,420")
```

---

## 4. Verification & Testing
- `npm run build` in `apps/web` must succeed with zero TypeScript or Svelte errors.
- Visual inspection: badge renders properly with pill count, links to GitHub Releases, and has hover micro-animations.
