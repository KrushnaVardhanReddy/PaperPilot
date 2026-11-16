use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JobRecord {
    pub id: String,
    pub operation: String,
    pub status: JobStatus,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub parameters_json: String,
    pub error_message: Option<String>,
    pub progress_percentage: u8,
    pub callback_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewJob {
    pub operation: String,
    pub input_path: Option<String>,
    pub output_path: Option<String>,
    pub parameters_json: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuditEvent {
    pub id: String,
    pub event_type: String,
    pub tool_id: String,
    pub actor: String,
    pub payload_hash: String,
    pub details_json: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewDocumentChunk {
    pub id: Option<String>,
    pub document_hash: String,
    pub document_name: String,
    pub page_number: u32,
    pub chunk_index: u32,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DocumentChunkRecord {
    pub id: String,
    pub document_hash: String,
    pub document_name: String,
    pub page_number: u32,
    pub chunk_index: u32,
    pub content: String,
    pub embedding: Vec<f32>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VectorSearchResult {
    pub chunk: DocumentChunkRecord,
    pub score: f32,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Db(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Job not found: {0}")]
    NotFound(String),
    #[error("Backend error: {0}")]
    Backend(String),
}

#[async_trait::async_trait]
pub trait DatabasePort: Send + Sync {
    async fn create_job(&self, job: NewJob) -> Result<String, StorageError>;
    async fn get_job(&self, id: &str) -> Result<Option<JobRecord>, StorageError>;
    async fn update_job_status(&self, id: &str, status: JobStatus, progress: u8, output: Option<&str>, error: Option<&str>) -> Result<(), StorageError>;
    async fn list_jobs(&self, limit: usize) -> Result<Vec<JobRecord>, StorageError>;

    async fn store_document_chunks(&self, chunks: &[DocumentChunkRecord]) -> Result<(), StorageError>;
    async fn search_similar_chunks(&self, query_embedding: &[f32], limit: usize, min_similarity: f32) -> Result<Vec<VectorSearchResult>, StorageError>;
    async fn delete_document_chunks(&self, document_hash: &str) -> Result<usize, StorageError>;

    async fn append_audit_log(&self, event: AuditEvent) -> Result<(), StorageError>;
    async fn list_audit_logs(&self, limit: usize) -> Result<Vec<AuditEvent>, StorageError>;
}
