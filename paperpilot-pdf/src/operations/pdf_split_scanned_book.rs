use lopdf::{Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfSplitScannedBookOptions {
    pub reading_direction: Option<String>,
    pub gutter_overlap: Option<f32>,
    pub split_horizontal: Option<bool>,
}

#[allow(clippy::collapsible_if)]
pub fn pdf_split_scanned_book_bytes(
    input_bytes: &[u8],
    options: &PdfSplitScannedBookOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let pages = doc.get_pages();
    let mut new_pages = Vec::new();
    let reading_dir = options.reading_direction.as_deref().unwrap_or("ltr");
    let gutter = options.gutter_overlap.unwrap_or(0.0);
    let split_horiz = options.split_horizontal.unwrap_or(false);

    let catalog = doc.catalog().unwrap();
    let pages_id = catalog
        .get(b"Pages")
        .and_then(Object::as_reference)
        .unwrap_or((0, 0));

    for (page_num, object_id) in pages.into_iter() {
        let page = doc
            .get_object(object_id)
            .and_then(Object::as_dict)
            .map_err(|_| PdfError::ParseError(format!("Page {} is not a dictionary", page_num)))?;

        let media_box = if let Ok(Object::Array(arr)) =
            page.get(b"MediaBox").or_else(|_| page.get(b"CropBox"))
        {
            if arr.len() == 4 {
                let x0 = arr[0]
                    .as_f32()
                    .or_else(|_| arr[0].as_i64().map(|v| v as f32))
                    .unwrap_or(0.0);
                let y0 = arr[1]
                    .as_f32()
                    .or_else(|_| arr[1].as_i64().map(|v| v as f32))
                    .unwrap_or(0.0);
                let x1 = arr[2]
                    .as_f32()
                    .or_else(|_| arr[2].as_i64().map(|v| v as f32))
                    .unwrap_or(0.0);
                let y1 = arr[3]
                    .as_f32()
                    .or_else(|_| arr[3].as_i64().map(|v| v as f32))
                    .unwrap_or(0.0);
                Some((x0, y0, x1, y1))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((x0, y0, x1, y1)) = media_box {
            let mut p1_box = vec![];
            let mut p2_box = vec![];

            if split_horiz {
                let mid_y = (y0 + y1) / 2.0;
                // Bottom half
                p1_box = vec![
                    Object::Real(x0),
                    Object::Real(y0),
                    Object::Real(x1),
                    Object::Real(mid_y + gutter),
                ];
                // Top half
                p2_box = vec![
                    Object::Real(x0),
                    Object::Real(mid_y - gutter),
                    Object::Real(x1),
                    Object::Real(y1),
                ];
            } else {
                let mid_x = (x0 + x1) / 2.0;
                // Left half
                p1_box = vec![
                    Object::Real(x0),
                    Object::Real(y0),
                    Object::Real(mid_x + gutter),
                    Object::Real(y1),
                ];
                // Right half
                p2_box = vec![
                    Object::Real(mid_x - gutter),
                    Object::Real(y0),
                    Object::Real(x1),
                    Object::Real(y1),
                ];
            }

            let mut page1 = page.clone();
            let mut page2 = page.clone();

            // Adjust MediaBox and CropBox
            page1.set("MediaBox", Object::Array(p1_box.clone()));
            page1.set("CropBox", Object::Array(p1_box));
            page1.set("Parent", Object::Reference(pages_id));

            page2.set("MediaBox", Object::Array(p2_box.clone()));
            page2.set("CropBox", Object::Array(p2_box));
            page2.set("Parent", Object::Reference(pages_id));

            let p1_id = doc.add_object(page1);
            let p2_id = doc.add_object(page2);

            if split_horiz {
                if reading_dir == "rtl" {
                    new_pages.push(p1_id);
                    new_pages.push(p2_id);
                } else {
                    new_pages.push(p2_id);
                    new_pages.push(p1_id);
                }
            } else {
                if reading_dir == "rtl" {
                    new_pages.push(p2_id);
                    new_pages.push(p1_id);
                } else {
                    new_pages.push(p1_id);
                    new_pages.push(p2_id);
                }
            }
        }
    }

    // Update the pages tree
    if let Ok(catalog) = doc.catalog() {
        if let Ok(pages_id) = catalog.get(b"Pages").and_then(Object::as_reference) {
            if let Ok(pages_dict) = doc.get_object_mut(pages_id).and_then(Object::as_dict_mut) {
                let kids = new_pages
                    .iter()
                    .map(|id| Object::Reference(*id))
                    .collect::<Vec<_>>();
                pages_dict.set("Kids", Object::Array(kids));
                pages_dict.set("Count", Object::Integer(new_pages.len() as i64));
            }
        }
    }

    let mut out_buf = Vec::new();
    doc.save_to(&mut out_buf)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    Ok(out_buf)
}

#[derive(Default)]
pub struct PdfSplitScannedBookOperation {
    _options: PdfSplitScannedBookOptions,
}

impl PdfSplitScannedBookOperation {
    pub fn new(options: PdfSplitScannedBookOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfSplitScannedBookOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_split_scanned_book_empty_document() {
        let options = PdfSplitScannedBookOptions::default();
        let bytes = vec![];
        let result = pdf_split_scanned_book_bytes(&bytes, &options);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use lopdf::{Document, dictionary};

    fn create_test_doc() -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = lopdf::content::Content { operations: vec![] };
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content.encode().unwrap(),
        ));

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.0.into(), 0.0.into(), 1000.0.into(), 500.0.into()], // landscape
        });

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut out = Vec::new();
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn test_split_landscape_spread_ltr() {
        let input = create_test_doc();
        let options = PdfSplitScannedBookOptions {
            reading_direction: Some("ltr".to_string()),
            gutter_overlap: Some(0.0),
            split_horizontal: Some(false),
        };

        let output = pdf_split_scanned_book_bytes(&input, &options).unwrap();
        let doc = Document::load_mem(&output).unwrap();
        let pages = doc.get_pages();
        assert_eq!(pages.len(), 2);

        let p1 = doc.get_object(pages[&1]).unwrap().as_dict().unwrap();
        let m1 = p1.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m1[0]
                .as_f32()
                .or_else(|_| m1[0].as_i64().map(|v| v as f32))
                .unwrap(),
            0.0
        );
        assert_eq!(
            m1[2]
                .as_f32()
                .or_else(|_| m1[2].as_i64().map(|v| v as f32))
                .unwrap(),
            500.0
        );

        let p2 = doc.get_object(pages[&2]).unwrap().as_dict().unwrap();
        let m2 = p2.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m2[0]
                .as_f32()
                .or_else(|_| m2[0].as_i64().map(|v| v as f32))
                .unwrap(),
            500.0
        );
        assert_eq!(
            m2[2]
                .as_f32()
                .or_else(|_| m2[2].as_i64().map(|v| v as f32))
                .unwrap(),
            1000.0
        );
    }

    #[test]
    fn test_split_landscape_spread_rtl() {
        let input = create_test_doc();
        let options = PdfSplitScannedBookOptions {
            reading_direction: Some("rtl".to_string()),
            gutter_overlap: Some(0.0),
            split_horizontal: Some(false),
        };

        let output = pdf_split_scanned_book_bytes(&input, &options).unwrap();
        let doc = Document::load_mem(&output).unwrap();
        let pages = doc.get_pages();
        assert_eq!(pages.len(), 2);

        let p1 = doc.get_object(pages[&1]).unwrap().as_dict().unwrap();
        let m1 = p1.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m1[0]
                .as_f32()
                .or_else(|_| m1[0].as_i64().map(|v| v as f32))
                .unwrap(),
            500.0
        );

        let p2 = doc.get_object(pages[&2]).unwrap().as_dict().unwrap();
        let m2 = p2.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m2[0]
                .as_f32()
                .or_else(|_| m2[0].as_i64().map(|v| v as f32))
                .unwrap(),
            0.0
        );
    }

    #[test]
    fn test_split_with_gutter_overlap() {
        let input = create_test_doc();
        let options = PdfSplitScannedBookOptions {
            reading_direction: Some("ltr".to_string()),
            gutter_overlap: Some(10.0),
            split_horizontal: Some(false),
        };

        let output = pdf_split_scanned_book_bytes(&input, &options).unwrap();
        let doc = Document::load_mem(&output).unwrap();
        let pages = doc.get_pages();

        let p1 = doc.get_object(pages[&1]).unwrap().as_dict().unwrap();
        let m1 = p1.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m1[2]
                .as_f32()
                .or_else(|_| m1[2].as_i64().map(|v| v as f32))
                .unwrap(),
            510.0
        );

        let p2 = doc.get_object(pages[&2]).unwrap().as_dict().unwrap();
        let m2 = p2.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m2[0]
                .as_f32()
                .or_else(|_| m2[0].as_i64().map(|v| v as f32))
                .unwrap(),
            490.0
        );
    }

    #[test]
    fn test_split_horizontal() {
        let input = create_test_doc();
        let options = PdfSplitScannedBookOptions {
            reading_direction: Some("ltr".to_string()),
            gutter_overlap: Some(0.0),
            split_horizontal: Some(true),
        };

        let output = pdf_split_scanned_book_bytes(&input, &options).unwrap();
        let doc = Document::load_mem(&output).unwrap();
        let pages = doc.get_pages();

        let p1 = doc.get_object(pages[&1]).unwrap().as_dict().unwrap();
        let m1 = p1.get(b"MediaBox").unwrap().as_array().unwrap();
        // LTR horizontal means top then bottom
        assert_eq!(
            m1[1]
                .as_f32()
                .or_else(|_| m1[1].as_i64().map(|v| v as f32))
                .unwrap(),
            250.0
        ); // bottom Y for top half
        assert_eq!(
            m1[3]
                .as_f32()
                .or_else(|_| m1[3].as_i64().map(|v| v as f32))
                .unwrap(),
            500.0
        ); // top Y for top half

        let p2 = doc.get_object(pages[&2]).unwrap().as_dict().unwrap();
        let m2 = p2.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(
            m2[1]
                .as_f32()
                .or_else(|_| m2[1].as_i64().map(|v| v as f32))
                .unwrap(),
            0.0
        ); // bottom Y for bottom half
        assert_eq!(
            m2[3]
                .as_f32()
                .or_else(|_| m2[3].as_i64().map(|v| v as f32))
                .unwrap(),
            250.0
        ); // top Y for bottom half
    }
}
