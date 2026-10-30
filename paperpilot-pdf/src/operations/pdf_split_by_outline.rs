use crate::document::LopdfDocument;
use lopdf::{Document, Object, ObjectId, StringFormat};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::{Cursor, Write};
use zip::CompressionMethod;
use zip::write::FileOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSplitByOutlineOptions {
    pub max_depth: Option<usize>,
    pub sanitize_names: bool,
}

impl Default for PdfSplitByOutlineOptions {
    fn default() -> Self {
        Self {
            max_depth: Some(1),
            sanitize_names: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineSectionResult {
    pub title: String,
    pub start_page: usize,
    pub end_page: usize,
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .replace("  ", " ")
        .trim()
        .to_string()
}

#[allow(clippy::collapsible_if)]
fn resolve_page_dest(
    doc: &Document,
    dest_obj: &Object,
    page_to_index: &HashMap<ObjectId, usize>,
) -> Option<usize> {
    let dest_array = match dest_obj {
        Object::Array(arr) => arr,
        Object::Reference(id) => {
            if let Ok(Object::Array(arr)) = doc.get_object(*id) {
                arr
            } else {
                return None;
            }
        }
        _ => return None,
    };

    if dest_array.is_empty() {
        return None;
    }

    if let Object::Reference(page_id) = &dest_array[0] {
        if let Some(&index) = page_to_index.get(page_id) {
            return Some(index + 1);
        }
    } else if let Ok(page_id) = dest_array[0].as_reference() {
        if let Some(&index) = page_to_index.get(&page_id) {
            return Some(index + 1);
        }
    }

    None
}

#[allow(clippy::collapsible_if)]
#[allow(clippy::while_let_loop)]
fn traverse_outlines(
    doc: &Document,
    item_id: ObjectId,
    depth: usize,
    max_depth: usize,
    page_to_index: &HashMap<ObjectId, usize>,
    results: &mut Vec<OutlineSectionResult>,
) -> Result<(), PdfError> {
    if depth > max_depth {
        return Ok(());
    }

    let mut current_id = item_id;

    while let Ok(item_dict) = doc.get_dictionary(current_id) {
        if let Ok(title_obj) = item_dict.get(b"Title") {
            let title = match title_obj {
                Object::String(bytes, StringFormat::Literal) => {
                    String::from_utf8_lossy(bytes).into_owned()
                }
                Object::String(bytes, StringFormat::Hexadecimal) => {
                    String::from_utf8_lossy(bytes).into_owned()
                }
                _ => String::new(),
            };

            let title =
                if title.starts_with("\u{feff}") || title.as_bytes().starts_with(b"\xFE\xFF") {
                    let bytes = title_obj.as_str().unwrap_or(b"");
                    if bytes.len() >= 2 {
                        let u16_chars: Vec<u16> = bytes[2..]
                            .chunks_exact(2)
                            .map(|c| u16::from_be_bytes([c[0], c[1]]))
                            .collect();
                        String::from_utf16_lossy(&u16_chars)
                    } else {
                        title
                    }
                } else {
                    title
                };

            let mut start_page = None;

            if let Ok(dest) = item_dict.get(b"Dest") {
                start_page = resolve_page_dest(doc, dest, page_to_index);
            } else if let Ok(a_dict) = item_dict.get(b"A").and_then(Object::as_dict) {
                if let Ok(action_type) = a_dict.get(b"S").and_then(Object::as_name) {
                    if action_type == b"GoTo" {
                        if let Ok(dest) = a_dict.get(b"D") {
                            start_page = resolve_page_dest(doc, dest, page_to_index);
                        }
                    }
                }
            }

            if let Some(page) = start_page {
                if !title.is_empty() {
                    results.push(OutlineSectionResult {
                        title: title.clone(),
                        start_page: page,
                        end_page: 0,
                    });
                }
            }
        }

        if depth < max_depth {
            if let Ok(first_child) = item_dict.get(b"First").and_then(Object::as_reference) {
                traverse_outlines(
                    doc,
                    first_child,
                    depth + 1,
                    max_depth,
                    page_to_index,
                    results,
                )?;
            }
        }

        match item_dict.get(b"Next").and_then(Object::as_reference) {
            Ok(next_id) => current_id = next_id,
            Err(_) => break,
        }
    }

    Ok(())
}

pub fn pdf_split_by_outline_bytes(
    input_bytes: &[u8],
    options: &PdfSplitByOutlineOptions,
) -> Result<Vec<u8>, PdfError> {
    let doc = Document::load_mem(input_bytes).map_err(|e| PdfError::Other(e.to_string()))?;

    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err(PdfError::Other("Document has no pages".to_string()));
    }

    let mut page_to_index = HashMap::new();
    let mut ordered_pages = BTreeMap::new();
    for (i, (&page_num, &page_id)) in pages.iter().enumerate() {
        page_to_index.insert(page_id, i);
        ordered_pages.insert(page_num, page_id);
    }

    let total_pages = pages.len();
    let max_depth = options.max_depth.unwrap_or(1);

    let mut sections = Vec::new();

    // Check if we have valid outlines
    let catalog = doc.catalog().map_err(|e| PdfError::Other(e.to_string()))?;
    let mut has_valid_outlines = false;

    if let Ok(outlines_ref) = catalog.get(b"Outlines").and_then(Object::as_reference) {
        if let Ok(outlines_dict) = doc.get_dictionary(outlines_ref) {
            if let Ok(first_ref) = outlines_dict.get(b"First").and_then(Object::as_reference) {
                traverse_outlines(&doc, first_ref, 1, max_depth, &page_to_index, &mut sections)?;
                has_valid_outlines = !sections.is_empty();
            }
        }
    }

    // Fallback to single page split
    if !has_valid_outlines {
        for page_idx in 1..=total_pages {
            sections.push(OutlineSectionResult {
                title: format!("Page_{}", page_idx),
                start_page: page_idx,
                end_page: page_idx,
            });
        }
    } else {
        sections.sort_by_key(|s| s.start_page);

        for i in 0..sections.len() {
            let next_start = if i + 1 < sections.len() {
                sections[i + 1].start_page
            } else {
                total_pages + 1
            };

            let end_page = next_start.saturating_sub(1);
            sections[i].end_page = std::cmp::max(sections[i].start_page, end_page);
        }
    }

    // Process ZIP
    let mut zip_buffer = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_buffer);
        let zip_options = FileOptions::<()>::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(0o755);

        for (idx, section) in sections.iter().enumerate() {
            let mut sub_doc = doc.clone();
            let mut pages_to_keep = Vec::new();

            for page_num in section.start_page..=section.end_page {
                if let Some(&page_id) = ordered_pages.get(&(page_num as u32)) {
                    pages_to_keep.push(page_id);
                }
            }

            if pages_to_keep.is_empty() {
                continue;
            }

            let all_page_ids: Vec<ObjectId> = ordered_pages.values().copied().collect();
            let pages_to_delete: Vec<ObjectId> = all_page_ids
                .into_iter()
                .filter(|id| !pages_to_keep.contains(id))
                .collect();

            let mut page_nums_to_delete = Vec::new();
            for (p_num, p_id) in sub_doc.get_pages().iter() {
                if pages_to_delete.contains(p_id) {
                    page_nums_to_delete.push(*p_num);
                }
            }

            sub_doc.delete_pages(&page_nums_to_delete);

            if let Ok(catalog_mut) = sub_doc.catalog_mut() {
                catalog_mut.remove(b"Outlines");
            }

            let mut pdf_buffer = Vec::new();
            sub_doc
                .save_to(&mut pdf_buffer)
                .map_err(|e| PdfError::Other(e.to_string()))?;

            let mut filename = if options.sanitize_names {
                sanitize_filename(&section.title)
            } else {
                section.title.clone()
            };

            if filename.is_empty() {
                filename = format!("Chapter_{}", idx + 1);
            }

            let filename = format!("{:02}_{}.pdf", idx + 1, filename);
            zip.start_file(&filename, zip_options)
                .map_err(|e| PdfError::Other(e.to_string()))?;
            zip.write_all(&pdf_buffer).map_err(PdfError::IoError)?;
        }

        zip.finish().map_err(|e| PdfError::Other(e.to_string()))?;
    }

