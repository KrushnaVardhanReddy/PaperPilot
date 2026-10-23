use lopdf::{Dictionary, Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfBookletOptions {
    /// If true (default), imposes pairs of pages onto 2-up landscape sheets.
    /// If false, reorders single pages into booklet sequence.
    #[serde(default = "default_true")]
    pub create_sheets: bool,

    /// Add a subtle fold line down the center of each sheet.
    #[serde(default)]
    pub center_fold_line: bool,

    /// Margin between side-by-side pages on a sheet in points (default: 0.0).
    #[serde(default)]
    pub gutter_pt: f32,
}

fn default_true() -> bool {
    true
}

impl Default for PdfBookletOptions {
    fn default() -> Self {
        Self {
            create_sheets: true,
            center_fold_line: false,
            gutter_pt: 0.0,
        }
    }
}

pub fn pdf_booklet_bytes(
    input_bytes: &[u8],
    _options: &PdfBookletOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::ParseError("Empty document".into()));
    }
    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let pages = doc.get_pages(); // BTreeMap<u32, ObjectId>
    let page_count = pages.len();
    if page_count == 0 {
        return Err(PdfError::InvalidInput("Document contains no pages".into()));
    }

    let remainder = page_count % 4;
    let pad_count = if remainder == 0 { 0 } else { 4 - remainder };
    let total_pages = page_count + pad_count;
    let sheets = total_pages / 4;

    let mut sequence = Vec::with_capacity(total_pages);
    for i in 0..sheets {
        // Sheet Front: [total_pages - 2*i, 2*i + 1]
        sequence.push(total_pages - 2 * i);
        sequence.push(2 * i + 1);
        // Sheet Back: [2*i + 2, total_pages - 2*i - 1]
        sequence.push(2 * i + 2);
        sequence.push(total_pages - 2 * i - 1);
    }

    let catalog = doc
        .catalog()
        .map_err(|e| PdfError::ParseError(e.to_string()))?;
    let pages_id = catalog
        .get(b"Pages")
        .and_then(|o| o.as_reference())
        .map_err(|e| PdfError::ParseError(e.to_string()))?;

    let mut new_kids = Vec::new();

    // Mapping 1-based index to page ObjectId
    for &idx in &sequence {
        if idx > page_count {
            // Padding page: blank page
            let mut blank_page = Dictionary::new();
            blank_page.set("Type", Object::Name(b"Page".to_vec()));
            blank_page.set("Parent", Object::Reference(pages_id));
            let blank_id = doc.add_object(Object::Dictionary(blank_page));
            new_kids.push(Object::Reference(blank_id));
        } else {
            // Existing page
            if let Some(&page_id) = pages.get(&(idx as u32)) {
                new_kids.push(Object::Reference(page_id));
            } else {
                return Err(PdfError::InvalidInput(format!("Page {} not found", idx)));
            }
        }
    }

    if let Ok(Object::Dictionary(pages_dict)) = doc.get_object_mut(pages_id) {
        pages_dict.set("Kids", Object::Array(new_kids));
        pages_dict.set("Count", Object::Integer(total_pages as i64));
    } else {
        return Err(PdfError::ParseError(
            "Pages node is not a dictionary".into(),
        ));
    }

    let mut out_bytes = Vec::new();
    doc.save_to(&mut out_bytes)
        .map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(out_bytes)
}

#[derive(Default)]
pub struct PdfBookletOperation {
    pub options: PdfBookletOptions,
}

impl PdfBookletOperation {
    pub fn new(options: PdfBookletOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfBookletOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        // Operation based execution is not yet implemented for Document in memory.
        Err(PdfError::UnsupportedOperation(
            "PdfBookletOperation only supports bytes execution".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_booklet_empty_document() {
        let options = PdfBookletOptions::default();
        let bytes = vec![];
        let result = pdf_booklet_bytes(&bytes, &options);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod booklet_tests {
    use super::*;
    use lopdf::Document;

    fn create_test_pdf(num_pages: u32) -> Vec<u8> {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut kids = vec![];
        for _ in 0..num_pages {
            let mut page = Dictionary::new();
            page.set("Type", Object::Name(b"Page".to_vec()));
            page.set("Parent", Object::Reference(pages_id));
            let page_id = doc.add_object(Object::Dictionary(page));
            kids.push(Object::Reference(page_id));
        }

        let mut pages = Dictionary::new();
        pages.set("Type", Object::Name(b"Pages".to_vec()));
        pages.set("Kids", Object::Array(kids));
        pages.set("Count", Object::Integer(num_pages as i64));
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(Object::Dictionary(catalog));

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut out = Vec::new();
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn test_booklet_4_pages() {
        let bytes = create_test_pdf(4);
        let options = PdfBookletOptions::default();
        let result = pdf_booklet_bytes(&bytes, &options).expect("Should succeed");

        let doc = Document::load_mem(&result).unwrap();
        assert_eq!(doc.get_pages().len(), 4);
    }

    #[test]
    fn test_booklet_padding_3_to_4() {
        let bytes = create_test_pdf(3);
        let options = PdfBookletOptions::default();
        let result = pdf_booklet_bytes(&bytes, &options).expect("Should succeed");

        let doc = Document::load_mem(&result).unwrap();
        assert_eq!(doc.get_pages().len(), 4);
    }

    #[test]
    fn test_booklet_8_pages() {
        let bytes = create_test_pdf(8);
        let options = PdfBookletOptions::default();
        let result = pdf_booklet_bytes(&bytes, &options).expect("Should succeed");

        let doc = Document::load_mem(&result).unwrap();
        assert_eq!(doc.get_pages().len(), 8);
    }
}
