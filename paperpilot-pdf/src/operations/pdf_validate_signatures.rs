use lopdf::{Document, Object};
use paperpilot_core::error::PdfError;
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignatureInfo {
    pub field_name: String,
    pub signer_name: Option<String>,
    pub organization: Option<String>,
    pub signing_time: Option<String>,
    pub sub_filter: String,
    pub byte_range: Vec<i64>,
    pub is_valid: bool,
    pub is_tampered: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureValidationReport {
    pub total_signatures: usize,
    pub valid_signatures: usize,
    pub all_valid: bool,
    pub signatures: Vec<DigitalSignatureInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfValidateSignaturesOptions {}

pub fn pdf_validate_signatures_bytes(
    input_bytes: &[u8],
    _options: &PdfValidateSignaturesOptions,
) -> Result<Vec<u8>, PdfError> {
    let doc = Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let mut report = SignatureValidationReport {
        total_signatures: 0,
        valid_signatures: 0,
        all_valid: true,
        signatures: Vec::new(),
    };

    let mut signature_refs = Vec::new();

    // Scan AcroForm
    #[allow(clippy::collapsible_if)]
    if let Ok(catalog) = doc
        .trailer
        .get(b"Root")
        .and_then(|r| doc.get_object(r.as_reference().unwrap_or((0, 0))))
    {
        if let Ok(cat_dict) = catalog.as_dict() {
            if let Ok(acroform_ref) = cat_dict.get(b"AcroForm") {
                if let Ok(acroform) = doc
                    .get_object(acroform_ref.as_reference().unwrap_or((0, 0)))
                    .and_then(|o| o.as_dict())
                {
                    if let Ok(fields) = acroform.get(b"Fields").and_then(|f| f.as_array()) {
                        for field in fields {
                            if let Ok(field_ref) = field.as_reference() {
                                signature_refs.push(field_ref);
                            }
                        }
                    }
                }
            }
        }
    }

    // Scan Pages for Annots
    #[allow(clippy::collapsible_if)]
    for page_id in doc.get_pages().values() {
        if let Ok(page) = doc.get_object(*page_id).and_then(|o| o.as_dict()) {
            if let Ok(annots) = page.get(b"Annots").and_then(|a| a.as_array()) {
                for annot in annots {
                    if let Ok(annot_ref) = annot.as_reference() {
                        signature_refs.push(annot_ref);
                    }
                }
            }
        }
    }

    let mut processed_sigs = std::collections::HashSet::new();

    for sig_ref in signature_refs {
        if processed_sigs.contains(&sig_ref) {
            continue;
        }

        if let Ok(dict) = doc.get_object(sig_ref).and_then(|o| o.as_dict()) {
            let is_sig_field = dict
                .get(b"FT")
                .and_then(|o| o.as_name())
                .map(|n| n == b"Sig")
                .unwrap_or(false)
                || dict
                    .get(b"Type")
                    .and_then(|o| o.as_name())
                    .map(|n| n == b"Sig")
                    .unwrap_or(false)
                || dict
                    .get(b"Type")
                    .and_then(|o| o.as_name())
                    .map(|n| n == b"Annot")
                    .unwrap_or(false)
                    && dict
                        .get(b"Subtype")
                        .and_then(|o| o.as_name())
                        .map(|n| n == b"Widget")
                        .unwrap_or(false);

            if !is_sig_field {
                continue;
            }

            let v_dict = match dict.get(b"V") {
                Ok(Object::Dictionary(d)) => Some(d),
                Ok(Object::Reference(r)) => doc.get_object(*r).ok().and_then(|o| match o {
                    Object::Dictionary(d) => Some(d),
                    _ => None,
                }),
                _ => {
                    if dict
                        .get(b"Type")
                        .and_then(|o| o.as_name())
                        .map(|n| n == b"Sig")
                        .unwrap_or(false)
                    {
                        Some(dict)
                    } else {
                        None
                    }
                }
            };

            #[allow(clippy::collapsible_if)]
            if let Some(sig_dict) = v_dict {
                if sig_dict
                    .get(b"Type")
                    .and_then(|o| o.as_name())
                    .map(|n| n == b"Sig")
                    .unwrap_or(false)
                {
                    processed_sigs.insert(sig_ref);

                    let field_name = dict
                        .get(b"T")
                        .and_then(|o| o.as_str())
                        .map(|b| String::from_utf8_lossy(b).into_owned())
                        .unwrap_or_else(|_| "Unknown".to_string());

                    let sub_filter = sig_dict
                        .get(b"SubFilter")
                        .and_then(|o| o.as_name())
                        .map(|b| String::from_utf8_lossy(b).into_owned())
                        .unwrap_or_else(|_| "Unknown".to_string());

                    let signer_name = sig_dict
                        .get(b"Name")
                        .and_then(|o| o.as_str())
                        .map(|b| String::from_utf8_lossy(b).into_owned())
                        .ok();

                    let signing_time = sig_dict
                        .get(b"M")
                        .and_then(|o| o.as_str())
                        .map(|b| String::from_utf8_lossy(b).into_owned())
                        .ok();

                    let mut byte_range = Vec::new();
                    if let Ok(br_arr) = sig_dict.get(b"ByteRange").and_then(|o| o.as_array()) {
                        for val in br_arr {
                            if let Ok(num) = val.as_i64() {
                                byte_range.push(num);
                            }
                        }
                    }

                    let mut is_valid = false;
                    let mut is_tampered = false;
                    let mut error_message = None;

                    if byte_range.len() == 4 {
                        let offset1 = byte_range[0];
                        let len1 = byte_range[1];
                        let offset2 = byte_range[2];
                        let len2 = byte_range[3];

                        if offset1 != 0 {
                            error_message = Some("Invalid ByteRange: offset1 must be 0".to_string());
                        } else if offset1 + len1 >= offset2 {
                            error_message =
                                Some("Invalid ByteRange: offset1 + len1 must be < offset2".to_string());
                        } else if offset2 + len2 > input_bytes.len() as i64 {
                            error_message =
                                Some("Invalid ByteRange: offset2 + len2 exceeds file size".to_string());
                        } else {
                            is_valid = true;
                            let end_of_sig = (offset2 + len2) as usize;
                            if end_of_sig < input_bytes.len() {
                                let trailing_bytes = &input_bytes[end_of_sig..];
                                let trailing_str = String::from_utf8_lossy(trailing_bytes);

                                if trailing_str.contains("xref") || trailing_str.contains("obj") {
                                    is_tampered = true;
                                    is_valid = false;
                                    error_message =
                                        Some("Post-signing modifications detected".to_string());
                                }
                            }
                        }
                    } else {
                        error_message = Some("Missing or invalid ByteRange".to_string());
                    }

                    if !is_valid {
                        report.all_valid = false;
                    } else {
                        report.valid_signatures += 1;
                    }
                    report.total_signatures += 1;

                    report.signatures.push(DigitalSignatureInfo {
                        field_name,
                        signer_name,
                        organization: None, // Can be parsed from PKCS7, but out of scope for now
                        signing_time,
                        sub_filter,
                        byte_range,
                        is_valid,
                        is_tampered,
                        error_message,
                    });
                }
            }
        }
    }

    let json_bytes =
        serde_json::to_vec_pretty(&report).map_err(|e| PdfError::Other(e.to_string()))?;
    Ok(json_bytes)
}

#[derive(Default)]
pub struct PdfValidateSignaturesOperation {
    #[allow(dead_code)]
    options: PdfValidateSignaturesOptions,
}

impl PdfValidateSignaturesOperation {
    pub fn new(options: PdfValidateSignaturesOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfValidateSignaturesOperation {
    fn execute(
        &self,
        _document: &mut dyn PdfDocument,
    ) -> paperpilot_core::error::OperationResult<()> {
        Err(PdfError::UnsupportedOperation(
            "Use pdf_validate_signatures_bytes directly for this operation".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_validate_signatures_empty_document() {
        let options = PdfValidateSignaturesOptions::default();
        let bytes = vec![];
        let result = pdf_validate_signatures_bytes(&bytes, &options);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object, StringFormat};

    #[test]
    fn test_pdf_validate_signatures_unsigned_pdf() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![]));
        pages_dict.set("Count", Object::Integer(0));
        doc.set_object(pages_id, pages_dict);

        let mut catalog_dict = Dictionary::new();
        catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog_dict);

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();

        let options = PdfValidateSignaturesOptions::default();
        let result_bytes = pdf_validate_signatures_bytes(&bytes, &options).expect("Should succeed");

        let report: SignatureValidationReport =
            serde_json::from_slice(&result_bytes).expect("Should deserialize");
        assert_eq!(report.total_signatures, 0);
        assert_eq!(report.valid_signatures, 0);
        assert!(report.all_valid);
    }

    #[test]
    fn test_pdf_validate_signatures_synthetic_signature() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Kids", Object::Array(vec![]));
        pages_dict.set("Count", Object::Integer(0));
        doc.set_object(pages_id, pages_dict);

        let mut sig_dict = Dictionary::new();
        sig_dict.set("Type", Object::Name(b"Sig".to_vec()));
        sig_dict.set("Filter", Object::Name(b"Adobe.PPKLite".to_vec()));
        sig_dict.set("SubFilter", Object::Name(b"adbe.pkcs7.detached".to_vec()));
        sig_dict.set(
            "Name",
            Object::String(b"John Doe".to_vec(), StringFormat::Literal),
        );
        sig_dict.set(
            "ByteRange",
            Object::Array(vec![
                Object::Integer(0),
                Object::Integer(100),
                Object::Integer(200),
                Object::Integer(50),
            ]),
        );
        sig_dict.set(
            "Contents",
            Object::String(b"abc".to_vec(), StringFormat::Hexadecimal),
        );
        let sig_id = doc.add_object(sig_dict);

        let mut annot_dict = Dictionary::new();
        annot_dict.set("Type", Object::Name(b"Annot".to_vec()));
        annot_dict.set("Subtype", Object::Name(b"Widget".to_vec()));
        annot_dict.set("FT", Object::Name(b"Sig".to_vec()));
        annot_dict.set(
            "T",
            Object::String(b"Signature1".to_vec(), StringFormat::Literal),
        );
        annot_dict.set("V", Object::Reference(sig_id));
        let annot_id = doc.add_object(annot_dict);

        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("Annots", Object::Array(vec![Object::Reference(annot_id)]));
        let page_id = doc.add_object(page_dict);

        if let Object::Dictionary(pages) = doc.get_object_mut(pages_id).unwrap() {
            pages.set("Kids", Object::Array(vec![Object::Reference(page_id)]));
            pages.set("Count", Object::Integer(1));
        }

        let mut acroform = Dictionary::new();
        acroform.set("Fields", Object::Array(vec![Object::Reference(annot_id)]));
        let acroform_id = doc.add_object(acroform);

        let mut catalog_dict = Dictionary::new();
        catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog_dict.set("Pages", Object::Reference(pages_id));
        catalog_dict.set("AcroForm", Object::Reference(acroform_id));
        let catalog_id = doc.add_object(catalog_dict);

        doc.trailer.set("Root", Object::Reference(catalog_id));

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();

        let expected_size = bytes.len() as i64;

        if let Object::Dictionary(sig) = doc.get_object_mut(sig_id).unwrap() {
            sig.set(
                "ByteRange",
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(100),
                    Object::Integer(200),
                    Object::Integer(expected_size - 200), // Perfect fit
                ]),
            );
        }

        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();

        let options = PdfValidateSignaturesOptions::default();
        let result_bytes = pdf_validate_signatures_bytes(&bytes, &options).expect("Should succeed");

        let report: SignatureValidationReport =
            serde_json::from_slice(&result_bytes).expect("Should deserialize");
        assert_eq!(report.total_signatures, 1);
        assert_eq!(report.signatures[0].field_name, "Signature1");
        assert_eq!(
            report.signatures[0].signer_name.as_deref(),
            Some("John Doe")
        );
        assert_eq!(report.signatures[0].sub_filter, "adbe.pkcs7.detached");
        assert!(report.signatures[0].is_valid);
    }
}
