use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct RotatePagesOperation {
    degrees: u16,
    pages: Option<Vec<u32>>,
}

impl RotatePagesOperation {
    pub fn new(degrees: u16, pages: Option<Vec<u32>>) -> Self {
        Self { degrees, pages }
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

        #[allow(clippy::manual_is_multiple_of)]
        if self.degrees % 90 != 0 {
            return Err(PdfError::UnsupportedOperation(
                "Rotation must be a multiple of 90 degrees".to_string(),
            ));
        }

        let all_pages = lopdf_doc.inner.get_pages();
        let mut pages_to_update = Vec::new();

        if let Some(target_pages) = &self.pages {
            for &page_num in target_pages {
                if let Some(&object_id) = all_pages.get(&page_num) {
                    pages_to_update.push(object_id);
                } else {
                    return Err(PdfError::ParseError(format!("Invalid page index: {}", page_num)));
                }
            }
        } else {
            for (_page_number, object_id) in all_pages {
                pages_to_update.push(object_id);
            }
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
        let op = RotatePagesOperation::new(90, None);

        assert!(op.execute(&mut doc).is_ok());

        let page_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        let rotate = dict.get(b"Rotate").unwrap().as_i64().unwrap();
        assert_eq!(rotate, 90);
    }

    #[test]
    fn test_rotate_specific_pages() {
        let mut inner = LopdfInnerDocument::with_version("1.5");
        let pages_id = inner.new_object_id();

        let mut page1_dict = lopdf::Dictionary::new();
        page1_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page1_dict.set("Parent", lopdf::Object::Reference(pages_id));
        let page1_id = inner.add_object(page1_dict);

        let mut page2_dict = lopdf::Dictionary::new();
        page2_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page2_dict.set("Parent", lopdf::Object::Reference(pages_id));
        let page2_id = inner.add_object(page2_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page1_id), lopdf::Object::Reference(page2_id)]),
        );
        pages_dict.set("Count", lopdf::Object::Integer(2));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner.trailer.set("Root", lopdf::Object::Reference(catalog_id));

        let mut doc = LopdfDocument { inner };

        // Rotate only page 2
        let op = RotatePagesOperation::new(180, Some(vec![2]));
        assert!(op.execute(&mut doc).is_ok());

        let p1_id = *doc.inner.get_pages().get(&1).unwrap();
        let dict1 = doc.inner.get_object(p1_id).unwrap().as_dict().unwrap();
        assert!(dict1.get(b"Rotate").is_err());

        let p2_id = *doc.inner.get_pages().get(&2).unwrap();
        let dict2 = doc.inner.get_object(p2_id).unwrap().as_dict().unwrap();
        let rotate2 = dict2.get(b"Rotate").unwrap().as_i64().unwrap();
        assert_eq!(rotate2, 180);
    }

    #[test]
    fn test_rotate_invalid_angle() {
        let mut doc = create_test_document();
        let op = RotatePagesOperation::new(45, None);

        let result = op.execute(&mut doc);
        assert!(result.is_err());

        if let Err(PdfError::UnsupportedOperation(msg)) = result {
            assert!(msg.contains("multiple of 90"));
        } else {
            panic!("Expected UnsupportedOperation");
        }
    }
}
