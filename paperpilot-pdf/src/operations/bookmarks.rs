use crate::document::LopdfDocument;
use lopdf::{Document, Object, ObjectId};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkItem {
    pub title: String,
    pub page: Option<u32>,
    pub level: usize,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<BookmarkItem>,
}

#[derive(Default)]
pub struct BookmarksOperation {
    pub output_path: Option<std::path::PathBuf>,
}

impl BookmarksOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_output(output: std::path::PathBuf) -> Self {
        Self {
            output_path: Some(output),
        }
    }

    /// Extract hierarchical bookmarks tree from lopdf Document
    pub fn extract(doc: &Document) -> Vec<BookmarkItem> {
        let catalog = match doc.catalog() {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let outlines_id = match catalog.get(b"Outlines") {
            Ok(Object::Reference(id)) => *id,
            _ => return Vec::new(),
        };

        let outlines_dict = match doc.get_object(outlines_id).and_then(|o| o.as_dict()) {
            Ok(d) => d,
            Err(_) => return Vec::new(),
        };

        let first_id = match outlines_dict.get(b"First") {
            Ok(Object::Reference(id)) => *id,
            _ => return Vec::new(),
        };

        // Build mapping from page ObjectId to 1-based page number
        let mut page_map = HashMap::new();
        for (idx, page_id) in doc.page_iter().enumerate() {
            page_map.insert(page_id, (idx + 1) as u32);
        }

        Self::traverse_items(doc, first_id, 0, &page_map)
    }

    fn traverse_items(
        doc: &Document,
        mut current_id: ObjectId,
        level: usize,
        page_map: &HashMap<ObjectId, u32>,
    ) -> Vec<BookmarkItem> {
        let mut items = Vec::new();
        let mut visited = std::collections::HashSet::new();

        while visited.insert(current_id) {
            let item_dict = match doc.get_object(current_id).and_then(|o| o.as_dict()) {
                Ok(d) => d,
                Err(_) => break,
            };

            let title = match item_dict.get(b"Title") {
                Ok(Object::String(bytes, _)) => {
                    // Try UTF-8 or Latin1 or UTF-16BE
                    if bytes.starts_with(&[0xFE, 0xFF]) {
                        // UTF-16BE BOM
                        let u16_chars: Vec<u16> = bytes[2..]
                            .chunks_exact(2)
                            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                            .collect();
                        String::from_utf16_lossy(&u16_chars)
                    } else {
                        String::from_utf8(bytes.clone())
                            .unwrap_or_else(|_| bytes.iter().map(|&b| b as char).collect())
                    }
                }
                _ => "(Untitled)".to_string(),
            };

            // Resolve destination page if present
            let mut page = None;
            if let Ok(dest_obj) = item_dict.get(b"Dest") {
                page = Self::resolve_dest_page(doc, dest_obj, page_map);
            } else if let Ok(Object::Dictionary(action_dict)) = item_dict.get(b"A") {
                if let Ok(dest_obj) = action_dict.get(b"D") {
                    page = Self::resolve_dest_page(doc, dest_obj, page_map);
                }
            }

            // Check for children
            let children = if let Ok(Object::Reference(first_child_id)) = item_dict.get(b"First") {
                Self::traverse_items(doc, *first_child_id, level + 1, page_map)
            } else {
                Vec::new()
            };

            items.push(BookmarkItem {
                title,
                page,
                level,
                children,
            });

            // Move to Next sibling
            match item_dict.get(b"Next") {
                Ok(Object::Reference(next_id)) => {
                    current_id = *next_id;
                }
                _ => break,
            }
        }

        items
    }

    fn resolve_dest_page(
        doc: &Document,
        dest_obj: &Object,
        page_map: &HashMap<ObjectId, u32>,
    ) -> Option<u32> {
        match dest_obj {
            Object::Array(arr) => {
                if let Some(first) = arr.first() {
                    match first {
                        Object::Reference(page_id) => page_map.get(page_id).copied(),
                        Object::Integer(p) => Some((*p + 1) as u32),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Object::Reference(dest_id) => {
                if let Ok(target) = doc.get_object(*dest_id) {
                    Self::resolve_dest_page(doc, target, page_map)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl PdfOperation for BookmarksOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| PdfError::UnsupportedOperation("Only LopdfDocument supported".into()))?;

        let items = Self::extract(&lopdf_doc.inner);

        if let Some(ref path) = self.output_path {
            let json_str = serde_json::to_string_pretty(&serde_json::json!({
                "bookmarks": items,
                "count": items.len()
            }))
            .map_err(|e| PdfError::Other(e.to_string()))?;

            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::write(path, json_str)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::LopdfDocument;

    #[test]
    fn test_bookmarks_operation() {
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

        let mut doc = LopdfDocument {
            inner,
            original_bytes: None,
        };

        let op = BookmarksOperation::new();
        let result = op.execute(&mut doc);

        assert!(result.is_ok());
    }
}
