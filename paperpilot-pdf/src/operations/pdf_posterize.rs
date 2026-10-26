use lopdf::{Document, Object, Stream};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPosterizeOptions {
    #[serde(default = "default_target_sheet")]
    pub target_sheet: String,
    #[serde(default)]
    pub grid_cols: Option<u32>,
    #[serde(default)]
    pub grid_rows: Option<u32>,
    #[serde(default = "default_overlap_pt")]
    pub overlap_pt: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default = "default_true")]
    pub include_guides: bool,
}

fn default_target_sheet() -> String { "a4".to_string() }
fn default_overlap_pt() -> f32 { 14.17 }
fn default_scale() -> f32 { 1.0 }
fn default_true() -> bool { true }

impl Default for PdfPosterizeOptions {
    fn default() -> Self {
        Self {
            target_sheet: default_target_sheet(),
            grid_cols: None,
            grid_rows: None,
            overlap_pt: default_overlap_pt(),
            scale: default_scale(),
            include_guides: default_true(),
        }
    }
}

pub fn get_sheet_dimensions(sheet_name: &str) -> (f32, f32) {
    match sheet_name.to_lowercase().as_str() {
        "letter" => (612.0, 792.0),
        "legal" => (612.0, 1008.0),
        "a3" => (841.89, 1190.55),
        "a5" => (419.53, 595.28),
        _ => (595.28, 841.89), // Default to A4
    }
}

