use crate::document::LopdfDocument;
use lopdf::dictionary;
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};

pub struct PageNumbersOperation {
    position: String,
    format: String,
}

impl PageNumbersOperation {
    pub fn new(position: String, format: String) -> Self {
        Self { position, format }
    }
}

impl PdfOperation for PageNumbersOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be an LopdfDocument".to_string())
            })?;

        let inner = &mut lopdf_doc.inner;

        let font_id = inner.add_object(dictionary!(
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        ));

        let pages = inner.get_pages();
        let total_pages = pages.len();
        let mut sorted_page_numbers: Vec<u32> = pages.keys().copied().collect();
        sorted_page_numbers.sort_unstable();

        for (page_number, object_id) in sorted_page_numbers
            .into_iter()
            .map(|n| (n, *pages.get(&n).unwrap()))
        {
            let mut width = 595.0;
            let mut height = 842.0;

            if let Ok(lopdf::Object::Dictionary(page_dict)) = inner.get_object(object_id)
                && let Ok(lopdf::Object::Array(rect)) = page_dict.get(b"MediaBox")
                && rect.len() == 4
                && let (Ok(x2), Ok(y2)) = (rect[2].as_f32(), rect[3].as_f32())
            {
                width = x2;
                height = y2;
            }

            if let Ok(lopdf::Object::Dictionary(page_dict)) = inner.get_object_mut(object_id) {
                let mut new_resources = None;
                if let Ok(res) = page_dict.get_mut(b"Resources") {
                    if let lopdf::Object::Dictionary(res_dict) = res {
                        let mut fonts = res_dict
                            .get(b"Font")
                            .and_then(|f| f.as_dict())
                            .cloned()
                            .unwrap_or_else(|_| lopdf::Dictionary::new());
                        fonts.set("F1", lopdf::Object::Reference(font_id));
                        res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                    }
                } else {
                    let mut fonts = lopdf::Dictionary::new();
                    fonts.set("F1", lopdf::Object::Reference(font_id));
                    let mut res_dict = lopdf::Dictionary::new();
                    res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                    new_resources = Some(lopdf::Object::Dictionary(res_dict));
                }
                if let Some(res) = new_resources {
                    page_dict.set("Resources", res);
                }
            }

            let text = self
                .format
                .replace("{n}", &page_number.to_string())
                .replace("{total}", &total_pages.to_string());
            let escaped_text = text.replace("(", "\\(").replace(")", "\\)");

            let margin = 30.0;
            let text_width = text.len() as f32 * 6.0;

            let (x_pos, y_pos) = match self.position.as_str() {
                "bottom-right" => (width - margin - text_width, margin),
                "bottom-center" => (width / 2.0 - (text_width / 2.0), margin),
                "top-right" => (width - margin - text_width, height - margin - 12.0),
                "top-center" => (width / 2.0 - (text_width / 2.0), height - margin - 12.0),
                _ => (width - margin - text_width, margin),
            };

            let content = format!(
                "q\nBT\n/F1 12 Tf\n1 0 0 1 {} {} Tm\n0 g\n({text}) Tj\nET\nQ\n",
                x_pos,
                y_pos,
                text = escaped_text
            );

            let content_stream =
                lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
            let stream_id = inner.add_object(content_stream);

            if let Ok(lopdf::Object::Dictionary(dict)) = inner.get_object_mut(object_id) {
                if let Ok(contents) = dict.get_mut(b"Contents") {
                    match contents {
                        lopdf::Object::Reference(ref_id) => {
                            let old_ref = *ref_id;
                            *contents = lopdf::Object::Array(vec![
                                lopdf::Object::Reference(old_ref),
                                lopdf::Object::Reference(stream_id),
                            ]);
                        }
                        lopdf::Object::Array(arr) => {
                            arr.push(lopdf::Object::Reference(stream_id));
                        }
                        _ => {}
                    }
                } else {
                    dict.set("Contents", lopdf::Object::Reference(stream_id));
                }
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

        let mut page_ids = vec![];
        for _ in 0..3 {
            let mut page_dict = lopdf::Dictionary::new();
            page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", lopdf::Object::Reference(pages_id));
            let page_id = inner.add_object(page_dict);
            page_ids.push(lopdf::Object::Reference(page_id));
        }

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", lopdf::Object::Array(page_ids));
        pages_dict.set("Count", lopdf::Object::Integer(3));
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
    fn test_page_numbers() {
        let mut doc = create_test_document();
        let op = PageNumbersOperation::new(
            "bottom-center".to_string(),
            "Page {n} / {total}".to_string(),
        );

        op.execute(&mut doc).unwrap();

        let page_id = *doc.inner.get_pages().get(&2).unwrap();
        let dict = doc.inner.get_object(page_id).unwrap().as_dict().unwrap();

        assert!(dict.has(b"Resources"));
        let contents = dict.get(b"Contents").unwrap();
        match contents {
            lopdf::Object::Reference(_) => {}
            _ => panic!("Contents is not Reference"),
        }
    }
}
