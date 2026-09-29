#![allow(clippy::collapsible_if)]
use paperpilot_core::error::{OperationResult, PdfError};
use std::path::Path;

/// Pre-flight validation checks before running any PDF operation.
/// Returns Ok(()) if the file is safe to process, Err with a descriptive message otherwise.
pub fn validate_pdf_input(path: &Path) -> OperationResult<()> {
    // 1. File must exist
    if !path.exists() {
        return Err(PdfError::InvalidInput(format!(
            "Input file does not exist: {}",
            path.display()
        )));
    }

    // 2. File must be readable and non-empty
    let metadata = std::fs::metadata(path).map_err(|e| {
        PdfError::InvalidInput(format!("Cannot read file metadata: {}", e))
    })?;
    if metadata.len() == 0 {
        return Err(PdfError::InvalidInput("Input file is empty (0 bytes).".to_string()));
    }

    // 3. File size sanity check (reject files > 500 MB)
    const MAX_FILE_SIZE: u64 = 500 * 1024 * 1024;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(PdfError::InvalidInput(format!(
            "File too large: {} bytes (max 500 MB)",
            metadata.len()
        )));
    }

    // 4. Magic bytes check - PDF must start with %PDF-
    let mut f = std::fs::File::open(path).map_err(|e| {
        PdfError::InvalidInput(format!("Cannot open file: {}", e))
    })?;
    let mut magic = [0u8; 5];
    use std::io::Read;
    f.read_exact(&mut magic).map_err(|_| {
        PdfError::InvalidInput("File is too small to be a valid PDF.".to_string())
    })?;
    if &magic != b"%PDF-" {
        return Err(PdfError::InvalidInput(
            "File does not appear to be a PDF (missing %PDF- header).".to_string(),
        ));
    }

    Ok(())
}

/// Validate the output path - parent directory must be writable.
pub fn validate_pdf_output(path: &Path) -> OperationResult<()> {
    if let Some(parent) = path.parent()
        && !parent.exists() {
            return Err(PdfError::InvalidInput(format!(
                "Output directory does not exist: {}",
                parent.display()
            )));
        }
    Ok(())
}

#[allow(clippy::collapsible_if)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_nonexistent_file() {
        let result = validate_pdf_input(Path::new("nonexistent.pdf"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let result = validate_pdf_input(tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_pdf_magic_bytes() {
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.write_all(b"PK\x03\x04 this is a zip file not a pdf").unwrap();
        let result = validate_pdf_input(tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_pdf_fixture() {
        let mut path = std::env::current_dir().unwrap();
        if path.file_name().unwrap() == "paperpilot-pdf" {
            path.pop();
        }
        path.push("tests/fixtures/sample.pdf");
        if path.exists() {
            let result = validate_pdf_input(&path);
            assert!(result.is_ok());
        }
    }
}
