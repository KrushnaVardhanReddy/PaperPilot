# Public roadmap

Task 131A exposes the project milestone tracker as a searchable Kanban modal in
the web portal. The header and footer open it, and `/roadmap` is a shareable URL
that survives refresh and browser back/forward navigation.

## Keep progress current

Edit task rows in `phases/phase-*.md`. From the repository root, run:

```sh
pnpm --dir apps/web sync-roadmap
pnpm --dir apps/web build
```

`prebuild` regenerates `apps/web/public/roadmap.json` automatically, including when
the website is built for deployment. `predev` also generates it before the local
development server starts. While that server is running, rerun `sync-roadmap`
after editing phase documents. No frontend task list needs to be maintained.

The snapshot is **build-time project metadata**. Publishing an updated build
publishes the updated roadmap; the browser does not poll GitHub PR status. The
existing download badge separately reads public GitHub release metadata with its
15-minute cache and offline fallback. Private document processing is unchanged.

## Data contract and status rules

The parser reads all `phases/phase-*.md` files, ignores fenced code examples,
and supports task tables, numbered text-only milestones, and task checklists.
An empty parse fails the build without overwriting the last valid snapshot.

- Explicit task notes take priority over stale status emoji in task titles.
- `Completed`, `Merged`, `✅`, `✓`, or strikethrough completion marks map to `shipped`.
- `In Progress`, `In-Flight`, scaffolding, and work awaiting review/integration map
  to `in_progress`. Locally implemented but unmerged work stays in this column.
- `Planned`, `Backlog`, `Not Started`, and otherwise unmarked tasks map to `planned`.
- An explicitly marked section supplies the default status for its child tasks.

Each item retains its original task ID and has a unique `source_path:source_line`
key. This preserves distinct tasks when legacy task numbers overlap. Phase
numbers come from the modular filenames, so legacy heading numbers in the Office
and Universal Document OS documents do not overwrite module identities.

The JSON contains its schema version, a deterministic source fingerprint, source
filenames, the tool count from `PHASES.md`, and task records. Records include
title, summary, category, status, phase, source line, and available PR/spec links.
Shipped cards with PR references are shown in descending PR-number order.
Text is rendered with Svelte's normal escaping, without injecting source HTML.

## Community requests

Planned cards open `.github/ISSUE_TEMPLATE/feature_request.yml` on GitHub. The URL
prefills `title`, `problem`, and `proposed_solution`, matching the existing issue
form field IDs. The user reviews and submits the request on GitHub; the portal
does not post issues or votes itself. Source and PR links also open in new tabs.

## Verify

```sh
pnpm --dir apps/web test-roadmap
pnpm --dir apps/web check-roadmap
pnpm --dir apps/web check
pnpm --dir apps/web build
pnpm --dir apps/web test-roadmap-ui
```

The browser suite uses the **production preview build** at `127.0.0.1:4187` and
actual generated roadmap data. Install Playwright Chromium before running it,
or set `PLAYWRIGHT_CHANNEL=chrome` to use an installed Chrome browser. Screenshots
and failure traces are written under `apps/web/test-results/roadmap/` (ignored).

The generated `_redirects` file provides `/roadmap` and `/roadmap/` SPA rewrites
for Cloudflare Pages and compatible hosts. Other static hosts must serve
`index.html` for those paths.
