use lopdf::{Dictionary, Document, Object, ObjectId, content::Content};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::workflow::{
    LeafCondition, NumericComparator, PageCountComparator, WorkflowCondition,
};
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
    evaluate_node(bytes, &doc, condition)
}

fn evaluate_node(
    bytes: &[u8],
    doc: &Document,
    condition: &WorkflowCondition,
) -> OperationResult<(bool, String)> {
    match condition {
        WorkflowCondition::All { all } => {
            if all.is_empty() {
                return Ok((true, "all: [] (vacuous truth)".into()));
            }
            let mut reasons = Vec::new();
            for (idx, child) in all.iter().enumerate() {
                let (matched, reason) = evaluate_node(bytes, doc, child)?;
                reasons.push(format!("all[{idx}]: {reason}"));
                if !matched {
                    return Ok((false, reasons.join("; ")));
                }
            }
            Ok((true, reasons.join("; ")))
        }
        WorkflowCondition::Any { any } => {
            if any.is_empty() {
                return Ok((false, "any: [] (empty disjunction)".into()));
            }
            let mut reasons = Vec::new();
            for (idx, child) in any.iter().enumerate() {
                let (matched, reason) = evaluate_node(bytes, doc, child)?;
                reasons.push(format!("any[{idx}]: {reason}"));
                if matched {
                    return Ok((true, reasons.join("; ")));
                }
            }
            Ok((false, reasons.join("; ")))
        }
        WorkflowCondition::Not { not } => {
            let (matched, reason) = evaluate_node(bytes, doc, not)?;
            Ok((!matched, format!("not({reason})")))
        }
        WorkflowCondition::Leaf(leaf) => evaluate_leaf(bytes, doc, leaf),
    }
}

fn evaluate_leaf(
    bytes: &[u8],
    doc: &Document,
    leaf: &LeafCondition,
) -> OperationResult<(bool, String)> {
    let count = doc.get_pages().len();
    let mut matches = true;
    let mut reasons = vec![format!("page_count={count}")];
    if let Some(threshold) = leaf.if_page_count_gt {
        matches &= count > threshold as usize;
        reasons.push(format!("if_page_count_gt={threshold}"));
    }
    if let (Some(comparator), Some(value)) = (leaf.comparator, leaf.value) {
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
    if let Some(expected) = leaf.if_scanned {
        let scanned = is_scanned(doc)?;
        matches &= scanned == expected;
        reasons.push(format!("scanned={scanned}, expected={expected}"));
    }

    if let Some(target) = &leaf.text_contains {
        let target_lower = target.to_lowercase();
        let mut found = false;
        for (number, _) in doc.get_pages() {
            if let Ok(text) = doc.extract_text_with_limit(&[number], 16 * 1024 * 1024) {
                if text.to_lowercase().contains(&target_lower) {
                    found = true;
                    break;
                }
            }
        }
        matches &= found;
        reasons.push(format!("text_contains('{target}')={found}"));
    }

    if let Some(expected) = leaf.has_form_fields {
        let has_forms = has_acroform_fields(doc)?;
        matches &= has_forms == expected;
        reasons.push(format!("has_form_fields={has_forms}, expected={expected}"));
    }

    if let Some(expected) = leaf.is_signed {
        let signed = is_digitally_signed(doc)?;
        matches &= signed == expected;
        reasons.push(format!("is_signed={signed}, expected={expected}"));
    }

    if let Some(expected) = leaf.is_encrypted {
        let encrypted = doc.is_encrypted();
        matches &= encrypted == expected;
        reasons.push(format!("is_encrypted={encrypted}, expected={expected}"));
    }

    if let Some(comparator) = &leaf.file_size_bytes {
        let size = bytes.len() as u64;
        let satisfied = match comparator {
            NumericComparator::Between {
                between: [min, max],
            } => size >= *min && size <= *max,
            NumericComparator::Comparison {
                eq,
                gt,
                gte,
                lt,
                lte,
            } => {
                let mut ok = true;
                if let Some(v) = eq {
                    ok &= size == *v;
                }
                if let Some(v) = gt {
                    ok &= size > *v;
                }
                if let Some(v) = gte {
                    ok &= size >= *v;
                }
                if let Some(v) = lt {
                    ok &= size < *v;
                }
                if let Some(v) = lte {
                    ok &= size <= *v;
                }
                ok
            }
        };
        matches &= satisfied;
        reasons.push(format!("file_size_bytes={size}, satisfied={satisfied}"));
    }

    Ok((matches, reasons.join("; ")))
}

fn has_acroform_fields(doc: &Document) -> OperationResult<bool> {
    crate::operations::document_inspection::has_form_fields(doc)
}
fn is_digitally_signed(doc: &Document) -> OperationResult<bool> {
    crate::operations::document_inspection::has_signature(doc)
}

// Conservative, local heuristic: at least one page paints a raster image and
// has no extractable non-whitespace text. Blank/vector-only pages are not scans.
// A mixed document counts as scanned if any page needs OCR. This is not a model.
pub(super) fn is_scanned(doc: &Document) -> OperationResult<bool> {
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
