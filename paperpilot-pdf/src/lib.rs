pub mod document;
pub mod operations;
pub mod validation;

pub use paperpilot_core::document_source::{
    DEFAULT_MMAP_THRESHOLD_BYTES, DocumentSource, PageBatchIterator,
};
pub use operations::pdf_chunk::{
    chunk_pdf_bytes, chunk_pdf_path, chunk_pdf_source, pdf_chunk_bytes, ChunkOptions, ChunkResult,
    DocumentChunk, PdfChunkOperation,
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
