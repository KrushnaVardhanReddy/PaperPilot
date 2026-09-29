use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct IntegrityHashOperation {
    pub filepath: PathBuf,
    pub hash_result: Arc<Mutex<Option<(String, usize)>>>,
}

impl IntegrityHashOperation {
    pub fn new(filepath: PathBuf) -> Self {
        Self {
            filepath,
            hash_result: Arc::new(Mutex::new(None)),
        }
    }
}

impl PdfOperation for IntegrityHashOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> OperationResult<()> {
        let mut file = File::open(&self.filepath)
            .map_err(|e| PdfError::Other(format!("Failed to open file for hashing: {}", e)))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        let mut total_size = 0;

        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| {
                PdfError::Other(format!("Failed to read file during hashing: {}", e))
            })?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
            total_size += bytes_read;
        }

        let hash = hasher.finalize();
        // Convert to hex string manually since the exact format specifier wasn't supported
        let hash_hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();

        if let Ok(mut lock) = self.hash_result.lock() {
            *lock = Some((hash_hex, total_size));
        } else {
            return Err(PdfError::Other("Failed to acquire mutex lock".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_integrity_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"dummy pdf content").unwrap();

        let op = IntegrityHashOperation::new(temp_file.path().to_path_buf());
        let mut doc = LopdfDocument::new();

        assert!(op.execute(&mut doc).is_ok());

        let result = op.hash_result.lock().unwrap();
        assert!(result.is_some());

        let (hash, size) = result.as_ref().unwrap();
        assert_eq!(*size, 17); // "dummy pdf content".len()
        assert_eq!(hash.len(), 64); // SHA-256 hash length in hex
    }
}
