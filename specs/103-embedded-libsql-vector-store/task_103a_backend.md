# Task 103A: Embedded libSQL Core Storage Engine & Schema Migrations

## Status
- Spec: [Spec 103](file:///home/krushna/Project/PaperPilot/specs/103-embedded-libsql-vector-store/spec.md)
- Subsystem: `paperpilot-storage`
- Target: Wave 7 / Enterprise Foundation
- Status: ⏳ Ready for Jules Submission

---

## Objectives & Work Items

### 1. Workspace Crate Setup
- [ ] Create `paperpilot-storage/Cargo.toml` with `libsql = "0.10.0"`, `tokio`, `serde`, `serde_json`, `thiserror`, `uuid`, `chrono`.
- [ ] Add `"paperpilot-storage"` to root `Cargo.toml` members.

### 2. Core Storage Engine & Schema Migrations (`paperpilot-storage/src/`)
- [ ] Define error types in `paperpilot-storage/src/error.rs`.
- [ ] Implement `StorageEngine` in `paperpilot-storage/src/lib.rs` supporting `open_local(path)` using `libsql::Builder::new_local(path).build().await`.
- [ ] Implement embedded SQL migrations:
  - `jobs` table (id, operation, status, input_path, output_path, parameters_json, progress_percentage, callback_url, created_at, updated_at).
  - `webhooks` table (id, name, target_url, secret_token, events_json, is_active, retry_count, created_at).
  - `document_chunks` table (id, document_hash, document_name, page_number, chunk_index, content, embedding_json, created_at).
  - `audit_logs` table (id, event_type, tool_id, actor, payload_hash, details_json, timestamp).
- [ ] Implement asynchronous Job CRUD operations:
  - `create_job(&self, job: &NewJob) -> Result<JobRecord, StorageError>`
  - `update_job_status(&self, id: &str, status: JobStatus, progress: u8, output: Option<&str>, error: Option<&str>) -> Result<(), StorageError>`
  - `get_job(&self, id: &str) -> Result<Option<JobRecord>, StorageError>`
  - `list_jobs(&self, limit: usize) -> Result<Vec<JobRecord>, StorageError>`
- [ ] Implement Audit Logging:
  - `record_audit_event(&self, event: &AuditEvent) -> Result<(), StorageError>`

### 3. Unit Tests & Conformance
- [ ] Write unmocked integration tests using `tempfile::tempdir()`:
  - `test_storage_engine_migration_and_job_lifecycle`: Creates a local DB, runs migrations, inserts a job, updates progress to 100%, fetches and asserts state.
  - `test_storage_engine_audit_log`: Records an event, queries and verifies persistence.
- [ ] Run `cargo test -p paperpilot-storage -j 2`
- [ ] Run `cargo check --workspace -j 2`
