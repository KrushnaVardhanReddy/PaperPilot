# Task 131A: Automated Roadmap Data Pipeline & Interactive Public Roadmap Board

## Objectives
Implement the complete dynamic public roadmap pipeline defined in `specs/131-dynamic-public-roadmap-ui/spec.md`.

## Subtasks
1. **Parser Script (`scripts/sync_roadmap.ts`)**:
   - Pure TypeScript using Node.js `fs` and `path`.
   - Parse all `phases/*.md` documents.
   - Extract phase number, task ID, title, status (`shipped`, `in_progress`, `planned`), description, and PR link.
   - Output structured JSON to `apps/web/public/roadmap.json`.
2. **Web UI Component (`apps/web/src/components/RoadmapModal.svelte`)**:
   - Modern glassmorphic 3-column Kanban interface.
   - Column 1: Shipped (with green badges and PR/release links).
   - Column 2: In Active Development (current active work).
   - Column 3: Planned / Up Next (with "Vote / Request" button linking to GitHub Feature Request template).
   - Category filter pills + live search input.
3. **App Header & Footer Integration**:
   - Add "Roadmap" button in `apps/web/src/App.svelte` header and footer.
   - Modal trigger state management in Svelte 5 runes (`let showRoadmap = $state(false)`).
4. **Build Automation**:
   - Update `apps/web/package.json` to run `pnpm tsx ../../scripts/sync_roadmap.ts` on build.
   - Verify `pnpm --dir apps/web build` passes with zero errors.

## Local Implementation / Review Handoff

Implemented on `feat/public-roadmap`, based on `origin/feature/dev` at `bac7682`.
The implementation is pending reviewed integration and deployment.

- [x] TypeScript parser, source references, deterministic snapshot and build hook.
- [x] Search/category filters, native dialog, responsive board and GitHub issue form links.
- [x] Header/footer links and direct `/roadmap` navigation.
- [x] Parser/data tests and production-browser tests.
- [ ] Reviewed integration into `feature/dev` and website deployment.

Usage: [PUBLIC_ROADMAP.md](../../docs/PUBLIC_ROADMAP.md).
Evidence: [validation report](../../reports/DYNAMIC_PUBLIC_ROADMAP_REPORT.md).
