use lopdf::{
    Dictionary, Document, Object, Stream,
    content::{Content, Operation as ContentOperation},
};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfStitchCanvasOptions {
    /// "vertical" (default) or "horizontal"
    #[serde(default = "default_direction")]
    pub direction: String,
    /// Margin around outer canvas in points (default: 0.0)
    #[serde(default)]
    pub margin_pt: f32,
    /// Spacing between pages in points (default: 0.0)
    #[serde(default)]
    pub spacing_pt: f32,
}

fn default_direction() -> String {
    "vertical".to_string()
}

impl Default for PdfStitchCanvasOptions {
    fn default() -> Self {
        Self {
            direction: default_direction(),
            margin_pt: 0.0,
            spacing_pt: 0.0,
        }
    }
}

pub fn pdf_stitch_canvas_bytes(
    input_bytes: &[u8],
    options: &PdfStitchCanvasOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::InvalidInput("Empty document".to_string()));
    }

    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let page_ids = doc.get_pages();
    if page_ids.is_empty() {
        return Err(PdfError::InvalidInput("No pages in document".to_string()));
    }

    let mut page_sizes = Vec::new();
    for (_page_num, page_id) in page_ids.iter() {
        let page_dict = doc
            .get_object(*page_id)
            .and_then(Object::as_dict)
            .map_err(|e| PdfError::ParseError(format!("Failed to get page dict: {}", e)))?;
        let mediabox = page_dict
            .get(b"MediaBox")
            .and_then(Object::as_array)
            .map_err(|_| PdfError::ParseError("Missing MediaBox".to_string()))?;

        if mediabox.len() != 4 {
            return Err(PdfError::ParseError("Invalid MediaBox format".to_string()));
        }
        let w = mediabox[2]
            .as_f32()
            .or_else(|_| mediabox[2].as_i64().map(|v| v as f32))
            .unwrap_or(0.0)
            - mediabox[0]
                .as_f32()
                .or_else(|_| mediabox[0].as_i64().map(|v| v as f32))
                .unwrap_or(0.0);
        let h = mediabox[3]
            .as_f32()
            .or_else(|_| mediabox[3].as_i64().map(|v| v as f32))
            .unwrap_or(0.0)
            - mediabox[1]
                .as_f32()
                .or_else(|_| mediabox[1].as_i64().map(|v| v as f32))
                .unwrap_or(0.0);
        page_sizes.push((w, h));
    }

    let n = page_sizes.len() as f32;
    let is_horizontal = options.direction.to_lowercase() == "horizontal";
    let m = options.margin_pt;
    let s = options.spacing_pt;

    let w_canvas;
    let h_canvas;
    let mut positions = Vec::new();

    if is_horizontal {
        w_canvas = page_sizes.iter().map(|(w, _)| *w).sum::<f32>() + 2.0 * m + (n - 1.0) * s;
        h_canvas = page_sizes
            .iter()
            .map(|(_, h)| *h)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
            + 2.0 * m;

        let mut current_x = m;
        for (w_i, h_i) in &page_sizes {
            let y_i = m + (h_canvas - 2.0 * m - h_i) / 2.0;
            positions.push((current_x, y_i));
            current_x += w_i + s;
        }
    } else {
        w_canvas = page_sizes
            .iter()
            .map(|(w, _)| *w)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
            + 2.0 * m;
        h_canvas = page_sizes.iter().map(|(_, h)| *h).sum::<f32>() + 2.0 * m + (n - 1.0) * s;

        let mut current_y = h_canvas - m;
        for (w_i, h_i) in &page_sizes {
            let x_i = m + (w_canvas - 2.0 * m - w_i) / 2.0;
            current_y -= h_i;
            positions.push((x_i, current_y));
            current_y -= s;
        }
    }

    let mut canvas_operations = Vec::new();
    let mut canvas_xobjects = Dictionary::new();
    let mut pages_to_remove = Vec::new();

    for (i, ((_page_num, page_id), &(x_i, y_i))) in
        page_ids.iter().zip(positions.iter()).enumerate()
    {
        let (w_i, h_i) = page_sizes[i];

        let page_content_bytes = doc.get_page_content(*page_id);
        let page_dict = doc
            .get_object(*page_id)
            .and_then(Object::as_dict)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        let resources = page_dict
            .get(b"Resources")
            .cloned()
            .unwrap_or_else(|_| Object::Dictionary(Dictionary::new()));

        let mut xobj_dict = Dictionary::new();
        xobj_dict.set("Type", Object::Name(b"XObject".to_vec()));
        xobj_dict.set("Subtype", Object::Name(b"Form".to_vec()));
        xobj_dict.set("BBox", vec![0.0.into(), 0.0.into(), w_i.into(), h_i.into()]);
        xobj_dict.set("Resources", resources);

        let mut xobj_stream = Stream::new(xobj_dict, page_content_bytes);
        let _ = xobj_stream.compress();
        let xobj_id = doc.add_object(xobj_stream);
        let xobj_name = format!("Fm{}", i);
        canvas_xobjects.set(xobj_name.as_bytes(), Object::Reference(xobj_id));

        canvas_operations.push(ContentOperation::new("q", vec![]));
        canvas_operations.push(ContentOperation::new(
            "cm",
            vec![
                1.0.into(),
                0.0.into(),
                0.0.into(),
                1.0.into(),
                x_i.into(),
                y_i.into(),
            ],
        ));
        canvas_operations.push(ContentOperation::new(
            "Do",
            vec![Object::Name(xobj_name.into_bytes())],
        ));
        canvas_operations.push(ContentOperation::new("Q", vec![]));

        pages_to_remove.push(page_id);
    }

    let canvas_content = Content {
        operations: canvas_operations,
    };
    let content_stream = Stream::new(
        Dictionary::new(),
        canvas_content.encode().unwrap_or_default(),
    );
    let content_id = doc.add_object(content_stream);

    let mut resources_dict = Dictionary::new();
    resources_dict.set("XObject", Object::Dictionary(canvas_xobjects));

    let mut new_page = Dictionary::new();
    new_page.set("Type", Object::Name(b"Page".to_vec()));
    new_page.set(
        "MediaBox",
        vec![0.0.into(), 0.0.into(), w_canvas.into(), h_canvas.into()],
    );
    new_page.set("Contents", Object::Reference(content_id));
    new_page.set("Resources", Object::Dictionary(resources_dict));

    // Get old Pages dict to get parent and other attrs
    let catalog = doc
        .catalog()
        .map_err(|e| PdfError::ParseError(e.to_string()))?;
    let old_pages_id = catalog
        .get(b"Pages")
        .and_then(Object::as_reference)
        .map_err(|_| PdfError::ParseError("Pages not found".to_string()))?;

    // We create a new Pages dictionary
    let mut new_pages = Dictionary::new();
    new_pages.set("Type", Object::Name(b"Pages".to_vec()));
    new_pages.set("Count", 1);

    let new_page_id = doc.add_object(new_page);
    new_pages.set("Kids", vec![Object::Reference(new_page_id)]);

    // Set parent of new page to old_pages_id (which will be updated)
    if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(new_page_id) {
        page_dict.set("Parent", Object::Reference(old_pages_id));
    }

    // Update old Pages dict
    if let Ok(Object::Dictionary(pages_dict)) = doc.get_object_mut(old_pages_id) {
        *pages_dict = new_pages;
    }

    doc.prune_objects();

    let mut output_bytes = Vec::new();
    doc.save_to(&mut output_bytes)
        .map_err(|e| PdfError::IoError(e))?;
    Ok(output_bytes)
}

