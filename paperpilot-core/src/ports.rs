use crate::error::OperationResult;
use std::path::Path;

/// Output port — abstracts where the output PDF bytes are written to.
///
/// Implementations:
/// - `LocalFileStorage`  → writes to the local filesystem (used today)
/// - `MemoryStorage`     → holds bytes in a Vec (used in tests — no temp files)
/// - `S3Storage`         → uploads to S3/GCS (Phase 5 cloud)
///
/// By depending on this port instead of `std::fs::write`, operations stay
/// testable and infrastructure-agnostic.
pub trait StoragePort: Send + Sync {
    /// Write raw bytes to the storage backend under the given name/key.
    /// `name` is a relative path, S3 key, or any identifier the adapter interprets.
    fn write(&self, name: &str, data: &[u8]) -> OperationResult<()>;

    /// Read raw bytes back from the storage backend.
    fn read(&self, name: &str) -> OperationResult<Vec<u8>>;

    /// Check whether a given name/key exists in the storage backend.
    fn exists(&self, name: &str) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// Local filesystem adapter (the default — works like before)
// ─────────────────────────────────────────────────────────────────────────────

/// Adapter that satisfies `StoragePort` by reading/writing the local filesystem.
/// This is the default for CLI and desktop usage.
pub struct LocalFileStorage;

impl StoragePort for LocalFileStorage {
    fn write(&self, name: &str, data: &[u8]) -> OperationResult<()> {
        use crate::error::PdfError;
        if let Some(parent) = Path::new(name).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(PdfError::IoError)?;
            }
        }
        std::fs::write(name, data).map_err(PdfError::IoError)
    }

    fn read(&self, name: &str) -> OperationResult<Vec<u8>> {
        use crate::error::PdfError;
        std::fs::read(name).map_err(PdfError::IoError)
    }

    fn exists(&self, name: &str) -> bool {
        Path::new(name).exists()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// In-memory adapter (used in unit tests — no disk I/O, no temp files)
// ─────────────────────────────────────────────────────────────────────────────

/// Adapter that satisfies `StoragePort` by holding bytes in memory.
/// Use this in tests instead of writing to the filesystem.
///
/// ```rust
/// use paperpilot_core::ports::MemoryStorage;
/// use paperpilot_core::ports::StoragePort;
///
/// let store = MemoryStorage::new();
/// store.write("output.pdf", b"fake-pdf-bytes").unwrap();
/// assert!(store.exists("output.pdf"));
/// let data = store.read("output.pdf").unwrap();
/// assert_eq!(data, b"fake-pdf-bytes");
/// ```
pub struct MemoryStorage {
    inner: std::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Retrieve all stored data as a snapshot (useful for test assertions).
    pub fn snapshot(&self) -> std::collections::HashMap<String, Vec<u8>> {
        self.inner.read().unwrap().clone()
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StoragePort for MemoryStorage {
    fn write(&self, name: &str, data: &[u8]) -> OperationResult<()> {
        self.inner
            .write()
            .unwrap()
            .insert(name.to_string(), data.to_vec());
        Ok(())
    }

    fn read(&self, name: &str) -> OperationResult<Vec<u8>> {
        use crate::error::PdfError;
        self.inner
            .read()
            .unwrap()
            .get(name)
            .cloned()
            .ok_or_else(|| PdfError::InvalidInput(format!("Key not found: {name}")))
    }

    fn exists(&self, name: &str) -> bool {
        self.inner.read().unwrap().contains_key(name)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PdfBackend port — abstracts which PDF library is used under the hood
// ─────────────────────────────────────────────────────────────────────────────

/// Input port — abstracts which PDF engine is used to open a document.
///
/// Current adapter: `LopdfBackend` (in `paperpilot-pdf`)
/// Future adapters: `PdfiumBackend`, `MuPdfBackend`
///
/// This allows swapping the entire PDF engine without touching any operation code.
pub trait PdfBackend: Send + Sync {
    type Document: crate::traits::PdfDocument;

    /// Load a PDF document from raw bytes.
    fn load_from_bytes(&self, bytes: &[u8]) -> OperationResult<Self::Document>;

    /// Load a PDF document from a filesystem path.
    fn load_from_path(&self, path: &Path) -> OperationResult<Self::Document>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage_write_read_exists() {
        let store = MemoryStorage::new();
        assert!(!store.exists("test.pdf"));

        store.write("test.pdf", b"hello world").unwrap();
        assert!(store.exists("test.pdf"));

        let data = store.read("test.pdf").unwrap();
        assert_eq!(data, b"hello world");
    }

    #[test]
    fn test_memory_storage_read_missing_returns_err() {
        let store = MemoryStorage::new();
        let result = store.read("ghost.pdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_storage_overwrite() {
        let store = MemoryStorage::new();
        store.write("file.pdf", b"v1").unwrap();
        store.write("file.pdf", b"v2").unwrap();
        let data = store.read("file.pdf").unwrap();
        assert_eq!(data, b"v2");
    }

    #[test]
    fn test_memory_storage_snapshot() {
        let store = MemoryStorage::new();
        store.write("a.pdf", b"aaa").unwrap();
        store.write("b.pdf", b"bbb").unwrap();
        let snap = store.snapshot();
        assert_eq!(snap.len(), 2);
        assert!(snap.contains_key("a.pdf"));
    }
}
