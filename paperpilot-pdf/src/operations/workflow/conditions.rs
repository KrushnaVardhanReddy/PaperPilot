use lopdf::{Dictionary, Document, Object, ObjectId, content::Content};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{PageCountComparator, WorkflowCondition};
use std::collections::HashSet;

pub(super) fn load_pdf(bytes: &[u8]) -> OperationResult<Document> {
    let doc = Document::load_mem(bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;
    if doc.is_encrypted() {
        return Err(PdfError::InvalidInput(
            "Workflow input must be decrypted first".into(),
        ));
    }
    Ok(doc)
}

pub(super) fn evaluate(
    bytes: &[u8],
    condition: &WorkflowCondition,
) -> OperationResult<(bool, String)> {
    let doc = load_pdf(bytes)?;
    let count = doc.get_pages().len();
    let mut matches = true;
    let mut reasons = vec![format!("page_count={count}")];
    if let Some(threshold) = condition.if_page_count_gt {
        matches &= count > threshold as usize;
        reasons.push(format!("if_page_count_gt={threshold}"));
    }
    if let (Some(comparator), Some(value)) = (condition.comparator, condition.value) {
        let value = value as usize;
        matches &= match comparator {
            PageCountComparator::Greater => count > value,
            PageCountComparator::GreaterOrEqual => count >= value,
            PageCountComparator::Equal => count == value,
            PageCountComparator::Less => count < value,
            PageCountComparator::LessOrEqual => count <= value,
        };
        reasons.push(format!("{comparator:?} {value}"));
    }
    if let Some(expected) = condition.if_scanned {
        let scanned = is_scanned(&doc)?;
        matches &= scanned == expected;
        reasons.push(format!("scanned={scanned}, expected={expected}"));
    }
    Ok((matches, reasons.join("; ")))
}

// Conservative, local heuristic: at least one page paints a raster image and
// has no extractable non-whitespace text. Blank/vector-only pages are not scans.
// A mixed document counts as scanned if any page needs OCR. This is not a model.
fn is_scanned(doc: &Document) -> OperationResult<bool> {
    for (number, page) in doc.get_pages() {
        let text = doc
            .extract_text_with_limit(&[number], 16 * 1024 * 1024)
            .map_err(|e| PdfError::ParseError(format!("Cannot inspect page text: {e}")))?;
        if text.chars().any(|c| !c.is_whitespace()) {
            continue;
        }
        let resources = inherited_resources(doc, page)?;
        let content = doc
            .get_page_content_with_limit(page, 16 * 1024 * 1024)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        if paints_image(doc, &content, &resources, &mut HashSet::new(), 0)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn inherited_resources(doc: &Document, mut id: ObjectId) -> OperationResult<Dictionary> {
    let mut seen = HashSet::new();
    for _ in 0..64 {
        if !seen.insert(id) {
            break;
        }
        let dict = doc
            .get_dictionary(id)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;
        if let Ok(resources) = dict.get(b"Resources") {
            return doc
                .dereference(resources)
                .and_then(|(_, r)| r.as_dict())
                .cloned()
                .map_err(|e| PdfError::ParseError(e.to_string()));
        }
        match dict.get(b"Parent").and_then(Object::as_reference) {
            Ok(parent) => id = parent,
            Err(_) => return Ok(Dictionary::new()),
        }
    }
    Err(PdfError::ParseError(
        "Cyclic or excessively deep page resources".into(),
    ))
}

fn paints_image(
    doc: &Document,
    bytes: &[u8],
    resources: &Dictionary,
    seen: &mut HashSet<ObjectId>,
    depth: usize,
) -> OperationResult<bool> {
    if depth > 32 {
        return Err(PdfError::ParseError(
            "Form XObject nesting limit exceeded".into(),
        ));
    }
    let content = Content::decode(bytes)
        .map_err(|e| PdfError::ParseError(format!("Cannot inspect page content: {e}")))?;
    for op in content.operations {
        if op.operator == "BI" {
            return Ok(true);
        }
        if op.operator != "Do" {
            continue;
        }
        let name = op
            .operands
            .first()
            .and_then(|v| v.as_name().ok())
            .ok_or_else(|| PdfError::ParseError("Invalid image/Form invocation".into()))?;
        let parse =
            |e: lopdf::Error| PdfError::ParseError(format!("Cannot inspect image/Form: {e}"));
        let objects = doc
            .dereference(resources.get(b"XObject").map_err(parse)?)
            .map_err(parse)?
            .1
            .as_dict()
            .map_err(parse)?;
        let (id, resolved) = doc
            .dereference(objects.get(name).map_err(parse)?)
            .map_err(parse)?;
        let stream = resolved.as_stream().map_err(parse)?;
        match stream
            .dict
            .get(b"Subtype")
            .and_then(Object::as_name)
            .map_err(parse)?
        {
            b"Image" => return Ok(true),
            b"Form" => {
                if let Some(id) = id {
                    if !seen.insert(id) {
                        return Err(PdfError::ParseError("Cyclic Form XObject".into()));
                    }
                }
                let nested = match stream.dict.get(b"Resources") {
                    Ok(r) => doc
                        .dereference(r)
                        .map_err(parse)?
                        .1
                        .as_dict()
                        .map_err(parse)?,
                    Err(_) => resources,
                };
                let bytes = if stream.dict.has(b"Filter") {
                    stream
                        .decompressed_content_with_limit(16 * 1024 * 1024)
                        .map_err(parse)?
                } else if stream.content.len() <= 16 * 1024 * 1024 {
                    stream.content.clone()
                } else {
                    return Err(PdfError::ParseError(
                        "Form content exceeds 16 MiB inspection limit".into(),
                    ));
                };
                let found = paints_image(doc, &bytes, nested, seen, depth + 1)?;
                if let Some(id) = id {
                    seen.remove(&id);
                }
                if found {
                    return Ok(true);
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
