use lopdf::{
    Dictionary, Document, Object, Stream, content::Content, content::Operation as ContentOperation,
};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfNupOptions {
    #[serde(default = "default_nup")]
    pub nup: u32,
    #[serde(default = "default_ordering")]
    pub ordering: String,
    #[serde(default = "default_margin")]
    pub margin_pt: f32,
    #[serde(default = "default_spacing")]
    pub spacing_pt: f32,
    #[serde(default)]
    pub draw_borders: bool,
}

fn default_nup() -> u32 {
    2
}
fn default_ordering() -> String {
    "horizontal".to_string()
}
fn default_margin() -> f32 {
    18.0
}
fn default_spacing() -> f32 {
    10.0
}

impl Default for PdfNupOptions {
    fn default() -> Self {
        Self {
            nup: 2,
            ordering: default_ordering(),
            margin_pt: 18.0,
            spacing_pt: 10.0,
            draw_borders: false,
        }
    }
}

pub fn pdf_nup_bytes(input_bytes: &[u8], options: &PdfNupOptions) -> Result<Vec<u8>, PdfError> {
    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let page_ids = doc.get_pages();
    if page_ids.is_empty() {
        return Err(PdfError::InvalidInput("No pages in document".into()));
    }

    // Default page dimensions (A4 in points) if we can't determine it
    let mut page_w = 595.28;
    let mut page_h = 841.89;

    // Try to get dimensions of first page
    if let Some(&first_page_id) = page_ids.values().next()
        && let Ok(Object::Dictionary(dict)) = doc.get_object(first_page_id)
        && let Ok(Object::Array(rect)) = dict.get(b"MediaBox")
        && rect.len() >= 4
        && let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (
            rect[0]
                .as_f32()
                .map(|v| v as f64)
                .or_else(|_| rect[0].as_i64().map(|v| v as f64)),
            rect[1]
                .as_f32()
                .map(|v| v as f64)
                .or_else(|_| rect[1].as_i64().map(|v| v as f64)),
            rect[2]
                .as_f32()
                .map(|v| v as f64)
                .or_else(|_| rect[2].as_i64().map(|v| v as f64)),
            rect[3]
                .as_f32()
                .map(|v| v as f64)
                .or_else(|_| rect[3].as_i64().map(|v| v as f64)),
        )
    {
        page_w = (x2 - x1).abs() as f32;
        page_h = (y2 - y1).abs() as f32;
    }

    let (rows, cols) = match options.nup {
        2 => {
            // For 2-up, we typically rotate the sheet to landscape and put pages side-by-side
            std::mem::swap(&mut page_w, &mut page_h);
            (1, 2)
        }
        4 => (2, 2),
        9 => (3, 3),
        16 => (4, 4),
        _ => (1, 2), // Default to 2
    };

    let chunk_size = (rows * cols) as usize;
    let mut page_chunks_vec = page_ids.into_iter().collect::<Vec<_>>();
    page_chunks_vec.sort_by_key(|k| k.0);
    let page_chunks: Vec<_> = page_chunks_vec.chunks(chunk_size).collect();

    let mut new_page_ids = Vec::new();

    // Compute cell dimensions
    let margin = options.margin_pt;
    let spacing = options.spacing_pt;

    let total_w = page_w - (2.0 * margin);
    let total_h = page_h - (2.0 * margin);

    let total_spacing_w = spacing * ((cols - 1) as f32).max(0.0);
    let total_spacing_h = spacing * ((rows - 1) as f32).max(0.0);

    let cell_w = (total_w - total_spacing_w) / cols as f32;
    let cell_h = (total_h - total_spacing_h) / rows as f32;

    for chunk in page_chunks {
        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set(
            "MediaBox",
            vec![0.into(), 0.into(), page_w.into(), page_h.into()],
        );

        let mut operations = Vec::new();
        let mut xobjects_dict = Dictionary::new();

        for (i, &(_page_num, page_id)) in chunk.iter().enumerate() {
            let row = i as u32 / cols;
            let col = i as u32 % cols;

            let (draw_row, draw_col) = if options.ordering == "vertical" {
                (col, row)
            } else {
                (row, col)
            };

            // Get original page dimensions
            let mut orig_w = 595.28;
            let mut orig_h = 841.89;

            if let Ok(Object::Dictionary(dict)) = doc.get_object(page_id)
                && let Ok(Object::Array(rect)) = dict.get(b"MediaBox")
                && rect.len() >= 4
                && let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (
                    rect[0]
                        .as_f32()
                        .map(|v| v as f64)
                        .or_else(|_| rect[0].as_i64().map(|v| v as f64)),
                    rect[1]
                        .as_f32()
                        .map(|v| v as f64)
                        .or_else(|_| rect[1].as_i64().map(|v| v as f64)),
                    rect[2]
                        .as_f32()
                        .map(|v| v as f64)
                        .or_else(|_| rect[2].as_i64().map(|v| v as f64)),
                    rect[3]
                        .as_f32()
                        .map(|v| v as f64)
                        .or_else(|_| rect[3].as_i64().map(|v| v as f64)),
                )
            {
                orig_w = (x2 - x1).abs() as f32;
                orig_h = (y2 - y1).abs() as f32;
            }

            let scale_x = cell_w / orig_w;
            let scale_y = cell_h / orig_h;
            let scale = scale_x.min(scale_y);

            let actual_w = orig_w * scale;
            let actual_h = orig_h * scale;

            let dx = margin + (draw_col as f32 * (cell_w + spacing)) + (cell_w - actual_w) / 2.0;
            let dy =
                page_h - margin - ((draw_row + 1) as f32 * cell_h) - (draw_row as f32 * spacing)
                    + (cell_h - actual_h) / 2.0;

            // Create Form XObject
            let mut xobject_stream = Stream::new(Dictionary::new(), Vec::new());
            xobject_stream
                .dict
                .set("Type", Object::Name(b"XObject".to_vec()));
            xobject_stream
                .dict
                .set("Subtype", Object::Name(b"Form".to_vec()));
            xobject_stream.dict.set(
                "BBox",
                vec![0.into(), 0.into(), orig_w.into(), orig_h.into()],
            );

            if let Ok(Object::Dictionary(orig_page_dict)) = doc.get_object(page_id) {
                if let Ok(resources) = orig_page_dict.get(b"Resources") {
                    xobject_stream.dict.set("Resources", resources.clone());
                }

                if let Ok(contents) = orig_page_dict.get(b"Contents") {
                    match contents {
                        Object::Reference(ref_id) => {
                            if let Ok(Object::Stream(stream)) = doc.get_object(*ref_id) {
                                let mut s = stream.clone();
                                let _ = s.decompress();
                                xobject_stream.content = s.content;
                            }
                        }
                        Object::Array(arr) => {
                            for item in arr {
                                if let Object::Reference(ref_id) = item
                                    && let Ok(Object::Stream(stream)) = doc.get_object(*ref_id)
                                {
                                    let mut s = stream.clone();
                                    let _ = s.decompress();
                                    xobject_stream.content.extend_from_slice(&s.content);
                                    xobject_stream.content.push(b'\n');
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            let xobject_id = doc.add_object(Object::Stream(xobject_stream));
            let xobject_name = format!("Fm{}", i);
            xobjects_dict.set(xobject_name.clone(), Object::Reference(xobject_id));

            operations.push(ContentOperation::new("q", vec![]));
            operations.push(ContentOperation::new(
                "cm",
                vec![
                    scale.into(),
                    0.0.into(),
                    0.0.into(),
                    scale.into(),
                    dx.into(),
                    dy.into(),
                ],
            ));
            operations.push(ContentOperation::new(
                "Do",
                vec![Object::Name(xobject_name.as_bytes().to_vec())],
            ));
            operations.push(ContentOperation::new("Q", vec![]));

            if options.draw_borders {
                operations.push(ContentOperation::new("q", vec![]));
                operations.push(ContentOperation::new("0", vec![])); // Black color
                operations.push(ContentOperation::new("0.5", vec![])); // Line width
                operations.push(ContentOperation::new(
                    "re",
                    vec![dx.into(), dy.into(), actual_w.into(), actual_h.into()],
                ));
                operations.push(ContentOperation::new("S", vec![])); // Stroke
                operations.push(ContentOperation::new("Q", vec![]));
            }
        }

        let content = Content { operations };
        let content_stream = Stream::new(Dictionary::new(), content.encode().unwrap());
        let content_id = doc.add_object(Object::Stream(content_stream));
        page_dict.set("Contents", Object::Reference(content_id));

        let mut resources_dict = Dictionary::new();
        resources_dict.set("XObject", Object::Dictionary(xobjects_dict));
        page_dict.set("Resources", Object::Dictionary(resources_dict));

        let new_page_id = doc.add_object(Object::Dictionary(page_dict));
        new_page_ids.push(new_page_id);
    }

    // Replace pages tree
    let pages_id = doc
        .catalog()
        .and_then(|c| c.get(b"Pages"))
        .and_then(|p| p.as_reference())
        .map_err(|_| PdfError::ParseError("No Pages tree".into()))?;
    if let Ok(Object::Dictionary(pages_dict)) = doc.get_object_mut(pages_id) {
        pages_dict.set(
            "Kids",
            Object::Array(
                new_page_ids
                    .iter()
                    .map(|id| Object::Reference(*id))
                    .collect(),
            ),
        );
        pages_dict.set("Count", Object::Integer(new_page_ids.len() as i64));
    }

    // Set parent on new pages
    for &id in &new_page_ids {
        if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(id) {
            page_dict.set("Parent", Object::Reference(pages_id));
        }
    }

    // Optionally remove old pages (lopdf will write them but unreferenced, they take space)
    doc.prune_objects();

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    Ok(buf)
}

#[derive(Default)]
pub struct PdfNupOperation {
    _options: PdfNupOptions,
}

impl PdfNupOperation {
    pub fn new(options: PdfNupOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfNupOperation {
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
    use lopdf::{
        Dictionary, Document, Object, Stream, content::Content, content::Operation, dictionary,
    };

    fn create_test_pdf(num_pages: u32) -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });

        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id },
        });

        for _ in 0..num_pages {
            let content = Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), 24.into()]),
                    Operation::new("Td", vec![100.into(), 100.into()]),
                    Operation::new(
                        "Tj",
                        vec![Object::String(
                            b"Test Page".to_vec(),
                            lopdf::StringFormat::Literal,
                        )],
                    ),
                    Operation::new("ET", vec![]),
                ],
            };

            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
            let page_id = doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
                "Resources" => resources_id,
                "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            });
            page_ids.push(Object::Reference(page_id));
        }

        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Count" => num_pages,
            "Kids" => page_ids,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        doc.trailer.set("Root", catalog_id);

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();
        buf
    }

    #[test]
    fn test_nup_empty_input() {
        let options = PdfNupOptions::default();
        let bytes = vec![];
        let result = pdf_nup_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_nup_2_up() {
        let input = create_test_pdf(4);
        let options = PdfNupOptions {
            nup: 2,
            ..Default::default()
        };
        let output = pdf_nup_bytes(&input, &options).expect("Should succeed");

        let doc = Document::load_mem(&output).unwrap();
        assert_eq!(
            doc.get_pages().len(),
            2,
            "4 pages at 2-up should be 2 output pages"
        );
    }

    #[test]
    fn test_nup_4_up() {
        let input = create_test_pdf(4);
        let options = PdfNupOptions {
            nup: 4,
            ..Default::default()
        };
        let output = pdf_nup_bytes(&input, &options).expect("Should succeed");

        let doc = Document::load_mem(&output).unwrap();
        assert_eq!(
            doc.get_pages().len(),
            1,
            "4 pages at 4-up should be 1 output page"
        );
    }
}
