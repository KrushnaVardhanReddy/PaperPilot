use lopdf::Document;
use paperpilot_core::document_source::DocumentSource;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfDocument;
use std::any::Any;
use std::path::Path;
use std::sync::Arc;

pub struct LopdfDocument {
    pub inner: Document,
    pub original_bytes: Option<Vec<u8>>,
    pub source: Option<Arc<DocumentSource>>,
}

impl Default for LopdfDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl LopdfDocument {
    pub fn new() -> Self {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", lopdf::Object::Integer(0));
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));

        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

        let mut catalog = lopdf::Dictionary::new();
        catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);

        doc.trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        Self {
            inner: doc,
            original_bytes: None,
            source: None,
        }
    }

    pub fn from_inner(inner: Document) -> Self {
        Self {
            inner,
            original_bytes: None,
            source: None,
        }
    }

    pub fn from_inner_with_bytes(inner: Document, bytes: Vec<u8>) -> Self {
        Self {
            inner,
            original_bytes: Some(bytes),
            source: None,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> OperationResult<Self> {
        let doc = crate::operations::repair::RepairOperation::load_document_resilient(bytes)?;
        Ok(Self {
            inner: doc,
            original_bytes: Some(bytes.to_vec()),
            source: None,
        })
    }

    /// Wraps an already-opened [`memmap2::Mmap`] for zero-copy streaming without duplicating
    /// the document buffer on the heap.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_mmap(mmap: memmap2::Mmap) -> OperationResult<Self> {
        let source = Arc::new(DocumentSource::Mmap(mmap));
        Self::from_shared_source(source)
    }

    /// Loads a PDF directly using `memmap2::Mmap::map` on native targets, bypassing full
    /// in-memory heap buffering. On WASM targets, falls back cleanly to `std::fs::read`.
    pub fn from_file_mmap<P: AsRef<Path>>(path: P) -> OperationResult<Self> {
        let path_ref = path.as_ref();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = std::fs::File::open(path_ref).map_err(PdfError::IoError)?;
            let meta = file.metadata().map_err(PdfError::IoError)?;
            if meta.len() == 0 {
                return Err(PdfError::ParseError(
                    "Cannot memory-map empty PDF file".to_string(),
                ));
            }
            // Safety: Document loading treats mapped bytes as immutable read-only slices.
            let mmap = unsafe { memmap2::Mmap::map(&file).map_err(PdfError::IoError)? };
            Self::from_mmap(mmap)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let bytes = std::fs::read(path_ref).map_err(PdfError::IoError)?;
            if bytes.is_empty() {
                return Err(PdfError::ParseError(
                    "Cannot load empty PDF file".to_string(),
                ));
            }
            Self::from_shared_source(Arc::new(DocumentSource::from_bytes(bytes)))
        }
    }

    /// Loads a PDF from a shared [`DocumentSource`] without cloning the backing slice onto the heap.
    pub fn from_shared_source(source: Arc<DocumentSource>) -> OperationResult<Self> {
        let doc =
            crate::operations::repair::RepairOperation::load_document_resilient(source.as_slice())?;
        Ok(Self {
            inner: doc,
            original_bytes: None,
            source: Some(source),
        })
    }

    pub fn from_source(source: &DocumentSource) -> OperationResult<Self> {
        if source.is_mmap() {
            let doc = crate::operations::repair::RepairOperation::load_document_resilient(
                source.as_slice(),
            )?;
            Ok(Self {
                inner: doc,
                original_bytes: None,
                source: None,
            })
        } else {
            Self::from_bytes(source.as_ref())
        }
    }

    pub fn load_with_threshold(path: &Path, threshold_bytes: Option<u64>) -> OperationResult<Self> {
        let source =
            Arc::new(DocumentSource::open(path, threshold_bytes).map_err(PdfError::IoError)?);
        Self::from_shared_source(source)
    }

    pub fn load(path: &Path) -> OperationResult<Self> {
        Self::load_with_threshold(path, None)
    }

    /// Returns a zero-copy view of the raw backing bytes whether stored in `original_bytes`
    /// or inside a shared/memory-mapped [`DocumentSource`].
    #[inline]
    pub fn source_bytes(&self) -> Option<&[u8]> {
        self.original_bytes
            .as_deref()
            .or_else(|| self.source.as_deref().map(DocumentSource::as_slice))
    }

    /// Returns the shared [`DocumentSource`] handle when the document was loaded zero-copy.
    #[inline]
    pub fn shared_source(&self) -> Option<&Arc<DocumentSource>> {
        self.source.as_ref()
    }

    /// Returns the byte length of the backing raw document, if available.
    #[inline]
    pub fn source_len(&self) -> Option<usize> {
        self.source_bytes().map(|b| b.len())
    }

    /// Returns `true` if this document is backed by an active `memmap2::Mmap` view.
    #[inline]
    pub fn is_mmap(&self) -> bool {
        self.source.as_deref().is_some_and(DocumentSource::is_mmap)
    }

    /// Extracts plain text across pages in parallel on native targets (via `rayon`), falling back
    /// cleanly to sequential extraction on WASM targets.
    pub fn extract_text_parallel(&self, pages: Option<&[u32]>) -> OperationResult<Vec<String>> {
        crate::operations::extract_text::extract_lopdf_pages_parallel(&self.inner, pages)
    }
}

impl PdfDocument for LopdfDocument {
    fn page_count(&self) -> OperationResult<u32> {
        Ok(self.inner.get_pages().len() as u32)
    }

    fn save(&self, path: &Path) -> OperationResult<()> {
        let mut inner = self.inner.clone();
        inner
            .save(path)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
        Ok(())
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_new_document() {
        let doc = LopdfDocument::new();
        assert_eq!(doc.page_count().unwrap(), 0);
    }

    #[test]
    fn test_save_and_load_document() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.pdf");

        let doc = LopdfDocument::new();
        doc.save(&file_path).unwrap();

        assert!(file_path.exists());

        let loaded_doc = LopdfDocument::load(&file_path).unwrap();
        assert_eq!(loaded_doc.page_count().unwrap(), 0);
    }
}