    Ok(zip_buffer.into_inner())
}

#[derive(Default)]
#[allow(dead_code)]
pub struct PdfSplitByOutlineOperation {
    options: PdfSplitByOutlineOptions,
}

impl PdfSplitByOutlineOperation {
    pub fn new(options: PdfSplitByOutlineOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfSplitByOutlineOperation {
    fn execute(
        &self,
        document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let bytes = if let Some(orig) = &lopdf_doc.original_bytes {
            orig.clone()
        } else {
            let mut buf = Vec::new();
            lopdf_doc
                .inner
                .save_to(&mut buf)
                .map_err(|e| PdfError::Other(e.to_string()))?;
            buf
        };

        let zip_bytes = pdf_split_by_outline_bytes(&bytes, &self.options)?;
        lopdf_doc.original_bytes = Some(zip_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::dictionary;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("Chapter 1: Intro/Test"),
            "Chapter 1_ Intro_Test"
        );
        assert_eq!(sanitize_filename("Part #1 *"), "Part _1 _");
    }

    #[test]
    fn test_pdf_split_by_outline_empty_document() {
        let options = PdfSplitByOutlineOptions::default();
        let bytes = vec![];
        let result = pdf_split_by_outline_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_split_by_outline_no_bookmarks() {
        let mut inner = Document::with_version("1.5");
        let pages_id = inner.new_object_id();

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", lopdf::Object::Array(vec![]));
        pages_dict.set("Count", lopdf::Object::Integer(0));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        inner.save_to(&mut bytes).unwrap();

        let options = PdfSplitByOutlineOptions::default();
        let result = pdf_split_by_outline_bytes(&bytes, &options);

        // Document has no pages, should fail with standard error
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Document has no pages"));
    }

    #[test]
    fn test_pdf_split_by_outline_with_fixture() {
        let path = std::path::Path::new("../fixtures/test_bookmarks.pdf");
        if path.exists() {
            let bytes = std::fs::read(path).unwrap();
            let options = PdfSplitByOutlineOptions::default();
            let result = pdf_split_by_outline_bytes(&bytes, &options);
            assert!(result.is_ok());

            let zip_bytes = result.unwrap();
            let cursor = std::io::Cursor::new(zip_bytes);
            let mut archive = zip::ZipArchive::new(cursor).unwrap();

            assert_eq!(archive.len(), 3);
            let mut file_names: Vec<String> = (0..archive.len())
                .map(|i| archive.by_index(i).unwrap().name().to_string())
                .collect();
            file_names.sort();

            assert_eq!(file_names[0], "01_Chapter 1.pdf");
            assert_eq!(file_names[1], "02_Chapter 2.pdf");
            assert_eq!(file_names[2], "03_Chapter 3.pdf");
        }
    }

    #[test]
    fn test_pdf_split_by_outline_fallback() {
        let mut inner = Document::with_version("1.5");
        let pages_id = inner.new_object_id();

        // Add two pages to test the fallback works and splits by page
        let page1_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
        });

        let page2_id = inner.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
        });

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", lopdf::Object::Array(vec![
            lopdf::Object::Reference(page1_id),
            lopdf::Object::Reference(page2_id)
        ]));
        pages_dict.set("Count", lopdf::Object::Integer(2));
        inner.set_object(pages_id, pages_dict);

        let mut catalog_dict = lopdf::Dictionary::new();
        catalog_dict.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = inner.add_object(catalog_dict);

        inner
            .trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        inner.save_to(&mut bytes).unwrap();

        let options = PdfSplitByOutlineOptions::default();
        let result = pdf_split_by_outline_bytes(&bytes, &options);

        // Document has pages but no outlines, should fallback and succeed
        assert!(result.is_ok());

        let zip_bytes = result.unwrap();
        let cursor = std::io::Cursor::new(zip_bytes);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();

        assert_eq!(archive.len(), 2);
        let mut file_names: Vec<String> = (0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().to_string())
            .collect();
        file_names.sort();

        assert_eq!(file_names[0], "01_Page_1.pdf");
        assert_eq!(file_names[1], "02_Page_2.pdf");
    }
}
