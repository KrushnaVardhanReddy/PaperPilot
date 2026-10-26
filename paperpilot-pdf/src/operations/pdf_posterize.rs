use lopdf::{Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPosterizeOptions {
    /// Target paper sheet size: "a4", "letter", "legal", or "custom" (default: "a4")
    #[serde(default = "default_target_sheet")]
    pub target_sheet: String,
    /// Explicit grid columns (default: 2)
    #[serde(default = "default_grid_cols")]
    pub grid_cols: u32,
    /// Explicit grid rows (default: 2)
    #[serde(default = "default_grid_rows")]
    pub grid_rows: u32,
    /// Overlap margin in points between adjacent tiles (72 points = 1 inch, default: 14.17 pt ~= 5mm)
    #[serde(default = "default_overlap_pt")]
    pub overlap_pt: f32,
    /// Scale factor multiplier (default: 1.0)
    #[serde(default = "default_scale")]
    pub scale: f32,
    /// Include corner cut trim marks and tile coordinates (default: true)
    #[serde(default = "default_true")]
    pub include_guides: bool,
}

fn default_target_sheet() -> String {
    "a4".to_string()
}
fn default_grid_cols() -> u32 {
    2
}
fn default_grid_rows() -> u32 {
    2
}
fn default_overlap_pt() -> f32 {
    14.17
}
fn default_scale() -> f32 {
    1.0
}
fn default_true() -> bool {
    true
}

impl Default for PdfPosterizeOptions {
    fn default() -> Self {
        Self {
            target_sheet: default_target_sheet(),
            grid_cols: default_grid_cols(),
            grid_rows: default_grid_rows(),
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
    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err(PdfError::InvalidInput("Document contains no pages".into()));
    }

    let cols = options.grid_cols.max(1);
    let rows = options.grid_rows.max(1);
    let overlap = options.overlap_pt.max(0.0);

    let catalog = doc.catalog().map_err(|e| PdfError::ParseError(e.to_string()))?;
    let pages_id = catalog
        .get(b"Pages")
        .and_then(Object::as_reference)
        .unwrap_or((0, 0));

    let mut new_pages = Vec::new();

    // Extract page data first so we don't hold an immutable borrow on doc when adding objects
    let mut pages_data = Vec::new();
    for (page_num, object_id) in pages.into_iter() {
        let page = doc
            .get_object(object_id)
            .and_then(Object::as_dict)
            .map_err(|_| PdfError::ParseError(format!("Page {} is not a dictionary", page_num)))?
            .clone();

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
                    .unwrap_or(595.28);
                let y1 = arr[3]
                    .as_f32()
                    .or_else(|_| arr[3].as_i64().map(|v| v as f32))
                    .unwrap_or(841.89);
                (x0, y0, x1, y1)
            } else {
                (0.0, 0.0, 595.28, 841.89)
            }
        } else {
            (0.0, 0.0, 595.28, 841.89)
        };

        pages_data.push((page, media_box));
    }

    for (page_dict, (x0, y0, x1, y1)) in pages_data {
        let total_w = (x1 - x0).abs();
        let total_h = (y1 - y0).abs();

        let tile_w = total_w / cols as f32;
        let tile_h = total_h / rows as f32;

        for r in 0..rows {
            // PDF origin is bottom-left; top row has highest y
            let row_from_top = (rows - 1) - r;
            let tile_y0 = y0 + (row_from_top as f32 * tile_h) - if row_from_top > 0 { overlap } else { 0.0 };
            let tile_y1 = y0 + ((row_from_top + 1) as f32 * tile_h) + if (row_from_top + 1) < rows { overlap } else { 0.0 };

            for c in 0..cols {
                let tile_x0 = x0 + (c as f32 * tile_w) - if c > 0 { overlap } else { 0.0 };
                let tile_x1 = x0 + ((c + 1) as f32 * tile_w) + if (c + 1) < cols { overlap } else { 0.0 };

                let crop_box = vec![
                    Object::Real(tile_x0),
                    Object::Real(tile_y0),
                    Object::Real(tile_x1),
                    Object::Real(tile_y1),
                ];

                let mut tile_page = page_dict.clone();
                tile_page.set("MediaBox", Object::Array(crop_box.clone()));
                tile_page.set("CropBox", Object::Array(crop_box));
                tile_page.set("Parent", Object::Reference(pages_id));

                let tile_id = doc.add_object(tile_page);
                new_pages.push(tile_id);
            }
        }
    }

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

    let mut output_bytes = Vec::new();
    doc.save_to(&mut output_bytes)
        .map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(output_bytes)
}

#[derive(Default)]
pub struct PdfPosterizeOperation {
    _options: PdfPosterizeOptions,
}

impl PdfPosterizeOperation {
    pub fn new(options: PdfPosterizeOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfPosterizeOperation {
    fn execute(&self, _document: &mut dyn PdfDocument) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation("Execute via pdf_posterize_bytes".to_string()))
    }
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
    fn test_pdf_posterize_2x2_grid() {
        // Synthesize a valid single-page A4 document
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set(
            "MediaBox",
            Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Object::Real(1000.0),
                Object::Real(1000.0),
            ]),
        );

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

        let options = PdfPosterizeOptions {
            grid_cols: 2,
            grid_rows: 2,
            overlap_pt: 10.0,
            ..Default::default()
        };

        let result = pdf_posterize_bytes(&buf, &options).expect("Posterize should succeed");
        let out_doc = Document::load_mem(&result).expect("Output should be valid PDF");
        let out_pages = out_doc.get_pages();
        assert_eq!(out_pages.len(), 4, "1 page sliced into 2x2 grid should yield 4 tile pages");
    }
}
