use std::ops::{Deref, RangeInclusive};
use std::path::Path;

/// Default threshold above which files are memory-mapped on native targets (10 MB).
pub const DEFAULT_MMAP_THRESHOLD_BYTES: u64 = 10 * 1024 * 1024;

/// A zero-copy abstraction for document data.
///
/// On native platforms, files equal to or larger than the configured threshold (default 10 MB)
/// are memory-mapped via [`memmap2::Mmap`], avoiding heap allocation and sudden RAM spikes.
/// Small files, in-memory buffers, and all files when compiling for WASM targets fall back
/// safely to heap-allocated byte vectors.
pub enum DocumentSource {
    /// In-memory byte buffer (used for WASM, small files, or dynamic byte slices)
    Bytes(Vec<u8>),
    /// Memory-mapped file slice (used on native OS for files >= threshold)
    #[cfg(not(target_arch = "wasm32"))]
    Mmap(memmap2::Mmap),
}

impl Deref for DocumentSource {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            DocumentSource::Bytes(bytes) => bytes.as_slice(),
            #[cfg(not(target_arch = "wasm32"))]
            DocumentSource::Mmap(mmap) => mmap.deref(),
        }
    }
}

impl AsRef<[u8]> for DocumentSource {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl std::fmt::Debug for DocumentSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentSource::Bytes(bytes) => {
                write!(f, "DocumentSource::Bytes(len = {})", bytes.len())
            }
            #[cfg(not(target_arch = "wasm32"))]
            DocumentSource::Mmap(mmap) => {
                write!(f, "DocumentSource::Mmap(len = {})", mmap.len())
            }
        }
    }
}

impl DocumentSource {
    /// Creates a `DocumentSource` taking ownership of a `Vec<u8>`.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        DocumentSource::Bytes(bytes)
    }

    /// Creates a `DocumentSource` by copying from a byte slice.
    pub fn from_slice(slice: &[u8]) -> Self {
        DocumentSource::Bytes(slice.to_vec())
    }

    /// Opens a document from the specified path.
    ///
    /// On non-WASM targets, if the file size is greater than or equal to `threshold_bytes`
    /// (defaulting to [`DEFAULT_MMAP_THRESHOLD_BYTES`] = 10 MB if `None`), it creates a
    /// memory-mapped view via `memmap2`. Otherwise (or on WASM), it reads the file contents
    /// into a standard heap buffer via `std::fs::read`.
    pub fn open<P: AsRef<Path>>(path: P, threshold_bytes: Option<u64>) -> std::io::Result<Self> {
        let _threshold_bytes = threshold_bytes;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path_ref = path.as_ref();
            let file = std::fs::File::open(path_ref)?;
            let meta = file.metadata()?;
            let threshold = _threshold_bytes.unwrap_or(DEFAULT_MMAP_THRESHOLD_BYTES);

            if meta.len() >= threshold && meta.len() > 0 {
                // Safety: Memory mapping requires that the underlying file is not concurrently truncated
                // or modified in an incompatible manner by other processes. DocumentSource is intended
                // for read-heavy operations where documents are read-only.
                let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
                return Ok(DocumentSource::Mmap(mmap));
            }
        }

        let bytes = std::fs::read(path)?;
        Ok(DocumentSource::Bytes(bytes))
    }

    /// Returns `true` if this document source is backed by a memory-mapped file.
    #[inline]
    pub fn is_mmap(&self) -> bool {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            DocumentSource::Mmap(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this document source is backed by an in-memory byte buffer.
    #[inline]
    pub fn is_bytes(&self) -> bool {
        matches!(self, DocumentSource::Bytes(_))
    }

    /// Returns the length of the underlying document in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.deref().len()
    }

    /// Returns `true` if the document source contains no bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.deref().is_empty()
    }

    /// Returns a slice view of the entire document bytes.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.deref()
    }

    /// Converts or copies the document into an owned `Vec<u8>`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.deref().to_vec()
    }

    /// Creates a `PageBatchIterator` for splitting operations over the document.
    pub fn page_batches(&self, total_pages: usize, batch_size: usize) -> PageBatchIterator {
        PageBatchIterator::new(total_pages, batch_size)
    }
}

/// An iterator yielding 1-based inclusive page ranges (`1..=25`, `26..=50`, etc.)
/// to standardize chunked and streamed processing across batch-enabled tools.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageBatchIterator {
    pub total_pages: usize,
    pub batch_size: usize,
    pub current_page: usize,
}

impl PageBatchIterator {
    /// Creates a new `PageBatchIterator`.
    ///
    /// If `batch_size` is 0, it defaults to 1 to prevent infinite loops.
    pub fn new(total_pages: usize, batch_size: usize) -> Self {
        Self {
            total_pages,
            batch_size: if batch_size == 0 { 1 } else { batch_size },
            current_page: 1,
        }
    }
}

