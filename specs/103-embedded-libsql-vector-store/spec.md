# Spec 103: Embedded libSQL / Turso Local Vector Store & Persistent Engine (`paperpilot-storage`)

## 1. Background & Executive Summary
PaperPilot requires persistent, zero-cloud storage for:
1. **Asynchronous Jobs & Long-Running Operations** (`jobs` table: async status, parameters, progress, outputs).
2. **Outbound Webhooks & Integration Events** (`webhooks` table: URLs, event subscriptions, secrets, retry queues).
3. **Cross-Document Semantic Search & Vector Embeddings** (`document_embeddings` table: vector cosine search across all documents imported into a workspace).
4. **Compliance & Audit Logging** (`audit_logs` table: cryptographic tamper-evident operation records).

**Spec 103** implements this unified persistence engine using **[libSQL / Turso](https://github.com/tursodatabase/turso)** (`libsql = "0.10.0"`).
It provides:
- **Zero-Cloud Local-First Mode**: Single embedded SQLite database file located in `get_app_data_root()` (`./data/paperpilot.db` or OS AppData).
- **Optional Enterprise Edge Sync**: Native support for 1-line replication to remote Turso cloud or private edge nodes without rewriting application code.
- **Pure-Rust Modular Crate**: Packaged cleanly inside `paperpilot-storage` within the workspace, ensuring zero dependency conflict with `paperpilot-pdf` or client-side WASM builds.

---

## 2. Technical Architecture & Data Model

### 2.1 Workspace Structure
A new workspace crate: `paperpilot-storage`
- Dependencies:
  - `libsql = "0.10.0"`
  - `tokio = { version = "1", features = ["full"] }`
  - `serde = { version = "1.0", features = ["derive"] }`
  - `serde_json = "1.0"`
  - `thiserror = "2.0"`
  - `uuid = { version = "1.15", features = ["v4", "serde"] }`
  - `chrono = { version = "0.4", features = ["serde"] }`

### 2.2 Database Schema & Migrations

```sql
-- 1. Asynchronous Job Processing Queue
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    operation TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'failed')),
    input_path TEXT,
    output_path TEXT,
    parameters_json TEXT NOT NULL DEFAULT '{}',
    error_message TEXT,
    progress_percentage INTEGER NOT NULL DEFAULT 0,
    callback_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Outbound Webhook Subscriptions
CREATE TABLE IF NOT EXISTS webhooks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    target_url TEXT NOT NULL,
    secret_token TEXT,
    events_json TEXT NOT NULL, -- e.g. ["job.completed", "signature.verified"]
    is_active BOOLEAN NOT NULL DEFAULT 1,
    retry_count INTEGER NOT NULL DEFAULT 3,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Cross-Document Semantic Embeddings & Search
CREATE TABLE IF NOT EXISTS document_chunks (
    id TEXT PRIMARY KEY,
    document_hash TEXT NOT NULL,
    document_name TEXT NOT NULL,
    page_number INTEGER NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    embedding_json TEXT NOT NULL, -- JSON array of floats (e.g. 384-dim or 768-dim)
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. Tamper-Evident Audit Compliance Log
CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    tool_id TEXT NOT NULL,
    actor TEXT NOT NULL DEFAULT 'local_user',
    payload_hash TEXT NOT NULL,
    details_json TEXT NOT NULL DEFAULT '{}',
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

---

## 3. High-Level Rust API Surface

In `paperpilot-storage/src/lib.rs`:

```rust
pub struct StorageEngine {
    db: libsql::Database,
    conn: libsql::Connection,
}

impl StorageEngine {
    pub async fn open_local(db_path: &Path) -> Result<Self, StorageError>;
    pub async fn open_remote(url: &str, auth_token: &str) -> Result<Self, StorageError>;
    
    // Schema
    pub async fn run_migrations(&self) -> Result<(), StorageError>;

    // Job Operations
    pub async fn create_job(&self, job: &NewJob) -> Result<JobRecord, StorageError>;
    pub async fn update_job_status(&self, id: &str, status: JobStatus, progress: u8, output: Option<&str>, error: Option<&str>) -> Result<(), StorageError>;
    pub async fn get_job(&self, id: &str) -> Result<Option<JobRecord>, StorageError>;
    pub async fn list_jobs(&self, limit: usize) -> Result<Vec<JobRecord>, StorageError>;

    // Vector Search
    pub async fn store_chunks(&self, chunks: &[NewChunk]) -> Result<(), StorageError>;
    pub async fn search_chunks(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>, StorageError>;

    // Webhooks & Audit
    pub async fn record_audit_event(&self, event: &AuditEvent) -> Result<(), StorageError>;
}
```

---

## 4. Multi-Phase Implementation Plan
To prevent merge conflicts and ensure clean step-by-step verification, Spec 103 is split into:
- **Task 103A (Backend Engine)**: Crate scaffolding, `libsql` local embedded database, migrations, async job queue API, and unit tests.
- **Task 103B (Vector Store & Semantic Search)**: Cosine similarity vector search implementation and cross-document chunking integration.
- **Task 103C (Desktop UI & Operations Dock Integration)**: Async job queue viewer drawer and Tauri command wiring in `apps/desktop`.
