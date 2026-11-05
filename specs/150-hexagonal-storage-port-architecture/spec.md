# Spec 150: Hexagonal Storage Port Architecture (`DatabasePort`)

## 1. Background & Executive Summary
Currently, `paperpilot-storage` is a standalone workspace crate in PaperPilot that embeds **Turso / libSQL** (`libsql = "0.10.0"`). It provides persistence for asynchronous jobs, outbound webhooks, audit compliance logs, and vector embeddings for semantic document search.

However, `StorageEngine` is currently a concrete struct hardwired directly to `libsql::Connection`:
```rust
pub struct StorageEngine {
    db: Arc<libsql::Database>,
    conn: Arc<libsql::Connection>,
}
```

This concrete coupling prevents:
1. **Database Swappability**: Downstream enterprise editions cannot swap libSQL for PostgreSQL, Supabase, or MySQL without modifying upstream code.
2. **Cleanroom Hexagonal Architecture**: Upstream `paperpilot-core` already defines an object/file `StoragePort` (for raw byte streams), but lacks a domain **Database Port** for state persistence and vector storage.

**Spec 150** decouples the storage layer using Hexagonal Architecture (Ports & Adapters).

---

## 2. Architectural Blueprint

```
                      ┌────────────────────────────────────────┐
                      │            paperpilot-core             │
                      │ ────────────────────────────────────── │
                      │  trait DatabasePort (Async Hexagonal)  │
                      │  JobRecord, AuditEvent, VectorChunk    │
                      └───────────────────▲────────────────────┘
                                          │
                   ┌──────────────────────┴──────────────────────┐
                   │                                             │
      ┌────────────┴─────────────┐                 ┌─────────────┴────────────┐
      │    paperpilot-storage    │                 │   PaperPilot-Enterprise  │
      │ ──────────────────────── │                 │ ──────────────────────── │
      │  impl DatabasePort for   │                 │  impl DatabasePort for   │
      │      StorageEngine       │                 │  PostgresStorageEngine   │
      │   (Embedded libSQL)      │                 │  (Postgres / Supabase)   │
      └──────────────────────────┘                 └──────────────────────────┘
```

### 2.1 The Port Contract (`paperpilot-core::storage`)
Define `pub trait DatabasePort: Send + Sync` in `paperpilot-core/src/ports.rs` (or `paperpilot-core/src/storage.rs`), covering:
- **Jobs CRUD**: `create_job`, `get_job`, `update_job_status`, `list_jobs`
- **Webhooks CRUD**: `register_webhook`, `get_webhook`, `list_webhooks`, `record_delivery`
- **Audit Logs**: `append_audit_log`, `list_audit_logs`
- **Vector Chunks & Search**: `store_document_chunks`, `search_similar_chunks`, `delete_document_chunks`

### 2.2 Domain Models in `paperpilot-core`
Move or re-export the storage domain models in `paperpilot-core`:
- `JobStatus` (`Pending`, `Running`, `Completed`, `Failed`)
- `JobRecord` & `NewJob`
- `AuditEvent`
- `DocumentChunkRecord` & `VectorSearchResult`
- `StorageError`

### 2.3 The Adapter (`paperpilot-storage`)
- `StorageEngine` in `paperpilot-storage` implements `DatabasePort`.
- Retains full support for `open_local(path)` and `open_in_memory()`.
- Zero changes to existing SQLite/libSQL table schemas (`jobs`, `webhooks`, `document_chunks`, `audit_logs`).

### 2.4 Downstream Compatibility
- `apps/desktop/src-tauri/src/storage.rs`: Calls `Arc<dyn DatabasePort>`.
- `PaperPilot-Enterprise`: Re-exports and implements `DatabasePort` for PostgreSQL/Supabase.

---

## 3. Acceptance Criteria
1. `paperpilot-core` exposes `pub trait DatabasePort: Send + Sync` with `async-trait`.
2. `paperpilot-storage::StorageEngine` satisfies `DatabasePort`.
3. `cargo test -p paperpilot-storage -j 4` passes 100% in-memory with zero regressions.
4. `cargo check --workspace -j 4` compiles with zero warnings or errors.
