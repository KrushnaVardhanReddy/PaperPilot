use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfToEpubOptions {
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
}

use crate::document::LopdfDocument;

use lopdf::{Document, Object, ObjectId, StringFormat};
use std::collections::HashMap;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

struct ChapterData {
    title: String,
    content: String,
}

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

fn traverse_outlines(
    doc: &Document,
    item_id: ObjectId,
    page_to_index: &HashMap<ObjectId, usize>,
    results: &mut Vec<(String, usize)>,
) -> Result<(), PdfError> {
    let mut current_id = item_id;

    while let Ok(item_dict) = doc.get_dictionary(current_id) {
        if let Ok(title_obj) = item_dict.get(b"Title") {
            let title = match title_obj {
                Object::String(bytes, StringFormat::Literal)
                | Object::String(bytes, StringFormat::Hexadecimal) => {
                    if bytes.starts_with(b"\xFE\xFF") {
                        let u16_chars: Vec<u16> = bytes[2..]
                            .chunks_exact(2)
                            .map(|c| u16::from_be_bytes([c[0], c[1]]))
                            .collect();
                        String::from_utf16_lossy(&u16_chars)
                    } else {
                        String::from_utf8_lossy(bytes).into_owned()
                    }
                }
                _ => String::new(),
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
                let trimmed_title = title.trim();
                if !trimmed_title.is_empty() {
                    results.push((trimmed_title.to_string(), page));
                }
            }
        }

        if let Ok(first_child) = item_dict.get(b"First").and_then(Object::as_reference) {
            let _ = traverse_outlines(doc, first_child, page_to_index, results);
        }

        match item_dict.get(b"Next").and_then(Object::as_reference) {
            Ok(next_id) => current_id = next_id,
            Err(_) => break,
        }
    }

    Ok(())
}

fn detect_heading(page_text: &str, page_idx: usize) -> String {
    for line in page_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cleaned = trimmed.trim_start_matches('#').trim();
        if cleaned.is_empty() {
            continue;
        }

        if cleaned.len() <= 100 {
            let lower = cleaned.to_lowercase();
            if lower.starts_with("chapter")
                || lower.starts_with("section")
                || lower.starts_with("heading")
                || lower.starts_with("page")
                || (cleaned.chars().next().map_or(false, |c| c.is_uppercase()) && cleaned.len() <= 60)
            {
                return cleaned.to_string();
            }
        }
        break;
    }

    format!("Page {}", page_idx)
}

