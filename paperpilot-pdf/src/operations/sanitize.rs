#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
use lopdf::{Document, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeOptions {
    #[serde(default = "default_true")]
    pub remove_metadata: bool,
    #[serde(default = "default_true")]
    pub remove_attachments: bool,
    #[serde(default = "default_true")]
    pub remove_scripts: bool,
    #[serde(default = "default_true")]
    pub remove_thumbnails: bool,
    #[serde(default = "default_true")]
    pub remove_private_data: bool,
}

fn default_true() -> bool {
    true
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self {
            remove_metadata: true,
            remove_attachments: true,
            remove_scripts: true,
            remove_thumbnails: true,
            remove_private_data: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeReport {
    pub metadata_stripped: bool,
    pub attachments_removed: usize,
    pub scripts_removed: usize,
    pub thumbnails_removed: usize,
    pub output_bytes: usize,
}

pub fn sanitize_pdf(
    input_bytes: &[u8],
    options: &SanitizeOptions,
) -> Result<(Vec<u8>, SanitizeReport), paperpilot_core::error::PdfError> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| paperpilot_core::error::PdfError::ParseError(e.to_string()))?;
    let mut attachments_removed = 0;
    let mut scripts_removed = 0;
    let mut thumbnails_removed = 0;

    // 1. Strip Document Information Dictionary (/Info) & Metadata XMP
    if options.remove_metadata {
        doc.trailer.remove(b"Info");
        // Remove /Metadata from Catalog and Pages
        if let Ok(catalog_id) = doc.catalog_mut() {
            catalog_id.remove(b"Metadata");
        }
    }

    // 2. Strip Embedded Files (/Names -> /EmbeddedFiles, /AF)
    if options.remove_attachments {
        if let Ok(catalog) = doc.catalog_mut() {
            if let Ok(names) = catalog.get_mut(b"Names") {
                if let Object::Dictionary(names_dict) = names {
                    if names_dict.remove(b"EmbeddedFiles").is_some() {
                        attachments_removed += 1;
                    }
                }
            }
            if catalog.remove(b"AF").is_some() {
                attachments_removed += 1;
            }
        }
    }

    // 3. Neutralize Active Scripts (/JavaScript, /JS, /OpenAction, /AA)
    if options.remove_scripts {
        if let Ok(catalog) = doc.catalog_mut() {
            if catalog.remove(b"OpenAction").is_some() {
                scripts_removed += 1;
            }
            if catalog.remove(b"AA").is_some() {
                scripts_removed += 1;
            }
            if let Ok(names) = catalog.get_mut(b"Names") {
                if let Object::Dictionary(names_dict) = names {
                    if names_dict.remove(b"JavaScript").is_some() {
                        scripts_removed += 1;
                    }
                }
            }
        }
    }

    // 4. Strip Page Thumbnails (/Thumb) & Private Editing Data (/PieceInfo)
    let page_ids = doc.get_pages();
    for (_page_num, page_id) in page_ids {
        if let Ok(page_obj) = doc.get_object_mut(page_id) {
            if let Object::Dictionary(page_dict) = page_obj {
                if options.remove_thumbnails && page_dict.remove(b"Thumb").is_some() {
                    thumbnails_removed += 1;
                }
                if options.remove_private_data {
                    page_dict.remove(b"PieceInfo");
                }
                if options.remove_scripts {
                    if page_dict.remove(b"AA").is_some() {
                        scripts_removed += 1;
                    }
                }
            }
        }
    }

    // Clean xref & write to bytes
    let mut output_bytes = Vec::new();
    doc.save_to(&mut output_bytes).map_err(|e| {
        paperpilot_core::error::PdfError::IoError(std::io::Error::other(e.to_string()))
    })?;

    let report = SanitizeReport {
        metadata_stripped: options.remove_metadata,
        attachments_removed,
        scripts_removed,
        thumbnails_removed,
        output_bytes: output_bytes.len(),
    };

    Ok((output_bytes, report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Object};

    fn create_dummy_pdf_with_metadata_and_scripts() -> Vec<u8> {
        let mut doc = Document::with_version("1.4");

        // Info dictionary
        let mut info_dict = Dictionary::new();
        info_dict.set(
            "Title",
            Object::String(b"Test Document".to_vec(), lopdf::StringFormat::Literal),
        );
        let info_id = doc.add_object(Object::Dictionary(info_dict));
        doc.trailer.set("Info", Object::Reference(info_id));

        let pages_id = doc.new_object_id();
        let catalog_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        // Page with Thumb, PieceInfo, and AA
        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("Thumb", Object::Reference(doc.new_object_id()));
        page_dict.set("PieceInfo", Object::Dictionary(Dictionary::new()));
        page_dict.set("AA", Object::Dictionary(Dictionary::new()));
        doc.objects.insert(page_id, Object::Dictionary(page_dict));

        // Pages
        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
        pages_dict.set("Count", Object::Integer(1));
        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        // Catalog with Metadata, OpenAction, AA, Names (with EmbeddedFiles and JavaScript), AF
        let mut catalog_dict = Dictionary::new();
        catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", Object::Reference(pages_id));
        catalog_dict.set("Metadata", Object::Reference(doc.new_object_id()));
        catalog_dict.set("OpenAction", Object::Array(vec![]));
        catalog_dict.set("AA", Object::Dictionary(Dictionary::new()));

        let mut names_dict = Dictionary::new();
        names_dict.set("EmbeddedFiles", Object::Dictionary(Dictionary::new()));
        names_dict.set("JavaScript", Object::Dictionary(Dictionary::new()));
        catalog_dict.set("Names", Object::Dictionary(names_dict));

        catalog_dict.set("AF", Object::Array(vec![]));

        doc.objects
            .insert(catalog_id, Object::Dictionary(catalog_dict));
        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();
        buf
    }

    #[test]
    fn test_sanitize_pdf_all_options() {
        let input_bytes = create_dummy_pdf_with_metadata_and_scripts();
        let options = SanitizeOptions::default();
        let (output_bytes, report) = sanitize_pdf(&input_bytes, &options).unwrap();

        assert!(report.metadata_stripped);
        assert_eq!(report.attachments_removed, 2); // EmbeddedFiles and AF
        assert_eq!(report.scripts_removed, 4); // OpenAction, AA in catalog, JavaScript in names, AA in page
        assert_eq!(report.thumbnails_removed, 1);

        let sanitized_doc = Document::load_mem(&output_bytes).unwrap();

        // Verify /Info stripped
        assert!(sanitized_doc.trailer.get(b"Info").is_err());

        // Verify catalog
        let catalog = sanitized_doc.catalog().unwrap();
        assert!(catalog.get(b"Metadata").is_err());
        assert!(catalog.get(b"OpenAction").is_err());
        assert!(catalog.get(b"AA").is_err());
        assert!(catalog.get(b"AF").is_err());

        if let Ok(names) = catalog.get(b"Names") {
            if let Ok(names_dict) = names.as_dict() {
                assert!(names_dict.get(b"EmbeddedFiles").is_err());
                assert!(names_dict.get(b"JavaScript").is_err());
            }
        }

        // Verify page
        let page_ids = sanitized_doc.get_pages();
        for (_, page_id) in page_ids {
            let page_dict = sanitized_doc
                .get_object(page_id)
                .unwrap()
                .as_dict()
                .unwrap();
            assert!(page_dict.get(b"Thumb").is_err());
            assert!(page_dict.get(b"PieceInfo").is_err());
            assert!(page_dict.get(b"AA").is_err());
        }
    }
}
