use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAutoRotateOptions {
    /// Target orientation: "portrait" (default), "landscape", or "auto"
    #[serde(default = "default_target_orientation")]
    pub target_orientation: String,

    /// Optional specific 1-based page numbers to rotate. None = all pages.
    #[serde(default)]
    pub pages: Option<Vec<u32>>,
}

fn default_target_orientation() -> String {
    "portrait".to_string()
}

impl Default for PdfAutoRotateOptions {
    fn default() -> Self {
        Self {
            target_orientation: default_target_orientation(),
            pages: None,
        }
    }
}

fn rotate_page_dict(page_dict: &mut lopdf::Dictionary, options: &PdfAutoRotateOptions) {
    // Extract MediaBox (inherited or directly on page)
    let media_box = if let Ok(lopdf::Object::Array(arr)) = page_dict.get(b"MediaBox") {
        arr.clone()
    } else {
        // Fallback to Letter [0, 0, 612, 792] for testing/robustness if not found
        vec![
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(612.0),
            lopdf::Object::Real(792.0),
        ]
    };

    let mut llx = 0.0;
    let mut lly = 0.0;
    let mut urx = 0.0;
    let mut ury = 0.0;

    if media_box.len() >= 4 {
        let get_num = |obj: &lopdf::Object| -> f64 {
            match obj {
                lopdf::Object::Integer(i) => *i as f64,
                lopdf::Object::Real(f) => *f as f64,
                _ => 0.0,
            }
        };
        llx = get_num(&media_box[0]);
        lly = get_num(&media_box[1]);
        urx = get_num(&media_box[2]);
        ury = get_num(&media_box[3]);
    }

    let width = (urx - llx).abs();
    let height = (ury - lly).abs();

    let current_rotation = if let Ok(lopdf::Object::Integer(r)) = page_dict.get(b"Rotate") {
        *r
    } else {
        0
    };
    let normalized_rotation = ((current_rotation % 360) + 360) % 360;

    let (w_eff, h_eff) = if normalized_rotation == 90 || normalized_rotation == 270 {
        (height, width)
    } else {
        (width, height)
    };

    let target = options.target_orientation.as_str();

    if target == "portrait" || target == "auto" {
        if w_eff > h_eff {
            let new_rotation = (normalized_rotation + 90) % 360;
            page_dict.set("Rotate", lopdf::Object::Integer(new_rotation));
        }
    } else if target == "landscape" && h_eff > w_eff {
        let new_rotation = (normalized_rotation + 90) % 360;
        page_dict.set("Rotate", lopdf::Object::Integer(new_rotation));
    }
}

pub fn pdf_auto_rotate_bytes(
    input_bytes: &[u8],
    options: &PdfAutoRotateOptions,
) -> Result<Vec<u8>, PdfError> {
    let mut doc =
        lopdf::Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let pages = doc.get_pages();

    for (page_num, page_id) in pages {
        if options
            .pages
            .as_ref()
            .is_some_and(|target_pages| !target_pages.contains(&page_num))
        {
            continue;
        }

        let page_dict = match doc.get_object_mut(page_id) {
            Ok(lopdf::Object::Dictionary(d)) => d,
            _ => continue,
        };

        rotate_page_dict(page_dict, options);
    }

    let mut output = Vec::new();
    doc.save_to(&mut output).map_err(PdfError::IoError)?;

    Ok(output)
}

#[derive(Default)]
pub struct PdfAutoRotateOperation {
    options: PdfAutoRotateOptions,
}

impl PdfAutoRotateOperation {
    pub fn new(options: PdfAutoRotateOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfAutoRotateOperation {
    fn execute(
        &self,
        document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        let any_doc = document.as_any_mut();

        if let Some(lopdf_doc) = any_doc.downcast_mut::<crate::document::LopdfDocument>() {
            let doc = &mut lopdf_doc.inner;
            let pages = doc.get_pages();

            for (page_num, page_id) in pages {
                if self
                    .options
                    .pages
                    .as_ref()
                    .is_some_and(|target_pages| !target_pages.contains(&page_num))
                {
                    continue;
                }

                let page_dict = match doc.get_object_mut(page_id) {
                    Ok(lopdf::Object::Dictionary(d)) => d,
                    _ => continue,
                };

                rotate_page_dict(page_dict, &self.options);
            }
            Ok(())
        } else {
            Err(PdfError::UnsupportedOperation(
                "Only LopdfDocument is supported".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_doc(media_box: Vec<lopdf::Object>) -> Vec<u8> {
        let mut doc = lopdf::Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut page_dict = lopdf::Dictionary::new();
        page_dict.set("Type", lopdf::Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", lopdf::Object::Reference(pages_id));
        page_dict.set("MediaBox", lopdf::Object::Array(media_box));

        let page_id = doc.add_object(page_dict);

        let mut pages_dict = lopdf::Dictionary::new();
        pages_dict.set("Type", lopdf::Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", lopdf::Object::Integer(1));
        pages_dict.set(
            "Kids",
            lopdf::Object::Array(vec![lopdf::Object::Reference(page_id)]),
        );

        doc.objects
            .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

        let mut catalog = lopdf::Dictionary::new();
        catalog.set("Type", lopdf::Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", lopdf::Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);

        doc.trailer
            .set("Root", lopdf::Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn test_auto_rotate_landscape_to_portrait() {
        // Landscape MediaBox [0, 0, 842, 595]
        let media_box = vec![
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(842.0),
            lopdf::Object::Real(595.0),
        ];
        let bytes = create_test_doc(media_box);

        let options = PdfAutoRotateOptions {
            target_orientation: "portrait".to_string(),
            pages: None,
        };

        let result_bytes = pdf_auto_rotate_bytes(&bytes, &options).unwrap();

        let result_doc = lopdf::Document::load_mem(&result_bytes).unwrap();
        let pages = result_doc.get_pages();
        let page_id = pages.get(&1).unwrap();

        if let lopdf::Object::Dictionary(dict) = result_doc.get_object(*page_id).unwrap() {
            let rotate = dict.get(b"Rotate").unwrap();
            if let lopdf::Object::Integer(r) = rotate {
                assert_eq!(*r, 90);
            } else {
                panic!("Rotate is not an integer");
            }
        } else {
            panic!("Page is not a dictionary");
        }
    }

    #[test]
    fn test_auto_rotate_already_portrait() {
        // Portrait MediaBox [0, 0, 595, 842]
        let media_box = vec![
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(0.0),
            lopdf::Object::Real(595.0),
            lopdf::Object::Real(842.0),
        ];
        let bytes = create_test_doc(media_box);

        let options = PdfAutoRotateOptions {
            target_orientation: "portrait".to_string(),
            pages: None,
        };

        let result_bytes = pdf_auto_rotate_bytes(&bytes, &options).unwrap();

        let result_doc = lopdf::Document::load_mem(&result_bytes).unwrap();
        let pages = result_doc.get_pages();
        let page_id = pages.get(&1).unwrap();

        if let lopdf::Object::Dictionary(dict) = result_doc.get_object(*page_id).unwrap() {
            assert!(
                dict.get(b"Rotate").is_err(),
                "Rotate should not be set for already portrait"
            );
        } else {
            panic!("Page is not a dictionary");
        }
    }

    #[test]
    fn test_auto_rotate_empty_document() {
        let options = PdfAutoRotateOptions::default();
        let bytes = vec![];
        let result = pdf_auto_rotate_bytes(&bytes, &options);
        assert!(result.is_err());
        match result {
            Err(PdfError::ParseError(_)) => (),
            _ => panic!("Expected ParseError"),
        }
    }
}
