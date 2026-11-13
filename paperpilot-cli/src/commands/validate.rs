#![cfg(feature = "pdf")]
use paperpilot_core::error::OperationResult;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;

pub fn handle_validate(input: &std::path::Path, output: Option<std::path::PathBuf>) -> OperationResult<()> {
    let mut doc = LopdfDocument::load(input)?;
    let op = paperpilot_pdf::operations::validate::ValidateOperation::new();
    let res = op.execute(&mut doc);
    let is_valid = res.is_ok();
    let page_count = doc.page_count().unwrap_or(0);

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let report = serde_json::json!({
            "file": input.to_string_lossy(),
            "is_valid": is_valid,
            "page_count": page_count,
            "status": if is_valid { "VALID_PDF_STRUCTURE" } else { "CORRUPTED_OR_INVALID" }
        });
        let _ = std::fs::write(&out_path, serde_json::to_string_pretty(&report).unwrap_or_default());
    }

    if is_valid {
        println!("Document is valid ({} pages)", page_count);
        Ok(())
    } else {
        res
    }
}

pub fn handle_hash(input: &std::path::Path, output: Option<std::path::PathBuf>) -> OperationResult<()> {
    use sha2::{Digest, Sha256};
    use std::fs;

    let data = fs::read(input).map_err(paperpilot_core::error::PdfError::IoError)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let hash_hex = hex::encode(result);

    if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let content = format!(
            "Algorithm: SHA-256\nFile: {}\nHash: {}\nSize: {} bytes\n",
            input.display(), hash_hex, data.len()
        );
        let _ = fs::write(&out_path, content);
    }

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
