# Task 103C: Desktop UI Async Job Drawer & Cross-Document Semantic Search Dock

## Status
- Spec: [Spec 103](file:///home/krushna/Project/PaperPilot/specs/103-embedded-libsql-vector-store/spec.md)
- Subsystem: `apps/desktop`
- Target: Wave 7 / Desktop UX & Enterprise Integration
- Status: 📋 Scoped

---

## Objectives & Work Items

### 1. Tauri Backend Commands (`apps/desktop/src-tauri/src/lib.rs`)
- [ ] Initialize `StorageEngine` in `get_app_data_root()` during desktop app startup.
- [ ] Expose Tauri IPC commands:
  - `get_jobs_history()` $\to$ returns list of recent `JobRecord` items.
  - `search_document_vectors(query: String)` $\to$ returns semantic chunk matches.
  - `get_audit_events(limit: usize)` $\to$ returns compliance log entries.

### 2. Frontend UI Components (`apps/desktop/src/lib/components/`)
- [ ] Create `<JobQueueDrawer />` in Desktop UI to view background jobs, progress percentages, and retry actions.
- [ ] Add semantic search toggle to Workspace Search (`multi-format-workspace-search`) allowing semantic vector matching alongside regex text search.
- [ ] Run Vitest unit tests: `pnpm --dir apps/desktop test`
