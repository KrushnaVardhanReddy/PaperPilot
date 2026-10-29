//! Local document facts shared by workflow conditions and desktop smart filters.
use lopdf::{Document, Object};
use paperpilot_core::error::{OperationResult, PdfError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentInspection {
    pub page_count: Option<u32>,
    pub is_scanned: Option<bool>,
    pub has_form_fields: Option<bool>,
    /// Presence of a filled digital signature, not a trust/validity verdict.
    pub is_signed: Option<bool>,
    pub is_encrypted: bool,
    pub size_bytes: u64,
}

pub fn inspect_document_bytes(bytes: &[u8]) -> OperationResult<DocumentInspection> {
    let doc = Document::load_mem(bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;
    if doc.is_encrypted() {
        return Ok(DocumentInspection {
            page_count: None,
            is_scanned: None,
            has_form_fields: None,
            is_signed: None,
            is_encrypted: true,
            size_bytes: bytes.len() as u64,
        });
    }
    let pages = doc.get_pages();
    if pages.is_empty() {
        return Err(PdfError::InvalidInput("Document has no pages".into()));
    }
    Ok(DocumentInspection {
        page_count: Some(pages.len() as u32),
        is_scanned: Some(super::workflow::inspect_scanned(&doc)?),
        has_form_fields: Some(has_form_fields(&doc)?),
        is_signed: Some(has_signature(&doc)?),
        is_encrypted: doc.was_encrypted(),
        size_bytes: bytes.len() as u64,
    })
}

pub(super) fn has_form_fields(doc: &Document) -> OperationResult<bool> {
    let catalog = doc
        .catalog()
        .map_err(|e| PdfError::ParseError(e.to_string()))?;
    let Ok(acroform) = catalog.get(b"AcroForm") else {
        return Ok(false);
    };
    let fields = doc
        .dereference(acroform)
        .and_then(|(_, o)| o.as_dict())
        .and_then(|d| d.get(b"Fields"));
    match fields {
        Ok(fields) => doc
            .dereference(fields)
            .and_then(|(_, o)| o.as_array())
            .map(|a| !a.is_empty())
            .map_err(|e| PdfError::ParseError(e.to_string())),
        Err(lopdf::Error::DictKey(_)) => Ok(false),
        Err(e) => Err(PdfError::ParseError(e.to_string())),
    }
}

pub(super) fn has_signature(doc: &Document) -> OperationResult<bool> {
    let mut stack = Vec::new();
    let mut seen = HashSet::new();
    let catalog = doc
        .catalog()
        .map_err(|e| PdfError::ParseError(e.to_string()))?;
    if let Ok(acroform) = catalog.get(b"AcroForm") {
        let dict = doc
            .dereference(acroform)
            .and_then(|(_, o)| o.as_dict())
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        if let Ok(fields) = dict.get(b"Fields") {
            stack.push((fields.clone(), 0usize));
        }
    }
    if let Ok(perms) = catalog
        .get(b"Perms")
        .and_then(|o| doc.dereference(o))
        .and_then(|(_, o)| o.as_dict())
    {
        if let Ok(signature) = perms.get(b"DocMDP") {
            stack.push((signature.clone(), 0));
        }
    }
    for page in doc.get_pages().values() {
        if let Ok(annotations) = doc.get_dictionary(*page).and_then(|d| d.get(b"Annots")) {
            stack.push((annotations.clone(), 0));
        }
    }
    while let Some((object, depth)) = stack.pop() {
        if depth > 64 {
            return Err(PdfError::ParseError(
                "Form tree nesting limit exceeded".into(),
            ));
        }
        if let Object::Reference(id) = object {
            if !seen.insert(id) {
                continue;
            }
        }
        let (_, object) = doc
            .dereference(&object)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        match object {
            Object::Array(items) => {
                for item in items {
                    stack.push((item.clone(), depth + 1));
                }
            }
            Object::Dictionary(dict) => {
                // SigFlags and an empty /FT /Sig widget are only placeholders.
                if dict
                    .get(b"ByteRange")
                    .and_then(Object::as_array)
                    .is_ok_and(|ranges| {
                        let Ok(ranges) = ranges
                            .iter()
                            .map(Object::as_i64)
                            .collect::<Result<Vec<_>, _>>()
                        else {
                            return false;
                        };
                        ranges.len() == 4
                            && ranges[0] == 0
                            && ranges[1] > 0
                            && ranges[2] > ranges[1]
                            && ranges[3] > 0
                    })
                    && dict
                        .get(b"Contents")
                        .and_then(Object::as_str)
                        .is_ok_and(|s| s.iter().any(|b| *b != 0))
                {
                    return Ok(true);
                }
                for key in [b"V".as_slice(), b"Kids".as_slice()] {
                    if let Ok(child) = dict.get(key) {
                        stack.push((child.clone(), depth + 1));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
