use lopdf::{Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfRemoveSignaturesOptions {
    #[serde(default = "default_true")]
    pub remove_empty_signature_fields: bool,
}

impl Default for PdfRemoveSignaturesOptions {
    fn default() -> Self {
        Self {
            remove_empty_signature_fields: true,
        }
    }
}

fn default_true() -> bool {
    true
}

pub fn pdf_remove_signatures_bytes(
    input_bytes: &[u8],
    options: &PdfRemoveSignaturesOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty input bytes",
        )));
    }

    let mut doc =
        Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let mut sig_object_ids_to_remove = Vec::new();
    let mut sig_field_refs = Vec::new();

    // 1. Process AcroForm and find signatures
    let catalog = doc.catalog().cloned();

    if let Ok(catalog) = catalog {
        let acroform_ref_or_dict = catalog.get(b"AcroForm").cloned();

        let mut acroform_id_to_update = None;
        let mut acroform_dict_to_update = None;

        if let Ok(acroform_val) = acroform_ref_or_dict {
            if let Object::Reference(id) = acroform_val {
                if let Ok(Object::Dictionary(acroform_dict)) = doc.get_object(id) {
                    acroform_id_to_update = Some(id);
                    acroform_dict_to_update = Some(acroform_dict.clone());
                }
            } else if let Object::Dictionary(acroform_dict) = acroform_val {
                acroform_dict_to_update = Some(acroform_dict);
            }
        }

        if let Some(mut acroform_dict) = acroform_dict_to_update
            && let Ok(Object::Array(fields)) = acroform_dict.get(b"Fields")
        {
            let mut new_fields = Vec::new();
            for field_obj in fields {
                let mut is_sig = false;
                if let Object::Reference(field_id) = field_obj {
                    if let Ok(Object::Dictionary(field_dict)) = doc.get_object(*field_id) {
                        if let Ok(Object::Name(ft)) = field_dict.get(b"FT")
                            && ft == b"Sig"
                        {
                            is_sig = true;
                        }

                        if let Ok(v_obj) = field_dict.get(b"V") {
                            let v_dict_res = match v_obj {
                                Object::Reference(v_id) => {
                                    if let Ok(Object::Dictionary(d)) = doc.get_object(*v_id) {
                                        sig_object_ids_to_remove.push(*v_id);
                                        Some(d)
                                    } else {
                                        None
                                    }
                                }
                                Object::Dictionary(d) => Some(d),
                                _ => None,
                            };

                            if let Some(v_dict) = v_dict_res
                                && let Ok(Object::Name(v_type)) = v_dict.get(b"Type")
                                && v_type == b"Sig"
                            {
                                is_sig = true;
                            }
                        }
                    }

                    if is_sig {
                        sig_field_refs.push(*field_id);
                        sig_object_ids_to_remove.push(*field_id);
                    } else {
                        new_fields.push(field_obj.clone());
                    }
                } else if let Object::Dictionary(field_dict) = field_obj {
                    if let Ok(Object::Name(ft)) = field_dict.get(b"FT")
                        && ft == b"Sig"
                    {
                        is_sig = true;
                    }

                    if let Ok(v_obj) = field_dict.get(b"V") {
                        let v_dict_res = match v_obj {
                            Object::Reference(v_id) => {
                                if let Ok(Object::Dictionary(d)) = doc.get_object(*v_id) {
                                    sig_object_ids_to_remove.push(*v_id);
                                    Some(d)
                                } else {
                                    None
                                }
                            }
                            Object::Dictionary(d) => Some(d),
                            _ => None,
                        };

                        if let Some(v_dict) = v_dict_res
                            && let Ok(Object::Name(v_type)) = v_dict.get(b"Type")
                            && v_type == b"Sig"
                        {
                            is_sig = true;
                        }
                    }

                    if !is_sig {
                        new_fields.push(field_obj.clone());
                    }
                } else {
                    new_fields.push(field_obj.clone());
                }
            }

            if options.remove_empty_signature_fields {
                acroform_dict.set(b"Fields".to_vec(), Object::Array(new_fields.clone()));
                acroform_dict.remove(b"SigFlags");

                if let Some(id) = acroform_id_to_update {
                    doc.objects.insert(id, Object::Dictionary(acroform_dict));
                } else {
                    if let Ok(catalog_mut) = doc.catalog_mut() {
                        if new_fields.is_empty() {
                            catalog_mut.remove(b"AcroForm");
                        } else {
                            catalog_mut
                                .set(b"AcroForm".to_vec(), Object::Dictionary(acroform_dict));
                        }
                    }
                }
            }
        }
    }

    // 2. Remove signatures from page Annots
    let page_ids = doc.get_pages();
    for (_, page_id) in page_ids {
        // First, read the annotations we need to remove
        let mut annots_to_keep = Vec::new();
        let mut has_annots = false;

        if let Ok(Object::Dictionary(page_dict)) = doc.get_object(page_id)
            && let Ok(Object::Array(annots)) = page_dict.get(b"Annots")
        {
            has_annots = true;
            for annot_obj in annots {
                let mut keep = true;
                if let Object::Reference(annot_id) = annot_obj {
                    if sig_field_refs.contains(annot_id) {
                        keep = false;
                    } else if let Ok(Object::Dictionary(annot_dict)) = doc.get_object(*annot_id)
                        && let Ok(Object::Name(subtype)) = annot_dict.get(b"Subtype")
                        && subtype == b"Widget"
                        && let Ok(Object::Name(ft)) = annot_dict.get(b"FT")
                        && ft == b"Sig"
                    {
                        keep = false;
                        sig_object_ids_to_remove.push(*annot_id);
                    }
                } else if let Object::Dictionary(annot_dict) = annot_obj
                    && let Ok(Object::Name(subtype)) = annot_dict.get(b"Subtype")
                    && subtype == b"Widget"
                    && let Ok(Object::Name(ft)) = annot_dict.get(b"FT")
                    && ft == b"Sig"
                {
                    keep = false;
                }

                if keep {
                    annots_to_keep.push(annot_obj.clone());
                }
            }
        }

        // Then, update the page object
        if has_annots && let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            if annots_to_keep.is_empty() {
                page_dict.remove(b"Annots");
            } else {
                page_dict.set(b"Annots".to_vec(), Object::Array(annots_to_keep));
            }
        }
    }

    // 3. Delete collected objects
    for id in sig_object_ids_to_remove {
        doc.objects.remove(&id);
    }

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buf)
}