pub fn pdf_to_epub_bytes(
    input_bytes: &[u8],
    options: &PdfToEpubOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::InvalidInput(
            "Input bytes cannot be empty".to_string(),
        ));
    }

    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF with lopdf: {:?}", e)))?;

    // Extract pages and text
    let pages = doc.get_pages();
    let total_pages = pages.len();
    if total_pages == 0 {
        return Err(PdfError::InvalidInput(
            "PDF contains no pages".to_string(),
        ));
    }

    let mut page_to_index = HashMap::new();
    let mut page_texts = Vec::new();

    // Iterate through pages in sorted order
    for (idx, (&page_num, &page_id)) in pages.iter().enumerate() {
        page_to_index.insert(page_id, idx);
        let text = doc.extract_text(&[page_num]).unwrap_or_default();

        // Unwrap lines and soft hyphens
        let text = text.replace("-\n", "").replace('\r', "");
        page_texts.push(text);
    }

    // Try to detect chapters from /Outlines
    let catalog = doc
        .catalog()
        .map_err(|e| PdfError::ParseError(format!("Failed to get catalog: {:?}", e)))?;

    let mut outline_sections = Vec::new();
    if let Ok(outlines_ref) = catalog.get(b"Outlines").and_then(Object::as_reference) {
        if let Ok(outlines_dict) = doc.get_dictionary(outlines_ref) {
            if let Ok(first_ref) = outlines_dict.get(b"First").and_then(Object::as_reference) {
                let _ = traverse_outlines(&doc, first_ref, &page_to_index, &mut outline_sections);
            }
        }
    }

    let mut chapter_data: Vec<ChapterData> = Vec::new();

    if !outline_sections.is_empty() {
        // Sort outline sections by page
        outline_sections.sort_by_key(|s| s.1);
        outline_sections.dedup_by_key(|s| s.1);

        for (i, (title, start_page)) in outline_sections.iter().enumerate() {
            let start_idx = start_page.saturating_sub(1);
            if start_idx >= total_pages {
                continue;
            }

            let next_page = if i + 1 < outline_sections.len() {
                outline_sections[i + 1].1
            } else {
                total_pages + 1
            };
            let end_idx = (next_page.saturating_sub(1)).min(total_pages);

            let content = if start_idx < end_idx {
                page_texts[start_idx..end_idx].join("\n\n")
            } else if start_idx < total_pages {
                page_texts[start_idx].clone()
            } else {
                String::new()
            };

            chapter_data.push(ChapterData {
                title: title.clone(),
                content,
            });
        }
    }

    // Fallback: if no outline chapters were extracted, create a chapter per page
    if chapter_data.is_empty() {
        for (i, text) in page_texts.iter().enumerate() {
            let page_num = i + 1;
            let title = detect_heading(text, page_num);
            chapter_data.push(ChapterData {
                title,
                content: text.clone(),
            });
        }
    }

    let mut chapters = Vec::new();

    for (i, chapter) in chapter_data.iter().enumerate() {
        let paragraphs: Vec<String> = chapter
            .content
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();

        let mut xhtml = String::new();
        xhtml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        xhtml.push_str("<!DOCTYPE html>\n");
        xhtml.push_str("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\">\n");
        xhtml.push_str("<head>\n");
        xhtml.push_str(&format!(
            "<title>{}</title>\n",
            escape_xml(&chapter.title)
        ));
        xhtml.push_str("<link href=\"styles.css\" rel=\"stylesheet\" type=\"text/css\"/>\n");
        xhtml.push_str("</head>\n");
        xhtml.push_str("<body>\n");
        xhtml.push_str(&format!("<h2>{}</h2>\n", escape_xml(&chapter.title)));

        if paragraphs.is_empty() {
            xhtml.push_str("<p></p>\n");
        } else {
            for p in paragraphs {
                // Convert remaining single newlines within paragraph to spaces
                let p_clean = p.replace('\n', " ");
                let escaped = escape_xml(&p_clean);
                xhtml.push_str(&format!("<p>{}</p>\n", escaped));
            }
        }

        xhtml.push_str("</body>\n");
        xhtml.push_str("</html>");

        chapters.push((format!("chapter_{}.xhtml", i + 1), chapter.title.clone(), xhtml));
    }

    let buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buffer);

    // 1. mimetype (MUST be uncompressed and first)
    let mimetype_options =
        SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("mimetype", mimetype_options)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
    zip.write_all(b"application/epub+zip")
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 2. META-INF/container.xml
    let options_deflated =
        SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file("META-INF/container.xml", options_deflated)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
    let container_xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
