use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::PdfOperation;
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::validate::ValidateOperation;
use std::path::Path;

pub fn execute_validate(path_str: &str) -> OperationResult<()> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(PdfError::IoError(std::io::Error::other(format!(
            "File does not exist: {}",
            path_str
        ))));
    }

    let mut doc = LopdfDocument::load(path)?;
    let op = ValidateOperation::new();
    op.execute(&mut doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf;
    use tempfile::tempdir;

    #[test]
    fn test_execute_validate_file_not_found() {
        let result = execute_validate("nonexistent_file.pdf");
        assert!(result.is_err());
        match result.unwrap_err() {
            PdfError::IoError(e) => {
                assert!(e.to_string().contains("File does not exist"));
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_execute_validate_valid_pdf() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_valid.pdf");

        // Create a simple valid PDF to test with
        let mut inner = lopdf::Document::with_version("1.5");
        let pages_id = inner.new_object_id();
        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        let page_id = inner.add_object(page_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );
        pages_dict.set("Count", lopdf::Object::Integer(1));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));
        inner.save(&path).unwrap();

        let result = execute_validate(path.to_str().unwrap());
        assert!(result.is_ok());
    }
}