#[derive(Default)]
pub struct PdfRemoveSignaturesOperation {
    _options: PdfRemoveSignaturesOptions,
}

impl PdfRemoveSignaturesOperation {
    pub fn new(options: PdfRemoveSignaturesOptions) -> Self {
        Self { _options: options }
    }
}

impl PdfOperation for PdfRemoveSignaturesOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Streaming operation not implemented; use bytes API".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, dictionary};

    #[test]
    fn test_remove_signatures_empty_bytes() {
        let options = PdfRemoveSignaturesOptions::default();
        let bytes = vec![];
        let result = pdf_remove_signatures_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_signatures_from_unsigned_pdf() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });
        let content = lopdf::content::Content { operations: vec![] };
        let content_id = doc.add_object(lopdf::Stream::new(
            dictionary! {},
            content.encode().unwrap(),
        ));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();

        let options = PdfRemoveSignaturesOptions::default();
        let result = pdf_remove_signatures_bytes(&buf, &options);
        assert!(result.is_ok());

        let out_buf = result.unwrap();
        let out_doc = Document::load_mem(&out_buf).unwrap();
        let pages = out_doc.get_pages();
        assert_eq!(pages.len(), 1);
    }

    #[test]
    fn test_remove_signatures_from_signed_pdf() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let sig_dict_id = doc.add_object(dictionary! {
            "Type" => "Sig",
            "Filter" => "Adobe.PPKLite",
            "SubFilter" => "adbe.pkcs7.detached",
        });

        let annot_id = doc.add_object(dictionary! {
            "Type" => "Annot",
            "Subtype" => "Widget",
            "FT" => "Sig",
            "V" => sig_dict_id,
            "Rect" => vec![0.into(), 0.into(), 100.into(), 100.into()],
        });

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            "Annots" => vec![annot_id.into()],
        });

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let acroform_id = doc.add_object(dictionary! {
            "Fields" => vec![annot_id.into()],
            "SigFlags" => 3,
        });

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
            "AcroForm" => acroform_id,
        });
        doc.trailer.set("Root", catalog_id);

        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();

        let options = PdfRemoveSignaturesOptions::default();
        let result = pdf_remove_signatures_bytes(&buf, &options);
        assert!(result.is_ok());

        let out_buf = result.unwrap();
        let out_doc = Document::load_mem(&out_buf).unwrap();

        let out_catalog = out_doc.catalog().unwrap();
        let acroform = out_catalog.get(b"AcroForm");
        if let Ok(lopdf::Object::Reference(id)) = acroform {
            if let Ok(lopdf::Object::Dictionary(dict)) = out_doc.get_object(*id) {
                if let Ok(lopdf::Object::Array(fields)) = dict.get(b"Fields") {
                    assert!(fields.is_empty());
                } else {
                    assert!(false, "Fields not an array or missing");
                }
            }
        } else if acroform.is_ok() {
            assert!(false, "AcroForm should be a reference or removed");
        }

        let pages = out_doc.get_pages();
        assert_eq!(pages.len(), 1);
        let first_page_id = pages.values().next().unwrap();
        if let Object::Dictionary(page_dict) = out_doc.get_object(*first_page_id).unwrap() {
            assert!(page_dict.get(b"Annots").is_err()); // Annotations should be empty and removed
        }
    }
}