<container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">\
  <rootfiles>\
    <rootfile full-path=\"OEBPS/content.opf\" media-type=\"application/oebps-package+xml\"/>\
  </rootfiles>\
</container>";
    zip.write_all(container_xml.as_bytes())
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 3. OEBPS/content.opf
    let title = options.title.as_deref().unwrap_or("Untitled Document");
    let author = options.author.as_deref().unwrap_or("Unknown Author");
    let language = options.language.as_deref().unwrap_or("en");

    zip.start_file("OEBPS/content.opf", options_deflated)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;

    let mut opf = String::new();
    opf.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    opf.push_str("<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"BookId\">\n");
    opf.push_str("  <metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n");
    opf.push_str(&format!(
        "    <dc:title>{}</dc:title>\n",
        escape_xml(title)
    ));
    opf.push_str(&format!(
        "    <dc:creator>{}</dc:creator>\n",
        escape_xml(author)
    ));
    opf.push_str(&format!(
        "    <dc:language>{}</dc:language>\n",
        escape_xml(language)
    ));
    opf.push_str("    <dc:identifier id=\"BookId\">urn:uuid:paperpilot-pdf-to-epub</dc:identifier>\n");
    opf.push_str("  </metadata>\n");

    opf.push_str("  <manifest>\n");
    opf.push_str("    <item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\"/>\n");
    opf.push_str("    <item id=\"nav\" href=\"toc.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n");
    opf.push_str("    <item id=\"css\" href=\"styles.css\" media-type=\"text/css\"/>\n");
    for (i, (filename, _, _)) in chapters.iter().enumerate() {
        opf.push_str(&format!(
            "    <item id=\"chapter_{}\" href=\"{}\" media-type=\"application/xhtml+xml\"/>\n",
            i + 1,
            filename
        ));
    }
    opf.push_str("  </manifest>\n");

    opf.push_str("  <spine toc=\"ncx\">\n");
    for i in 0..chapters.len() {
        opf.push_str(&format!(
            "    <itemref idref=\"chapter_{}\"/>\n",
            i + 1
        ));
    }
    opf.push_str("  </spine>\n");
    opf.push_str("</package>");

    zip.write_all(opf.as_bytes())
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 4. OEBPS/toc.xhtml
    zip.start_file("OEBPS/toc.xhtml", options_deflated)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
    let mut nav = String::new();
    nav.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    nav.push_str("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\">\n");
    nav.push_str("<head><title>Table of Contents</title></head>\n");
    nav.push_str("<body>\n");
    nav.push_str("<nav epub:type=\"toc\" id=\"toc\">\n");
    nav.push_str("  <h1>Table of Contents</h1>\n");
    nav.push_str("  <ol>\n");
    for (i, (filename, ch_title, _)) in chapters.iter().enumerate() {
        let label = if ch_title.is_empty() {
            format!("Chapter {}", i + 1)
        } else {
            ch_title.clone()
        };
        nav.push_str(&format!(
            "    <li><a href=\"{}\">{}</a></li>\n",
            filename,
            escape_xml(&label)
        ));
    }
    nav.push_str("  </ol>\n");
    nav.push_str("</nav>\n");
    nav.push_str("</body>\n");
    nav.push_str("</html>");
    zip.write_all(nav.as_bytes())
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 5. OEBPS/toc.ncx
    zip.start_file("OEBPS/toc.ncx", options_deflated)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
    let mut ncx = String::new();
    ncx.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    ncx.push_str("<ncx xmlns=\"http://www.daisy.org/z3986/2005/ncx/\" version=\"2005-1\">\n");
    ncx.push_str("  <head>\n");
    ncx.push_str("    <meta name=\"dtb:uid\" content=\"urn:uuid:paperpilot-pdf-to-epub\"/>\n");
    ncx.push_str("    <meta name=\"dtb:depth\" content=\"1\"/>\n");
    ncx.push_str("    <meta name=\"dtb:totalPageCount\" content=\"0\"/>\n");
    ncx.push_str("    <meta name=\"dtb:maxPageNumber\" content=\"0\"/>\n");
    ncx.push_str("  </head>\n");
    ncx.push_str(&format!(
        "  <docTitle><text>{}</text></docTitle>\n",
        escape_xml(title)
    ));
    ncx.push_str("  <navMap>\n");
    for (i, (filename, ch_title, _)) in chapters.iter().enumerate() {
        let label = if ch_title.is_empty() {
            format!("Chapter {}", i + 1)
        } else {
            ch_title.clone()
        };
        ncx.push_str(&format!(
            "    <navPoint id=\"navPoint-{}\" playOrder=\"{}\">\n",
            i + 1,
            i + 1
        ));
        ncx.push_str(&format!(
            "      <navLabel><text>{}</text></navLabel>\n",
            escape_xml(&label)
        ));
        ncx.push_str(&format!(
            "      <content src=\"{}\"/>\n",
            filename
        ));
        ncx.push_str("    </navPoint>\n");
    }
    ncx.push_str("  </navMap>\n");
    ncx.push_str("</ncx>");
    zip.write_all(ncx.as_bytes())
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 6. OEBPS/styles.css
    zip.start_file("OEBPS/styles.css", options_deflated)
        .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
    let css = "body { font-family: serif; margin: 5%; text-align: justify;\n}\np { margin-bottom: 1em; line-height: 1.5;\n}\nh1, h2, h3, h4, h5, h6 { font-family: sans-serif;\n}";
    zip.write_all(css.as_bytes())
        .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;

    // 7. Chapters
    for (filename, _, xhtml_content) in chapters.iter() {
        zip.start_file(format!("OEBPS/{}", filename), options_deflated)
            .map_err(|e| PdfError::Other(format!("ZIP error: {:?}", e)))?;
        zip.write_all(xhtml_content.as_bytes())
            .map_err(|e| PdfError::Other(format!("ZIP write error: {:?}", e)))?;
    }

    let finished_buffer = zip
        .finish()
        .map_err(|e| PdfError::Other(format!("ZIP finish error: {:?}", e)))?;

    Ok(finished_buffer.into_inner())
}

