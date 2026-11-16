# Task: Spec 150 - Hexagonal Storage Port Architecture

- Spec: [Spec 150](spec.md)
- Subsystems: `paperpilot-core`, `paperpilot-storage`, `apps/desktop/src-tauri`

## Tasks

- [ ] **Task 1: Define `DatabasePort` & Domain Models in `paperpilot-core`**
  - Add `pub trait DatabasePort: Send + Sync` in `paperpilot-core/src/storage.rs` (or `paperpilot-core/src/ports.rs`).
  - Move domain structs: `JobRecord`, `NewJob`, `JobStatus`, `AuditEvent`, `DocumentChunkRecord`, `VectorSearchResult`, `StorageError` into `paperpilot-core::storage`.
  - Re-export in `paperpilot-core::ports`.

- [ ] **Task 2: Implement `DatabasePort` for `StorageEngine` in `paperpilot-storage`**
  - Update `paperpilot-storage/src/lib.rs` to implement `DatabasePort for StorageEngine`.
  - Ensure all methods (`create_job`, `get_job`, `update_job_status`, `list_jobs`, `store_document_chunks`, `search_similar_chunks`, `delete_document_chunks`, `append_audit_log`, `list_audit_logs`) satisfy the trait.
  - Re-export the domain types for backwards compatibility.

- [ ] **Task 3: Update Callers & Desktop Integration**
  - In `apps/desktop/src-tauri/src/storage.rs`, use `Arc<dyn DatabasePort>` or generic bounds.
  - Verify that `GLOBAL_STORAGE` and helper functions work transparently.

- [ ] **Task 4: Unit Testing & Workspace Compilation**
  - Run `cargo test -p paperpilot-storage -j 4`.
  - Run `cargo check --workspace -j 4`.
  - Ensure all existing tests pass with zero regressions.
