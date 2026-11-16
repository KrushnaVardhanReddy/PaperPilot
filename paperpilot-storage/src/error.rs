use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Db(#[from] libsql::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Job not found: {0}")]
    NotFound(String),
}

impl From<StorageError> for paperpilot_core::storage::StorageError {
    fn from(e: StorageError) -> Self {
        match e {
            StorageError::Db(err) => paperpilot_core::storage::StorageError::Db(err.to_string()),
            StorageError::Serialization(err) => paperpilot_core::storage::StorageError::Serialization(err.to_string()),
            StorageError::NotFound(err) => paperpilot_core::storage::StorageError::NotFound(err),
        }
    }
}