#[derive(Default)]
pub struct PdfToEpubOperation {
    _options: PdfToEpubOptions,
}

impl PdfToEpubOperation {
    pub fn new(options: PdfToEpubOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfToEpubOperation {
    fn execute(
        &self,
        document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        let doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document must be LopdfDocument".to_string())
            })?;

        let mut bytes = Vec::new();
        if let Some(orig) = &doc.original_bytes {
            bytes = orig.clone();
        } else {
            doc.inner
                .save_to(&mut bytes)
                .map_err(|e| PdfError::Other(format!("Failed to save PDF: {:?}", e)))?;
        }

        let _epub_bytes = pdf_to_epub_bytes(&bytes, &self._options)?;

        // This is a conversion operation.
        // We shouldn't replace original bytes with EPUB bytes on LopdfDocument.
        // Usually the user receives this via binary output adapter instead of in-place mutation.
        // So for PdfOperation we might just not do much or return an error since it's a binary export.
        // Since pdf_to_epub_bytes is the primary function, let's just make execute a no-op
        // or store the result somewhere if it were supported. For now, it returns error or ok.
        Err(PdfError::UnsupportedOperation(
            "Use pdf_to_epub_bytes directly. In-place conversion is unsupported.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::io::Read;
    use zip::ZipArchive;

    #[test]
    fn test_pdf_to_epub_empty_document() {
        let options = PdfToEpubOptions::default();
        let bytes = vec![];
        let result = pdf_to_epub_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_to_epub_valid_document() {
        // Create a dummy valid PDF bytes (empty page pdf)
        let pdf_bytes = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Count 1 /Kids [3 0 R] >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\ntrailer\n<< /Root 1 0 R /Size 4 >>\nstartxref\n0\n%%EOF";
        let options = PdfToEpubOptions {
            title: Some("Test Title".to_string()),
            author: Some("Test Author".to_string()),
            language: Some("en".to_string()),
        };

        let result = pdf_to_epub_bytes(pdf_bytes, &options);
        assert!(
            result.is_ok(),
            "pdf_to_epub_bytes should succeed on a valid PDF"
        );

        let epub_bytes = result.unwrap();
        assert!(!epub_bytes.is_empty(), "EPUB output should not be empty");

        // EPUB is a ZIP archive, starts with PK\x03\x04
        assert_eq!(
            &epub_bytes[0..4],
            b"PK\x03\x04",
            "Output is not a ZIP archive"
        );

        // Read with zip archive
        let cursor = Cursor::new(epub_bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();

        // Verify mimetype is entry index 0 and stored uncompressed
        let mut mimetype = archive.by_index(0).unwrap();
        assert_eq!(mimetype.name(), "mimetype");
        assert_eq!(mimetype.compression(), zip::CompressionMethod::Stored);

        let mut mimetype_content = String::new();
        mimetype.read_to_string(&mut mimetype_content).unwrap();
        assert_eq!(mimetype_content, "application/epub+zip");

        // Cannot read other files while holding lock on `mimetype` so drop it
        drop(mimetype);

        // Verify META-INF/container.xml
        let mut container = archive.by_name("META-INF/container.xml").unwrap();
        let mut container_content = String::new();
        container.read_to_string(&mut container_content).unwrap();
        assert!(container_content.contains("OEBPS/content.opf"));
        drop(container);

        // Verify OEBPS/content.opf
        let mut opf = archive.by_name("OEBPS/content.opf").unwrap();
        let mut opf_content = String::new();
        opf.read_to_string(&mut opf_content).unwrap();
        assert!(opf_content.contains("<dc:title>Test Title</dc:title>"));
        assert!(opf_content.contains("<dc:creator>Test Author</dc:creator>"));
        assert!(opf_content.contains("<dc:language>en</dc:language>"));
        assert!(opf_content.contains("<item id=\"chapter_1\" href=\"chapter_1.xhtml\""));
        drop(opf);

        // Verify OEBPS/chapter_1.xhtml exists
        let mut chapter = archive.by_name("OEBPS/chapter_1.xhtml").unwrap();
        let mut chapter_content = String::new();
        chapter.read_to_string(&mut chapter_content).unwrap();
        assert!(chapter_content.contains("<?xml"));
        assert!(chapter_content.contains("<html"));
        drop(chapter);
    }

    #[test]
    fn test_pdf_to_epub_multi_page() {
        // Create a 2-page PDF
        let pdf_bytes = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Count 2 /Kids [3 0 R 4 0 R] >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\n4 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\ntrailer\n<< /Root 1 0 R /Size 5 >>\nstartxref\n0\n%%EOF";
        let options = PdfToEpubOptions {
            title: Some("Multi-Page Title".to_string()),
            author: Some("Multi Author".to_string()),
            language: Some("en".to_string()),
        };

        let result = pdf_to_epub_bytes(pdf_bytes, &options);
        assert!(result.is_ok(), "pdf_to_epub_bytes should succeed on multi-page PDF");

        let epub_bytes = result.unwrap();
        let cursor = Cursor::new(epub_bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();

        // Check chapter_1.xhtml and chapter_2.xhtml
        {
            let chapter1 = archive.by_name("OEBPS/chapter_1.xhtml");
            assert!(chapter1.is_ok(), "chapter_1.xhtml should exist");
        }
        {
            let chapter2 = archive.by_name("OEBPS/chapter_2.xhtml");
            assert!(chapter2.is_ok(), "chapter_2.xhtml should exist");
        }

        // Check content.opf has both itemrefs
        {
            let mut opf = archive.by_name("OEBPS/content.opf").unwrap();
            let mut opf_content = String::new();
            opf.read_to_string(&mut opf_content).unwrap();
            assert!(opf_content.contains("<itemref idref=\"chapter_1\"/>"));
            assert!(opf_content.contains("<itemref idref=\"chapter_2\"/>"));
        }

        // Check toc.xhtml has both
        {
            let mut toc = archive.by_name("OEBPS/toc.xhtml").unwrap();
            let mut toc_content = String::new();
            toc.read_to_string(&mut toc_content).unwrap();
            assert!(toc_content.contains("chapter_1.xhtml"));
            assert!(toc_content.contains("chapter_2.xhtml"));
        }

        // Check toc.ncx has both
        {
            let mut ncx = archive.by_name("OEBPS/toc.ncx").unwrap();
            let mut ncx_content = String::new();
            ncx.read_to_string(&mut ncx_content).unwrap();
            assert!(ncx_content.contains("chapter_1.xhtml"));
            assert!(ncx_content.contains("chapter_2.xhtml"));
        }
    }
}
