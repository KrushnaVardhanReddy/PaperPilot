# Spec 131: Dynamic Public Roadmap UI & Automated Milestone Synchronization

## Executive Summary
PaperPilot has completed 78+ verified tools across 5 interfaces, an on-device SLM, and a comprehensive security audit. To build community trust, drive GitHub stargazers toward 1,000+ stars, and adhere to our active Feature Freeze policy, PaperPilot requires a **live, visual public Roadmap UI** on `usepaperpilot.com`. 

Crucially, this roadmap must be **100% dynamic and automated**: whenever a feature or task is merged or updated in `phases/*.md`, it must automatically reflect in the UI without manual frontend edits.

---

## Architecture & Data Flow

```
   ┌────────────────────────────────────────┐
   │         phases/phase-*.md              │  (Single Source of Truth)
   └───────────────────┬────────────────────┘
                       │
                       ▼  (Invoked during build / pre-commit)
   ┌────────────────────────────────────────┐
   │        scripts/sync_roadmap.ts         │
   │  - Pure TypeScript / Node (0 Python)   │
   │  - Parses markdown tables              │
   │  - Extracts ID, Title, Status, PR link │
   │  - Buckets: shipped / in_progress /    │
   │             planned                    │
   └───────────────────┬────────────────────┘
                       │
                       ▼
   ┌────────────────────────────────────────┐
   │      apps/web/public/roadmap.json      │  (Static JSON Artifact)
   └───────────────────┬────────────────────┘
                       │
                       ▼
   ┌────────────────────────────────────────┐
   │  apps/web/src/components/              │
   │    RoadmapModal.svelte                 │
   │  - 3-Column Glassmorphic Kanban        │
   │  - Live search & category filters      │
   │  - "Vote / Request" GitHub Issue link  │
   └────────────────────────────────────────┘
```

---

## Detailed Requirements

### 1. Extraction Engine (`scripts/sync_roadmap.ts`)
- Pure TypeScript / Node.js script (Zero Python dependency).
- Traverses all modular phase documents under `phases/` (`phase-01-rust-core.md` through `phase-13-universal-document-os.md`).
- Parses markdown tables and status indicators:
  - `✅` / `Completed` / `Merged` -> `status: "shipped"`
  - `🚀` / `In Progress` -> `status: "in_progress"`
  - `📋` / `Planned` / `Backlog` -> `status: "planned"`
- Extracts metadata: `id`, `title`, `phase`, `phase_name`, `status`, `summary`, `pr_url`, `spec_path`.
- Emits formatted JSON to `apps/web/public/roadmap.json`.
- Integrates with `apps/web/package.json`: `"prebuild": "pnpm tsx ../../scripts/sync_roadmap.ts"`.

### 2. Frontend Roadmap Component (`RoadmapModal.svelte` / `RoadmapView`)
- Sleek, dark-mode glassmorphic styling consistent with PaperPilot's design tokens.
- **Top Metrics Bar**:
  - Total Tools: 78+
  - Total Downloads: Live count from GitHub API
  - Privacy: 100% Client-Side Air-Gapped
- **3-Column Board**:
  - **🟢 Shipped (Production)**: Shows task badge, description, and link to PR/spec.
  - **🟡 In Active Development**: Shows active milestone and link to in-flight tasks.
  - **🔵 Planned / Up Next**: Each planned item has a **"👍 Vote / Request on GitHub"** button that pre-populates a new GitHub issue via `.github/ISSUE_TEMPLATE/feature_request.yml`.
- **Search & Filter**:
  - Filter by Category (All, Core Rust Engine, AI & SLM, WebAssembly, Desktop, Security).
  - Quick text search across milestone names.

### 3. Navigation Integration
- Add **"🗺️ Roadmap"** navigation link in `apps/web/src/App.svelte` header actions (between Logo and Download CTA).
- Add "Roadmap" link in `app-footer`.

---

## Acceptance Criteria
1. `python3 scripts/sync_roadmap.py` parses all phase documents and generates valid `apps/web/public/roadmap.json`.
2. `RoadmapModal.svelte` renders all items accurately in their respective columns (`shipped`, `in_progress`, `planned`).
3. Clicking "Vote / Request" opens GitHub with pre-filled title and problem description matching the selected task.
4. `pnpm --dir apps/web build` succeeds 100% cleanly with zero TypeScript/Svelte warnings.
