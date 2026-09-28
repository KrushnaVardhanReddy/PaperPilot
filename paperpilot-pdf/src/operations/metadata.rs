use crate::document::LopdfDocument;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

#[derive(Default)]
pub struct MetadataOperation {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
}

impl MetadataOperation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PdfOperation for MetadataOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let info_id = if let Ok(info_ref) = lopdf_doc.inner.trailer.get(b"Info") {
            if let Ok(info_id) = info_ref.as_reference() {
                info_id
            } else {
                let info_dict = lopdf::Dictionary::new();
                let new_id = lopdf_doc.inner.add_object(info_dict);
                lopdf_doc
                    .inner
                    .trailer
                    .set("Info", lopdf::Object::Reference(new_id));
                new_id
            }
        } else {
            let info_dict = lopdf::Dictionary::new();
            let new_id = lopdf_doc.inner.add_object(info_dict);
            lopdf_doc
                .inner
                .trailer
                .set("Info", lopdf::Object::Reference(new_id));
            new_id
        };

        if let Ok(lopdf::Object::Dictionary(info_dict)) = lopdf_doc.inner.get_object_mut(info_id) {
            if let Some(title) = &self.title {
                info_dict.set(
                    "Title",
                    lopdf::Object::String(title.as_bytes().to_vec(), lopdf::StringFormat::Literal),
                );
            }
            if let Some(author) = &self.author {
                info_dict.set(
                    "Author",
                    lopdf::Object::String(author.as_bytes().to_vec(), lopdf::StringFormat::Literal),
                );
            }
            if let Some(subject) = &self.subject {
                info_dict.set(
                    "Subject",
                    lopdf::Object::String(
                        subject.as_bytes().to_vec(),
                        lopdf::StringFormat::Literal,
                    ),
                );
            }
            if let Some(keywords) = &self.keywords {
                info_dict.set(
                    "Keywords",
                    lopdf::Object::String(
                        keywords.as_bytes().to_vec(),
                        lopdf::StringFormat::Literal,
                    ),
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::Document as LopdfInnerDocument;

    fn create_empty_document() -> LopdfDocument {
        LopdfDocument {
            inner: LopdfInnerDocument::with_version("1.5"),
        }
    }

    #[test]
    fn test_add_metadata_to_empty_document() {
        let mut doc = create_empty_document();
        let mut op = MetadataOperation::new();
        op.title = Some("Test Title".to_string());
        op.author = Some("Test Author".to_string());

        assert!(op.execute(&mut doc).is_ok());

        let info_id = doc
            .inner
            .trailer
            .get(b"Info")
            .unwrap()
            .as_reference()
            .unwrap();
        let info_dict = doc.inner.get_object(info_id).unwrap().as_dict().unwrap();

        let title = info_dict.get(b"Title").unwrap().as_str().unwrap();
        assert_eq!(title, b"Test Title");

        let author = info_dict.get(b"Author").unwrap().as_str().unwrap();
        assert_eq!(author, b"Test Author");
    }

    #[test]
    fn test_update_existing_metadata() {
        let mut doc = create_empty_document();

        let mut initial_info = lopdf::Dictionary::new();
        initial_info.set(
            "Title",
            lopdf::Object::String(b"Old Title".to_vec(), lopdf::StringFormat::Literal),
        );
        let info_id = doc.inner.add_object(initial_info);
        doc.inner
            .trailer
            .set("Info", lopdf::Object::Reference(info_id));

        let mut op = MetadataOperation::new();
        op.title = Some("New Title".to_string());
        op.subject = Some("Test Subject".to_string());
        op.keywords = Some("Key1, Key2".to_string());

        assert!(op.execute(&mut doc).is_ok());

        let updated_info_id = doc
            .inner
            .trailer
            .get(b"Info")
            .unwrap()
            .as_reference()
            .unwrap();
        assert_eq!(updated_info_id, info_id);

        let info_dict = doc
            .inner
            .get_object(updated_info_id)
            .unwrap()
            .as_dict()
            .unwrap();

        let title = info_dict.get(b"Title").unwrap().as_str().unwrap();
        assert_eq!(title, b"New Title");

        let subject = info_dict.get(b"Subject").unwrap().as_str().unwrap();
        assert_eq!(subject, b"Test Subject");

        let keywords = info_dict.get(b"Keywords").unwrap().as_str().unwrap();
        assert_eq!(keywords, b"Key1, Key2");

        assert!(info_dict.get(b"Author").is_err());
    }
}
