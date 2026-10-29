# Task 4.7.8: Desktop Real Watched Folders Daemon & Document Smart Filter Tabs

## Objectives
Implement Spec 142 (`specs/142-desktop-watched-folders-and-smart-filters/spec.md`) connecting the backend hot folder watcher daemon to the Tauri desktop GUI and adding reactive document smart filter pills.

## Subtasks
- [x] **Task 142.1: Tauri IPC Watch Commands**
  - Create `apps/desktop/src-tauri/src/watch_commands.rs`.
  - Implement `start_watch_folder`, `stop_watch_folder`, and `list_watched_folders`.
  - Register commands in `apps/desktop/src-tauri/src/lib.rs`.
- [x] **Task 142.2: Svelte State Management**
  - Create `apps/desktop/src/lib/state/watchers.svelte.ts` tracking active folder watchers with local persistence.
- [x] **Task 142.3: Watched Folders UI**
  - Create `apps/desktop/src/lib/components/layout/WatchedFoldersPanel.svelte` (or integrate into `SettingsPanel.svelte`).
  - Allow browsing folder, choosing workflow recipe, and starting/stopping watchers with status indicator.
- [x] **Task 142.4: Real Smart Filters in DocumentList.svelte**
  - Add reactive filter pill strip (`All`, `Scanned`, `Signed`, `Forms`, `Multi-Page`) in `apps/desktop/src/lib/components/ui/DocumentList.svelte`.
  - Calculate real document counts dynamically and filter the displayed document items.
- [x] **Task 142.5: Tests & Verification**
  - Add unit tests verifying filter computations and watcher state lifecycle.
  - Run `pnpm --dir apps/desktop test` to ensure zero regressions.

Implemented on `feat/desktop-watched-folders-and-smart-filters-task-4-7-8`; awaiting integration review. See `reports/DESKTOP_AUTOMATION_REPORT.md` for real-PDF evidence and verification limits. Running state is never restored from preferences, and no daemon is auto-started after relaunch.
