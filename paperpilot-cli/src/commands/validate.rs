use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;

pub fn handle_validate(input: &std::path::Path) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::validate::ValidateOperation::new();
    op.execute(&mut doc)
}

pub fn handle_hash(input: &std::path::Path) -> OperationResult<()> {
    use sha2::{Digest, Sha256};
    use std::fs;

    let data = fs::read(input).map_err(paperpilot_core::error::PdfError::IoError)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let hash_hex = hex::encode(result);

    println!("{}", hash_hex);
    Ok(())
}

pub fn handle_verify(input: &std::path::Path, expected_hash: &str) -> OperationResult<()> {
    use sha2::{Digest, Sha256};
    use std::fs;

    let data = fs::read(input).map_err(paperpilot_core::error::PdfError::IoError)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let computed_hash = hex::encode(result);

    if computed_hash.eq_ignore_ascii_case(expected_hash) {
        Ok(())
    } else {
        Err(paperpilot_core::error::PdfError::Other(format!(
            "Hash mismatch: expected {}, but found {}",
            expected_hash, computed_hash
        )))
    }
}