impl Iterator for PageBatchIterator {
    type Item = RangeInclusive<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_pages == 0 || self.current_page > self.total_pages {
            return None;
        }

        let start = self.current_page;
        let end = (start + self.batch_size - 1).min(self.total_pages);
        self.current_page = end + 1;
        Some(start..=end)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.total_pages == 0 || self.current_page > self.total_pages {
            (0, Some(0))
        } else {
            let remaining_pages = self.total_pages - self.current_page + 1;
            let remaining_batches = (remaining_pages + self.batch_size - 1) / self.batch_size;
            (remaining_batches, Some(remaining_batches))
        }
    }
}

impl ExactSizeIterator for PageBatchIterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_from_bytes_and_slice() {
        let raw = b"PaperPilot Spec 163 Zero-Copy".to_vec();
        let src_bytes = DocumentSource::from_bytes(raw.clone());
        assert_eq!(src_bytes.len(), raw.len());
        assert_eq!(&src_bytes[0..10], b"PaperPilot");
        assert_eq!(src_bytes.as_ref(), raw.as_slice());
        assert!(src_bytes.is_bytes());
        assert!(!src_bytes.is_mmap());

        let slice = b"Slice Source";
        let src_slice = DocumentSource::from_slice(slice);
        assert_eq!(src_slice.as_ref(), slice);
        assert_eq!(src_slice.to_bytes(), slice.to_vec());
    }

    #[test]
    fn test_deref_and_slicing() {
        let content = b"0123456789abcdef".to_vec();
        let src = DocumentSource::from_bytes(content);
        assert_eq!(&src[0..4], b"0123");
        assert_eq!(&src[10..16], b"abcdef");
        assert_eq!(src.len(), 16);
        assert!(!src.is_empty());
    }

    #[test]
    fn test_open_small_file_below_threshold() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        let sample = b"Small Document Under 10MB Threshold";
        tmp.write_all(sample).unwrap();
        tmp.flush().unwrap();

        let src = DocumentSource::open(tmp.path(), None).unwrap();
        assert_eq!(src.as_ref(), sample);
        assert!(src.is_bytes());
        assert!(!src.is_mmap());
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_open_mmap_above_threshold() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        let sample = vec![0x42u8; 4096];
        tmp.write_all(&sample).unwrap();
        tmp.flush().unwrap();

        // Pass threshold of 2048 bytes; 4096 >= 2048 -> selects Mmap
        let src = DocumentSource::open(tmp.path(), Some(2048)).unwrap();
        assert_eq!(src.as_ref(), sample.as_slice());
        assert!(src.is_mmap());
        assert!(!src.is_bytes());
        assert_eq!(src.len(), 4096);
    }

    #[test]
    fn test_open_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let src = DocumentSource::open(tmp.path(), Some(0)).unwrap();
        assert!(src.is_empty());
        assert_eq!(src.len(), 0);
        assert!(src.is_bytes());
    }

    #[test]
    fn test_page_batch_iterator_standard_100_pages() {
        let iter = PageBatchIterator::new(100, 25);
        assert_eq!(iter.len(), 4);
        let batches: Vec<RangeInclusive<usize>> = iter.collect();
        assert_eq!(
            batches,
            vec![1..=25, 26..=50, 51..=75, 76..=100]
        );
    }

    #[test]
    fn test_page_batch_iterator_uneven_split() {
        let iter = PageBatchIterator::new(10, 4);
        assert_eq!(iter.len(), 3);
        let batches: Vec<RangeInclusive<usize>> = iter.collect();
        assert_eq!(batches, vec![1..=4, 5..=8, 9..=10]);
    }

    #[test]
    fn test_page_batch_iterator_edge_cases() {
        // 0 pages
        let mut iter_zero = PageBatchIterator::new(0, 10);
        assert_eq!(iter_zero.len(), 0);
        assert_eq!(iter_zero.next(), None);

        // batch_size larger than total_pages
        let mut iter_large_batch = PageBatchIterator::new(5, 50);
        assert_eq!(iter_large_batch.len(), 1);
        assert_eq!(iter_large_batch.next(), Some(1..=5));
        assert_eq!(iter_large_batch.next(), None);

        // batch_size = 0 is safely clamped to 1
        let iter_batch_zero = PageBatchIterator::new(3, 0);
        assert_eq!(iter_batch_zero.batch_size, 1);
        let batches: Vec<RangeInclusive<usize>> = iter_batch_zero.collect();
        assert_eq!(batches, vec![1..=1, 2..=2, 3..=3]);
    }
}
