use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct PdfAConversionOperation;

impl PdfAConversionOperation {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PdfAConversionOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfOperation for PdfAConversionOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        // Add PDF/A conformance metadata to the Info dictionary
        let mut info_dict = lopdf::Dictionary::new();
        info_dict.set("GTS_PDFXVersion", lopdf::Object::string_literal("PDF/A-1b"));

        let info_id = lopdf_doc
            .inner
            .add_object(lopdf::Object::Dictionary(info_dict));
        lopdf_doc
            .inner
            .trailer
            .set("Info", lopdf::Object::Reference(info_id));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;
    use lopdf::Document as LopdfInnerDocument;

    fn create_test_document() -> LopdfDocument {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));
        pages_dict.set("Count", lopdf::Object::Integer(0));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        LopdfDocument { inner }
    }

    #[test]
    fn test_pdf_a_conversion_operation() {
        let mut doc = create_test_document();
        let op = PdfAConversionOperation::new();

        let result = op.execute(&mut doc);
        assert!(result.is_ok());

        let info_id = doc
            .inner
            .trailer
            .get(b"Info")
            .unwrap()
            .as_reference()
            .unwrap();
        let info_dict = doc.inner.get_object(info_id).unwrap().as_dict().unwrap();

        assert_eq!(
            info_dict.get(b"GTS_PDFXVersion").unwrap().as_str().unwrap(),
            b"PDF/A-1b"
        );
    }
}
