pub mod document;
pub mod operations;
pub mod validation;

pub use paperpilot_core::document_source::{
    DEFAULT_MMAP_THRESHOLD_BYTES, DocumentSource, PageBatchIterator,
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