#[derive(Default)]
pub struct PdfStitchCanvasOperation {
    _options: PdfStitchCanvasOptions,
}

impl PdfStitchCanvasOperation {
    pub fn new(options: PdfStitchCanvasOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfStitchCanvasOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Operation based execution is not yet implemented. Use pdf_stitch_canvas_bytes."
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pdf(pages: usize, width: f32, height: f32) -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut kids = Vec::new();

        for _ in 0..pages {
            let mut page = Dictionary::new();
            page.set("Type", Object::Name(b"Page".to_vec()));
            page.set(
                "MediaBox",
                vec![0.0.into(), 0.0.into(), width.into(), height.into()],
            );
            page.set("Parent", Object::Reference(pages_id));
            kids.push(Object::Reference(doc.add_object(page)));
        }

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", pages as i32);
        pages_dict.set("Kids", kids);

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut out = Vec::new();
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn test_pdf_stitch_canvas_empty_document() {
        let options = PdfStitchCanvasOptions::default();
        let bytes = vec![];
        let result = pdf_stitch_canvas_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_vertical_stitch() {
        let input = create_test_pdf(3, 300.0, 400.0);
        let options = PdfStitchCanvasOptions {
            direction: "vertical".to_string(),
            margin_pt: 10.0,
            spacing_pt: 5.0,
        };
        let result = pdf_stitch_canvas_bytes(&input, &options).expect("failed to stitch");
        let doc = Document::load_mem(&result).expect("failed to load output");

        let pages = doc.get_pages();
        assert_eq!(pages.len(), 1);

        let page_id = pages[&1];
        let page = doc.get_object(page_id).unwrap().as_dict().unwrap();
        let mediabox = page.get(b"MediaBox").unwrap().as_array().unwrap();

        let w = mediabox[2]
            .as_f32()
            .or_else(|_| mediabox[2].as_i64().map(|v| v as f32))
            .unwrap()
            - mediabox[0]
                .as_f32()
                .or_else(|_| mediabox[0].as_i64().map(|v| v as f32))
                .unwrap();
        let h = mediabox[3]
            .as_f32()
            .or_else(|_| mediabox[3].as_i64().map(|v| v as f32))
            .unwrap()
            - mediabox[1]
                .as_f32()
                .or_else(|_| mediabox[1].as_i64().map(|v| v as f32))
                .unwrap();

        // Width: max(300) + 2*10 = 320
        assert_eq!(w, 320.0);
        // Height: 3*400 + 2*10 + 2*5 = 1200 + 20 + 10 = 1230
        assert_eq!(h, 1230.0);
    }

    #[test]
    fn test_horizontal_stitch() {
        let input = create_test_pdf(2, 200.0, 500.0);
        let options = PdfStitchCanvasOptions {
            direction: "horizontal".to_string(),
            margin_pt: 20.0,
            spacing_pt: 10.0,
        };
        let result = pdf_stitch_canvas_bytes(&input, &options).expect("failed to stitch");
        let doc = Document::load_mem(&result).expect("failed to load output");

        let pages = doc.get_pages();
        assert_eq!(pages.len(), 1);

        let page_id = pages[&1];
        let page = doc.get_object(page_id).unwrap().as_dict().unwrap();
        let mediabox = page.get(b"MediaBox").unwrap().as_array().unwrap();

        let w = mediabox[2]
            .as_f32()
            .or_else(|_| mediabox[2].as_i64().map(|v| v as f32))
            .unwrap()
            - mediabox[0]
                .as_f32()
                .or_else(|_| mediabox[0].as_i64().map(|v| v as f32))
                .unwrap();
        let h = mediabox[3]
            .as_f32()
            .or_else(|_| mediabox[3].as_i64().map(|v| v as f32))
            .unwrap()
            - mediabox[1]
                .as_f32()
                .or_else(|_| mediabox[1].as_i64().map(|v| v as f32))
                .unwrap();

        // Width: 2*200 + 2*20 + 1*10 = 400 + 40 + 10 = 450
        assert_eq!(w, 450.0);
        // Height: max(500) + 2*20 = 540
        assert_eq!(h, 540.0);
    }
}
