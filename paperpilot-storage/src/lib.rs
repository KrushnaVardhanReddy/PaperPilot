pub mod error;
pub mod models;
pub mod vector;

use error::StorageError;
use models::*;
use std::path::Path;

#[derive(Clone)]
pub struct StorageEngine {
    #[allow(dead_code)]
    db: std::sync::Arc<libsql::Database>,
    conn: std::sync::Arc<libsql::Connection>,
}

impl StorageEngine {
    pub async fn open_local(path: &Path) -> Result<Self, StorageError> {
        let path_str = path.to_string_lossy().to_string();
        let db = libsql::Builder::new_local(path_str).build().await?;
        let conn = db.connect()?;
        let engine = Self {
            db: std::sync::Arc::new(db),
            conn: std::sync::Arc::new(conn),
        };
        engine.run_migrations().await?;
        Ok(engine)
    }

    pub async fn open_in_memory() -> Result<Self, StorageError> {
        let db = libsql::Builder::new_local(":memory:").build().await?;
        let conn = db.connect()?;
        let engine = Self {
            db: std::sync::Arc::new(db),
            conn: std::sync::Arc::new(conn),
        };
        engine.run_migrations().await?;
        Ok(engine)
    }

    pub async fn run_migrations(&self) -> Result<(), StorageError> {
        self.conn
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id TEXT PRIMARY KEY,
                operation TEXT NOT NULL,
                status TEXT NOT NULL,
                input_path TEXT,
                output_path TEXT,
                parameters_json TEXT NOT NULL DEFAULT '{}',
                error_message TEXT,
                progress_percentage INTEGER NOT NULL DEFAULT 0,
                callback_url TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS webhooks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                target_url TEXT NOT NULL,
                secret_token TEXT,
                events_json TEXT NOT NULL,
                is_active BOOLEAN NOT NULL DEFAULT 1,
                retry_count INTEGER NOT NULL DEFAULT 3,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS document_chunks (
                id TEXT PRIMARY KEY,
                document_hash TEXT NOT NULL,
                document_name TEXT NOT NULL,
                page_number INTEGER NOT NULL,
                chunk_index INTEGER NOT NULL,
                content TEXT NOT NULL,
                embedding_json TEXT NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS audit_logs (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                tool_id TEXT NOT NULL,
                actor TEXT NOT NULL DEFAULT 'local_user',
                payload_hash TEXT NOT NULL,
                details_json TEXT NOT NULL DEFAULT '{}',
                timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
        "#,
            )
            .await?;
        Ok(())
    }

    pub async fn create_job(&self, job: &NewJob) -> Result<JobRecord, StorageError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let params = job
            .parameters_json
            .clone()
            .unwrap_or_else(|| "{}".to_string());

        self.conn.execute(
            "INSERT INTO jobs (id, operation, status, input_path, output_path, parameters_json, progress_percentage, callback_url, created_at, updated_at)
             VALUES (?1, ?2, 'pending', ?3, ?4, ?5, 0, ?6, ?7, ?7)",
            libsql::params![
                id.clone(),
                job.operation.clone(),
                job.input_path.clone(),
                job.output_path.clone(),
                params.clone(),
                job.callback_url.clone(),
                now.clone(),
            ]
        ).await?;

        Ok(JobRecord {
            id,
            operation: job.operation.clone(),
            status: JobStatus::Pending,
            input_path: job.input_path.clone(),
            output_path: job.output_path.clone(),
            parameters_json: params,
            error_message: None,
            progress_percentage: 0,
            callback_url: job.callback_url.clone(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn update_job_status(
        &self,
        id: &str,
        status: JobStatus,
        progress: u8,
        output: Option<&str>,
        error: Option<&str>,
    ) -> Result<(), StorageError> {
        let now = chrono::Utc::now().to_rfc3339();
        let status_str = match status {
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
        };

        self.conn.execute(
            "UPDATE jobs SET status = ?1, progress_percentage = ?2, output_path = COALESCE(?3, output_path), error_message = ?4, updated_at = ?5 WHERE id = ?6",
            libsql::params![status_str, progress as i64, output.map(|s| s.to_string()), error.map(|s| s.to_string()), now, id.to_string()]
        ).await?;

        Ok(())
    }

    pub async fn get_job(&self, id: &str) -> Result<Option<JobRecord>, StorageError> {
        let mut rows = self.conn.query(
            "SELECT id, operation, status, input_path, output_path, parameters_json, error_message, progress_percentage, callback_url, created_at, updated_at FROM jobs WHERE id = ?1",
            libsql::params![id.to_string()]
        ).await?;

        if let Some(row) = rows.next().await? {
            let status_str: String = row.get(2)?;
            let status = match status_str.as_str() {
                "running" => JobStatus::Running,
                "completed" => JobStatus::Completed,
                "failed" => JobStatus::Failed,
                _ => JobStatus::Pending,
            };

            let prog: i64 = row.get(7)?;
            Ok(Some(JobRecord {
                id: row.get(0)?,
                operation: row.get(1)?,
                status,
                input_path: row.get(3).ok(),
                output_path: row.get(4).ok(),
                parameters_json: row.get(5)?,
                error_message: row.get(6).ok(),
                progress_percentage: prog as u8,
                callback_url: row.get(8).ok(),
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_jobs(&self, limit: usize) -> Result<Vec<JobRecord>, StorageError> {
        let mut rows = self.conn.query(
            "SELECT id, operation, status, input_path, output_path, parameters_json, error_message, progress_percentage, callback_url, created_at, updated_at FROM jobs ORDER BY created_at DESC LIMIT ?1",
            libsql::params![limit as i64]
        ).await?;

        let mut jobs = Vec::new();
        while let Some(row) = rows.next().await? {
            let status_str: String = row.get(2)?;
            let status = match status_str.as_str() {
                "running" => JobStatus::Running,
                "completed" => JobStatus::Completed,
                "failed" => JobStatus::Failed,
                _ => JobStatus::Pending,
            };

            let prog: i64 = row.get(7)?;
            jobs.push(JobRecord {
                id: row.get(0)?,
                operation: row.get(1)?,
                status,
                input_path: row.get(3).ok(),
                output_path: row.get(4).ok(),
                parameters_json: row.get(5)?,
                error_message: row.get(6).ok(),
                progress_percentage: prog as u8,
                callback_url: row.get(8).ok(),
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            });
        }
        Ok(jobs)
    }

    pub async fn record_audit_event(&self, event: &AuditEvent) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO audit_logs (id, event_type, tool_id, actor, payload_hash, details_json, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            libsql::params![
                event.id.clone(),
                event.event_type.clone(),
                event.tool_id.clone(),
                event.actor.clone(),
                event.payload_hash.clone(),
                event.details_json.clone(),
                event.timestamp.clone(),
            ]
        ).await?;
        Ok(())
    }

    pub async fn list_audit_events(&self, limit: usize) -> Result<Vec<AuditEvent>, StorageError> {
        let mut rows = self.conn.query(
            "SELECT id, event_type, tool_id, actor, payload_hash, details_json, timestamp FROM audit_logs ORDER BY timestamp DESC LIMIT ?1",
            libsql::params![limit as i64]
        ).await?;

        let mut events = Vec::new();
        while let Some(row) = rows.next().await? {
            events.push(AuditEvent {
                id: row.get(0)?,
                event_type: row.get(1)?,
                tool_id: row.get(2)?,
                actor: row.get(3)?,
                payload_hash: row.get(4)?,
                details_json: row.get(5)?,
                timestamp: row.get(6)?,
            });
        }
        Ok(events)
    }

    pub async fn store_chunks(&self, chunks: &[NewDocumentChunk]) -> Result<usize, StorageError> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut count = 0;
        for chunk in chunks {
            let id = chunk
                .id
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let embedding_json =
                serde_json::to_string(&chunk.embedding).unwrap_or_else(|_| "[]".to_string());

            self.conn.execute(
                "INSERT INTO document_chunks (id, document_hash, document_name, page_number, chunk_index, content, embedding_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                libsql::params![
                    id,
                    chunk.document_hash.clone(),
                    chunk.document_name.clone(),
                    chunk.page_number as i64,
                    chunk.chunk_index as i64,
                    chunk.content.clone(),
                    embedding_json,
                    now.clone(),
                ]
            ).await?;
            count += 1;
        }
        Ok(count)
    }

    pub async fn delete_chunks_by_document(
        &self,
        document_hash: &str,
    ) -> Result<usize, StorageError> {
        let changes = self
            .conn
            .execute(
                "DELETE FROM document_chunks WHERE document_hash = ?1",
                libsql::params![document_hash.to_string()],
            )
            .await?;
        Ok(changes as usize)
    }

    pub async fn get_chunks_by_document(
        &self,
        document_hash: &str,
    ) -> Result<Vec<DocumentChunkRecord>, StorageError> {
        let mut rows = self.conn.query(
            "SELECT id, document_hash, document_name, page_number, chunk_index, content, embedding_json, created_at FROM document_chunks WHERE document_hash = ?1 ORDER BY chunk_index",
            libsql::params![document_hash.to_string()]
        ).await?;

        let mut chunks = Vec::new();
        while let Some(row) = rows.next().await? {
            let embedding_json: String = row.get(6)?;
            let embedding: Vec<f32> = serde_json::from_str(&embedding_json).unwrap_or_default();

            chunks.push(DocumentChunkRecord {
                id: row.get(0)?,
                document_hash: row.get(1)?,
                document_name: row.get(2)?,
                page_number: row.get::<i64>(3)? as u32,
                chunk_index: row.get::<i64>(4)? as u32,
                content: row.get(5)?,
                embedding,
                created_at: row.get(7)?,
            });
        }
        Ok(chunks)
    }

    pub async fn count_chunks(&self) -> Result<usize, StorageError> {
        let mut rows = self
            .conn
            .query("SELECT COUNT(*) FROM document_chunks", ())
            .await?;
        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count as usize)
        } else {
            Ok(0)
        }
    }

    pub async fn search_chunks(
        &self,
        query_embedding: &[f32],
        limit: usize,
        filter_document_hash: Option<&str>,
    ) -> Result<Vec<SemanticSearchResult>, StorageError> {
        let mut rows = if let Some(hash) = filter_document_hash {
            self.conn.query(
                "SELECT id, document_hash, document_name, page_number, chunk_index, content, embedding_json, created_at FROM document_chunks WHERE document_hash = ?1",
                libsql::params![hash.to_string()]
            ).await?
        } else {
            self.conn.query(
                "SELECT id, document_hash, document_name, page_number, chunk_index, content, embedding_json, created_at FROM document_chunks",
                ()
            ).await?
        };

        let mut results = Vec::new();
        while let Some(row) = rows.next().await? {
            let embedding_json: String = row.get(6)?;
            let embedding: Vec<f32> = serde_json::from_str(&embedding_json).unwrap_or_default();

            let score = vector::cosine_similarity(query_embedding, &embedding);

            let chunk = DocumentChunkRecord {
                id: row.get(0)?,
                document_hash: row.get(1)?,
                document_name: row.get(2)?,
                page_number: row.get::<i64>(3)? as u32,
                chunk_index: row.get::<i64>(4)? as u32,
                content: row.get(5)?,
                embedding,
                created_at: row.get(7)?,
            };

            results.push(SemanticSearchResult { chunk, score });
        }

        // Sort descending by score
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_count_chunks() {
        let engine = StorageEngine::open_in_memory().await.unwrap();
        let chunks = vec![
            NewDocumentChunk {
                id: None,
                document_hash: "hash1".to_string(),
                document_name: "doc1".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "content 1".to_string(),
                embedding: vec![0.1, 0.2],
            },
            NewDocumentChunk {
                id: None,
                document_hash: "hash1".to_string(),
                document_name: "doc1".to_string(),
                page_number: 1,
                chunk_index: 2,
                content: "content 2".to_string(),
                embedding: vec![0.3, 0.4],
            },
            NewDocumentChunk {
                id: None,
                document_hash: "hash2".to_string(),
                document_name: "doc2".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "content 3".to_string(),
                embedding: vec![0.5, 0.6],
            },
        ];

        engine.store_chunks(&chunks).await.unwrap();
        assert_eq!(engine.count_chunks().await.unwrap(), 3);

        let doc1_chunks = engine.get_chunks_by_document("hash1").await.unwrap();
        assert_eq!(doc1_chunks.len(), 2);
    }

    #[tokio::test]
    async fn test_semantic_search_ranking() {
        let engine = StorageEngine::open_in_memory().await.unwrap();
        let chunks = vec![
            NewDocumentChunk {
                id: None,
                document_hash: "hash1".to_string(),
                document_name: "doc1".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "Chunk A".to_string(),
                embedding: vec![1.0, 0.0, 0.0],
            },
            NewDocumentChunk {
                id: None,
                document_hash: "hash2".to_string(),
                document_name: "doc2".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "Chunk B".to_string(),
                embedding: vec![0.1, 0.9, 0.0],
            },
        ];
        engine.store_chunks(&chunks).await.unwrap();

        let query = vec![1.0, 0.05, 0.0];
        let results = engine.search_chunks(&query, 2, None).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].chunk.content, "Chunk A");
        assert!(results[0].score > 0.95);
        assert_eq!(results[1].chunk.content, "Chunk B");
    }

    #[tokio::test]
    async fn test_delete_chunks_by_document() {
        let engine = StorageEngine::open_in_memory().await.unwrap();
        let chunks = vec![
            NewDocumentChunk {
                id: None,
                document_hash: "doc1".to_string(),
                document_name: "name1".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "A".to_string(),
                embedding: vec![1.0],
            },
            NewDocumentChunk {
                id: None,
                document_hash: "doc2".to_string(),
                document_name: "name2".to_string(),
                page_number: 1,
                chunk_index: 1,
                content: "B".to_string(),
                embedding: vec![2.0],
            },
        ];
        engine.store_chunks(&chunks).await.unwrap();
        assert_eq!(engine.count_chunks().await.unwrap(), 2);

        let deleted = engine.delete_chunks_by_document("doc1").await.unwrap();
        assert_eq!(deleted, 1);

        assert_eq!(engine.count_chunks().await.unwrap(), 1);
        let remaining = engine.get_chunks_by_document("doc2").await.unwrap();
        assert_eq!(remaining.len(), 1);
    }

    #[tokio::test]
    async fn test_storage_engine_in_memory_migrations() {
        let engine = StorageEngine::open_in_memory().await.unwrap();
        // Just opening it in memory runs migrations, so we can check if it succeeds.
        // Let's run a simple query to ensure tables exist
        let mut rows = engine
            .conn
            .query(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='jobs'",
                (),
            )
            .await
            .unwrap();
        if let Some(row) = rows.next().await.unwrap() {
            let count: i64 = row.get(0).unwrap();
            assert_eq!(count, 1);
        } else {
            panic!("Could not query sqlite_master");
        }
    }

    #[tokio::test]
    async fn test_job_lifecycle() {
        let engine = StorageEngine::open_in_memory().await.unwrap();

        let new_job = NewJob {
            operation: "test_op".to_string(),
            input_path: Some("in.txt".to_string()),
            output_path: Some("out.txt".to_string()),
            parameters_json: Some(r#"{"key": "value"}"#.to_string()),
            callback_url: None,
        };

        let job = engine.create_job(&new_job).await.unwrap();
        assert_eq!(job.status, JobStatus::Pending);
        assert_eq!(job.progress_percentage, 0);

        // Update to Running
        engine
            .update_job_status(&job.id, JobStatus::Running, 50, None, None)
            .await
            .unwrap();
        let running_job = engine.get_job(&job.id).await.unwrap().unwrap();
        assert_eq!(running_job.status, JobStatus::Running);
        assert_eq!(running_job.progress_percentage, 50);

        // Update to Completed
        engine
            .update_job_status(
                &job.id,
                JobStatus::Completed,
                100,
                Some("out_final.txt"),
                None,
            )
            .await
            .unwrap();
        let completed_job = engine.get_job(&job.id).await.unwrap().unwrap();
        assert_eq!(completed_job.status, JobStatus::Completed);
        assert_eq!(completed_job.progress_percentage, 100);
        assert_eq!(completed_job.output_path, Some("out_final.txt".to_string()));
    }

    #[tokio::test]
    async fn test_record_audit_event() {
        let engine = StorageEngine::open_in_memory().await.unwrap();

        let event = AuditEvent {
            id: "evt-123".to_string(),
            event_type: "execution".to_string(),
            tool_id: "pdf_extract".to_string(),
            actor: "test_user".to_string(),
            payload_hash: "abcd123".to_string(),
            details_json: r#"{"status": "ok"}"#.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        engine.record_audit_event(&event).await.unwrap();

        let mut rows = engine
            .conn
            .query("SELECT count(*) FROM audit_logs", ())
            .await
            .unwrap();
        if let Some(row) = rows.next().await.unwrap() {
            let count: i64 = row.get(0).unwrap();
            assert_eq!(count, 1);
        } else {
            panic!("Could not query audit_logs");
        }
    }
}
