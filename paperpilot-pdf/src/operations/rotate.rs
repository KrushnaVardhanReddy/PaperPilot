use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct RotatePagesOperation {
    degrees: u16,
}

impl RotatePagesOperation {
    pub fn new(degrees: u16) -> Self {
        Self { degrees }
    }
}

impl PdfOperation for RotatePagesOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        if !self.degrees.is_multiple_of(90) {
            return Err(PdfError::UnsupportedOperation(
                "Rotation must be a multiple of 90 degrees".to_string(),
            ));
        }

        let mut pages_to_update = Vec::new();
        for (_page_number, object_id) in lopdf_doc.inner.get_pages() {
            pages_to_update.push(object_id);
        }

        for object_id in pages_to_update {
            if let Ok(lopdf::Object::Dictionary(dict)) = lopdf_doc.inner.get_object_mut(object_id) {
                let current_rotation = match dict.get(b"Rotate") {
                    Ok(lopdf::Object::Integer(r)) => *r,
                    _ => 0,
                };

                let new_rotation = (current_rotation + self.degrees as i64) % 360;
                dict.set("Rotate", lopdf::Object::Integer(new_rotation));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    fn create_test_document() -> LopdfDocument {
        let mut inner = LopdfInnerDocument::with_version("1.5");
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
        inner.set_object(pages_id, pages_dict); // Replaced insert_object with set_object

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
    fn test_rotate_90() {
        let mut doc = create_test_document();
        let op = RotatePagesOperation::new(90);

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        let rotate = dict.get(b"Rotate").unwrap().as_i64().unwrap();
        assert_eq!(rotate, 90);
    }

    #[test]
    fn test_rotate_invalid_angle() {
        let mut doc = create_test_document();
        let op = RotatePagesOperation::new(45);

        let result = op.execute(&mut doc);
        assert!(result.is_err());

        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert!(msg.contains("multiple of 90"));
        } else {
            panic!("Expected UnsupportedOperation");
        }
    }
}