pub fn pdf_posterize_bytes(
    input_bytes: &[u8],
    options: &PdfPosterizeOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc = Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;
    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err(PdfError::InvalidInput("Document contains no pages".into()));
    }
    let overlap = options.overlap_pt.max(0.0);
    let scale = options.scale.max(0.01);
    let (sheet_w, sheet_h) = get_sheet_dimensions(&options.target_sheet);
    let catalog = doc.catalog().map_err(|e| PdfError::ParseError(e.to_string()))?;
    let pages_id = catalog.get(b"Pages").and_then(Object::as_reference).unwrap_or((0, 0));
    let mut new_pages = Vec::new();
    let mut pages_data = Vec::new();
    for (page_num, object_id) in pages.into_iter() {
        let page = doc.get_object(object_id).and_then(Object::as_dict).map_err(|_| PdfError::ParseError(format!("Page {} is not a dictionary", page_num)))?.clone();
        let media_box = if let Ok(Object::Array(arr)) = page.get(b"MediaBox").or_else(|_| page.get(b"CropBox")) {
            if arr.len() == 4 {
                let x0 = arr[0].as_f32().or_else(|_| arr[0].as_i64().map(|v| v as f32)).unwrap_or(0.0);
                let y0 = arr[1].as_f32().or_else(|_| arr[1].as_i64().map(|v| v as f32)).unwrap_or(0.0);
                let x1 = arr[2].as_f32().or_else(|_| arr[2].as_i64().map(|v| v as f32)).unwrap_or(595.28);
                let y1 = arr[3].as_f32().or_else(|_| arr[3].as_i64().map(|v| v as f32)).unwrap_or(841.89);
                (x0, y0, x1, y1)
            } else { (0.0, 0.0, 595.28, 841.89) }
        } else { (0.0, 0.0, 595.28, 841.89) };
        pages_data.push((page_num, page, media_box));
    }
    for (_page_num, page_dict, (x0, y0, x1, y1)) in pages_data {
        let orig_w = (x1 - x0).abs();
        let orig_h = (y1 - y0).abs();
        let scaled_w = orig_w * scale;
        let scaled_h = orig_h * scale;
        let cols = options.grid_cols.unwrap_or_else(|| {
            if sheet_w <= overlap { return 1; }
            let eff_w = sheet_w - overlap;
            ((scaled_w - overlap) / eff_w).ceil().max(1.0) as u32
        });
        let rows = options.grid_rows.unwrap_or_else(|| {
            if sheet_h <= overlap { return 1; }
            let eff_h = sheet_h - overlap;
            ((scaled_h - overlap) / eff_h).ceil().max(1.0) as u32
        });
        let tile_w = sheet_w;
        let tile_h = sheet_h;
        for r in 0..rows {
            for c in 0..cols {
                let mut tile_page = page_dict.clone();
                let crop_box = vec![Object::Real(0.0), Object::Real(0.0), Object::Real(tile_w), Object::Real(tile_h)];
                tile_page.set("MediaBox", Object::Array(crop_box.clone()));
                tile_page.set("CropBox", Object::Array(crop_box));
                tile_page.set("Parent", Object::Reference(pages_id));
                let mut tx = - (c as f32 * (tile_w - overlap));
                let mut ty = - (((rows - 1 - r) as f32) * (tile_h - overlap));
                tx -= x0 * scale;
                ty -= y0 * scale;
                let matrix_str = format!("{} 0 0 {} {} {} cm\n", scale, scale, tx, ty);
                let mut new_contents = Vec::new();
                new_contents.extend_from_slice(b"q\n");
                new_contents.extend_from_slice(matrix_str.as_bytes());
                let mut old_content_stream = Vec::new();
                if let Ok(contents) = tile_page.get(b"Contents") {
                    match contents {
                        Object::Reference(id) => {
                            if let Ok(stream) = doc.get_object(*id).and_then(Object::as_stream) {
                                if let Ok(data) = stream.decompressed_content() {
                                    old_content_stream.extend_from_slice(&data);
                                }
                            }
                        }
                        Object::Array(arr) => {
                            for obj in arr {
                                if let Ok(id) = obj.as_reference() {
                                    if let Ok(stream) = doc.get_object(id).and_then(Object::as_stream) {
                                        if let Ok(data) = stream.decompressed_content() {
                                            old_content_stream.extend_from_slice(&data);
                                            old_content_stream.push(b'\n');
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                new_contents.extend_from_slice(&old_content_stream);
                new_contents.extend_from_slice(b"\nQ\n");
                if options.include_guides {
                    let mark_len = 10.0;
                    let m = 5.0;
                    let guides_str = format!("q\n0 0 0 RG 0.5 w\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\n{} {} m {} {} l\nS\nQ\n", m, m, m+mark_len, m, m, m, m, m+mark_len, tile_w-m, m, tile_w-m-mark_len, m, tile_w-m, m, tile_w-m, m+mark_len, m, tile_h-m, m+mark_len, tile_h-m, m, tile_h-m, m, tile_h-m-mark_len, tile_w-m, tile_h-m, tile_w-m-mark_len, tile_h-m, tile_w-m, tile_h-m, tile_w-m, tile_h-m-mark_len);
                    new_contents.extend_from_slice(guides_str.as_bytes());
                }
                let stream_dict = lopdf::Dictionary::new();
                let new_stream = Stream::new(stream_dict, new_contents).with_compression(false);
                let stream_id = doc.add_object(Object::Stream(new_stream));
                tile_page.set("Contents", Object::Reference(stream_id));
                let tile_id = doc.add_object(tile_page);
                new_pages.push(tile_id);
            }
        }
    }
    if let Ok(catalog) = doc.catalog() {
        if let Ok(pages_id) = catalog.get(b"Pages").and_then(Object::as_reference) {
            if let Ok(pages_dict) = doc.get_object_mut(pages_id).and_then(Object::as_dict_mut) {
                let kids = new_pages.iter().map(|id| Object::Reference(*id)).collect::<Vec<_>>();
                pages_dict.set("Kids", Object::Array(kids));
                pages_dict.set("Count", Object::Integer(new_pages.len() as i64));
            }
        }
    }
    let mut output_bytes = Vec::new();
    doc.save_to(&mut output_bytes).map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(output_bytes)
}

#[derive(Default)]
pub struct PdfPosterizeOperation {
    _options: PdfPosterizeOptions,
}
impl PdfPosterizeOperation {
    pub fn new(options: PdfPosterizeOptions) -> Self { Self { _options: options } }
}
impl PdfOperation for PdfPosterizeOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> { Err(PdfError::UnsupportedOperation("Execute via pdf_posterize_bytes".to_string())) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pdf_posterize_empty_document() {
        let options = PdfPosterizeOptions::default();
        let bytes = vec![];
        let result = pdf_posterize_bytes(&bytes, &options);
        assert!(result.is_err());
    }
    #[test]
    fn test_pdf_posterize_grid_auto_calc() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("MediaBox", Object::Array(vec![Object::Integer(0), Object::Integer(0), Object::Real(1200.0), Object::Real(1700.0)]));
        let stream = Stream::new(lopdf::Dictionary::new(), b"10 10 m 100 100 l S\n".to_vec());
        let stream_id = doc.add_object(Object::Stream(stream));
        page_dict.set("Contents", Object::Reference(stream_id));
        let page_id = doc.add_object(page_dict);
        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
        pages_dict.set("Count", Object::Integer(1));
        doc.set_object(pages_id, Object::Dictionary(pages_dict));
        let mut catalog = lopdf::Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));
        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();
        let options = PdfPosterizeOptions { target_sheet: "a4".into(), grid_cols: None, grid_rows: None, overlap_pt: 15.0, scale: 1.0, include_guides: true };
        let result = pdf_posterize_bytes(&buf, &options).expect("Posterize should succeed");
        let out_doc = Document::load_mem(&result).expect("Output should be valid PDF");
        let out_pages = out_doc.get_pages();
        assert_eq!(out_pages.len(), 9, "Should auto-calculate 3x3 grid for this size");
    }
}
