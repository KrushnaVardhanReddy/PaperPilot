# Task 103C: Desktop UI Async Job Drawer & Cross-Document Semantic Search Integration

## Metadata
- **Spec**: [Spec 103: Embedded libSQL / Turso Local Vector Store & Persistent Engine](file:///home/krushna/Project/PaperPilot/specs/103-embedded-libsql-vector-store/spec.md)
- **Subsystem**: `apps/desktop` (`src-tauri` and Svelte frontend)
- **Primary Crate**: `desktop` (`apps/desktop/src-tauri`) & `apps/desktop/src/lib/components/`
- **Dependencies**: `paperpilot-storage = { path = "../../../paperpilot-storage" }`
- **Zero Conflict Guarantee**: Completely isolated to `apps/desktop`. Does not touch CLI, MCP, or `paperpilot-pdf`.

---

## Detailed Requirements

### 1. Tauri Backend Integration (`apps/desktop/src-tauri/`)

1. **`Cargo.toml`**:
   - Add `paperpilot-storage = { path = "../../../paperpilot-storage" }` under `[dependencies]`.
2. **Database Initialization in `src-tauri/src/lib.rs` / `storage.rs`**:
   - Create a global `STORAGE_ENGINE: Arc<tokio::sync::Mutex<Option<paperpilot_storage::StorageEngine>>>` or initialize in Tauri app state:
     - Target database path: `crate::storage::get_app_data_root().join("paperpilot.db")`.
     - Ensure parent directory exists.
     - Call `StorageEngine::open_local(&db_path).await` and `engine.run_migrations().await`.
3. **Expose Tauri IPC Commands**:
   - `get_jobs_history(limit: Option<usize>) -> Result<Vec<paperpilot_storage::models::JobRecord>, String>`:
     - Returns list of recent async jobs ordered by `created_at DESC`.
   - `get_audit_events(limit: Option<usize>) -> Result<Vec<paperpilot_storage::models::AuditRecord>, String>`:
     - Returns list of immutable audit compliance records.
   - `search_document_vectors(query_embedding: Vec<f32>, limit: Option<usize>) -> Result<Vec<paperpilot_storage::models::SemanticSearchResult>, String>`:
     - Calls `engine.search_chunks(&query_embedding, limit.unwrap_or(10)).await`.
   - `store_document_chunks(chunks: Vec<paperpilot_storage::models::NewDocumentChunk>) -> Result<(), String>`:
     - Persists vector chunks for a document.

---

### 2. Desktop Frontend UI Components (`apps/desktop/src/lib/components/`)

1. **`<JobQueueDrawer.svelte>` (`apps/desktop/src/lib/components/pipeline/JobQueueDrawer.svelte`)**:
   - Slide-out side drawer or bottom tray to monitor background async operations (OCR, PDF/A, large batch compression, vector indexing).
   - Display: Job ID, Operation badge, Status pill (`pending`, `running`, `completed`, `failed`), progress percentage bar, timestamp, and error message if failed.
   - Actions: Refresh button, Clear/Retry trigger.
2. **Semantic Search Toggle in `<WorkspaceSearchModal.svelte>`**:
   - Add a toggle switch / button: `[Exact Text | 🧠 Semantic Vector]`.
   - When Semantic Vector is active, displays vector cosine similarity score (e.g. `94% match`) next to chunk snippets.
3. **Global Header / Status Bar Integration**:
   - Add a small Job Queue icon badge in `<GlobalCommandBar.svelte>` or toolbar indicating active background tasks.

---

### 3. Verification & Acceptance Criteria
1. `cargo check -p desktop -j 2` compiles with zero errors.
2. `pnpm --dir apps/desktop run check` passes Svelte type-checking.
3. Unit / integration tests for storage commands pass.
