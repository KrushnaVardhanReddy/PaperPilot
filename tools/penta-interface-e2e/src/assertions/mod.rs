use anyhow::{anyhow, Result};
use lopdf::Document;
use std::path::Path;

pub struct PdfAssertions;

impl PdfAssertions {
    pub fn assert_page_count<P: AsRef<Path>>(path: P, expected: usize) -> Result<()> {
        let doc = Document::load(path)?;
        let pages = doc.get_pages();
        if pages.len() == expected {
            Ok(())
        } else {
            Err(anyhow!("Expected {} pages, got {}", expected, pages.len()))
        }
    }

    pub fn assert_page_contains_text<P: AsRef<Path>>(
        path: P,
        page_num: u32,
        expected_text: &str,
    ) -> Result<()> {
        let doc = Document::load(path)?;
        let text = doc.extract_text(&[page_num]).unwrap_or_default();
        if text.contains(expected_text) {
            Ok(())
        } else {
            // Also check raw streams as fallback
            let pages = doc.get_pages();
            if let Some(&page_id) = pages.get(&page_num) {
                let content = doc.get_page_content(page_id);
                let content_str = String::from_utf8_lossy(&content);
                if content_str.contains(expected_text) {
                    return Ok(());
                }
            }
            Err(anyhow!(
                "Page {} does not contain expected text '{}'",
                page_num,
                expected_text
            ))
        }
    }

    pub fn assert_rotation<P: AsRef<Path>>(
        path: P,
        page_num: u32,
        expected_rotate: i64,
    ) -> Result<()> {
        let doc = Document::load(path)?;
        let pages = doc.get_pages();
        if let Some(&page_id) = pages.get(&page_num) {
            if let Ok(page_dict) = doc.get_dictionary(page_id) {
                if let Ok(rotate) = page_dict.get_deref(b"Rotate", &doc) {
                    if let Ok(r) = rotate.as_i64() {
                        if r == expected_rotate {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(anyhow!(
            "Page {} does not have expected rotation {}",
            page_num,
            expected_rotate
        ))
    }
}
