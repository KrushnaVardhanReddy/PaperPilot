use crate::document::LopdfDocument;
use lopdf::content::{Content, Operation};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct RedactOperation {
    page_number: u32,
    bounding_box: BoundingBox,
}

impl RedactOperation {
    pub fn new(page_number: u32, bounding_box: BoundingBox) -> Self {
        Self {
            page_number,
            bounding_box,
        }
    }
}

impl PdfOperation for RedactOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let page_id = *lopdf_doc
            .inner
            .get_pages()
            .get(&self.page_number)
            .ok_or_else(|| PdfError::Other(format!("Page {} not found", self.page_number)))?;

        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new(
                    "rg",
                    vec![
                        lopdf::Object::Real(0.0),
                        lopdf::Object::Real(0.0),
                        lopdf::Object::Real(0.0),
                    ],
                ),
                Operation::new(
                    "re",
                    vec![
                        lopdf::Object::Real(self.bounding_box.x),
                        lopdf::Object::Real(self.bounding_box.y),
                        lopdf::Object::Real(self.bounding_box.width),
                        lopdf::Object::Real(self.bounding_box.height),
                    ],
                ),
                Operation::new("f", vec![]),
                Operation::new("Q", vec![]),
            ],
        };

        lopdf_doc
            .inner
            .add_to_page_content(page_id, content)
            .map_err(|e| PdfError::Other(format!("Failed to add redaction to page: {}", e)))?;

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
    fn test_redact_valid_page() {
        let mut doc = create_test_document();
        let op = RedactOperation::new(
            1,
            BoundingBox {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 50.0,
            },
        );

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let content_bytes = doc.inner.get_page_content(page_id);
        let decoded = Content::decode(&content_bytes).unwrap();

        // We added 5 operations
        assert_eq!(decoded.operations.len(), 5);

        // Verify the operations
        assert_eq!(decoded.operations[0].operator, "q");
        assert_eq!(decoded.operations[1].operator, "rg");
        assert_eq!(decoded.operations[2].operator, "re");
        let get_val = |obj: &lopdf::Object| {
            if let lopdf::Object::Real(r) = obj {
                *r
            } else if let lopdf::Object::Integer(i) = obj {
                *i as f32
            } else {
                0.0
            }
        };

        assert_eq!(get_val(&decoded.operations[2].operands[0]), 10.0);
        assert_eq!(get_val(&decoded.operations[2].operands[1]), 20.0);
        assert_eq!(get_val(&decoded.operations[2].operands[2]), 100.0);
        assert_eq!(get_val(&decoded.operations[2].operands[3]), 50.0);
        assert_eq!(decoded.operations[3].operator, "f");
        assert_eq!(decoded.operations[4].operator, "Q");
    }

    #[test]
    fn test_redact_invalid_page() {
        let mut doc = create_test_document();
        let op = RedactOperation::new(
            99, // Non-existent page
            BoundingBox {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 50.0,
            },
        );

        let result = op.execute(&mut doc);
        assert!(result.is_err());
        if let Err(PdfError::Other(msg)) = result {
            assert!(msg.contains("Page 99 not found"));
        } else {
            panic!("Expected Other error");
        }
    }
}
