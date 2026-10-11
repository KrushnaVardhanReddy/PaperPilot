use office_oxide::Document;
use paperpilot_core::error::{OperationResult, PdfError};
use std::fs;
use std::path::Path;

pub struct OfficeToMarkdownOperation;

impl OfficeToMarkdownOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let md = office_oxide::to_markdown(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        fs::write(output_path, md).map_err(PdfError::IoError)?;
        Ok(())
    }
}

pub struct OfficeExtractTextOperation;

impl OfficeExtractTextOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let text = office_oxide::extract_text(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        fs::write(output_path, text).map_err(PdfError::IoError)?;
        Ok(())
    }
}

pub struct OfficeModernizeOperation;

impl OfficeModernizeOperation {
    pub fn execute(input_path: &Path, output_path: &Path) -> OperationResult<()> {
        let doc = Document::open(input_path)
            .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        doc.save_as(output_path)
            .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_office_to_markdown_invalid_file() {
        let res = OfficeToMarkdownOperation::execute(
            &PathBuf::from("does_not_exist.docx"),
            &PathBuf::from("out.md"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_office_extract_text_invalid_file() {
        let res = OfficeExtractTextOperation::execute(
            &PathBuf::from("does_not_exist.docx"),
            &PathBuf::from("out.txt"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_office_modernize_invalid_file() {
        let res = OfficeModernizeOperation::execute(
            &PathBuf::from("does_not_exist.doc"),
            &PathBuf::from("out.docx"),
        );
        assert!(res.is_err());
        match res {
            Err(PdfError::IoError(_)) => {}
            _ => panic!("Expected IoError"),
        }
    }
}