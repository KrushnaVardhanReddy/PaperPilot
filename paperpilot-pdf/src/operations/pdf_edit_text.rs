use crate::document::LopdfDocument;
use lopdf::content::Content;
use lopdf::{Dictionary, Document, Object, ObjectId, Stream, StringFormat};
use paperpilot_core::error::{OperationResult, PdfError};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfEditTextOptions {
    #[serde(default)]
    pub find: String,
    #[serde(default)]
    pub replace: String,
    #[serde(default)]
    pub page: Option<u32>, // 1-indexed; if None, replace on all pages
    #[serde(default)]
    pub match_case: Option<bool>,
    #[serde(default)]
    pub occurrence: Option<u32>, // 1-indexed match occurrence (e.g. Some(1) = 1st match only; None = all)
}

#[derive(Default)]
pub struct PdfEditTextOperation {
    options: PdfEditTextOptions,
}

impl PdfEditTextOperation {
    pub fn new(options: PdfEditTextOptions) -> Self {
        Self { options }
    }
}

impl PdfOperation for PdfEditTextOperation {
    fn execute(&self, document: &mut dyn PdfDocument) -> OperationResult<()> {
        let lopdf_doc = document
            .as_any_mut()
            .downcast_mut::<LopdfDocument>()
            .ok_or_else(|| {
                PdfError::UnsupportedOperation("Document is not a LopdfDocument".to_string())
            })?;

        let modified = edit_pdf_text_in_doc(&mut lopdf_doc.inner, &self.options)?;
        if modified {
            // In-place content stream mutation invalidates cached pre-edit backing bytes.
            lopdf_doc.original_bytes = None;
            lopdf_doc.source = None;
        }
        Ok(())
    }
}

pub fn pdf_edit_text_bytes(
    input_bytes: &[u8],
    options: &PdfEditTextOptions,
) -> Result<Vec<u8>, PdfError> {
    if input_bytes.is_empty() {
        return Err(PdfError::InvalidInput(
            "Input PDF bytes cannot be empty".to_string(),
        ));
    }
    if options.find.is_empty() {
        return Err(PdfError::InvalidInput(
            "Search text ('find') cannot be empty".to_string(),
        ));
    }

    let mut doc = LopdfDocument::from_bytes(input_bytes)?;
    let op = PdfEditTextOperation::new(options.clone());
    op.execute(&mut doc)?;

    let mut output = Vec::new();
    doc.inner
        .save_to(&mut output)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;
    Ok(output)
}

/// Finds and replaces text in-place inside target page `/Contents` streams of a [`lopdf::Document`].
/// Returns `Ok(true)` if at least one page content stream was modified.
pub fn edit_pdf_text_in_doc(
    doc: &mut Document,
    options: &PdfEditTextOptions,
) -> OperationResult<bool> {
    if options.find.is_empty() {
        return Err(PdfError::InvalidInput(
            "Search text ('find') cannot be empty".to_string(),
        ));
    }
    if options.occurrence == Some(0) {
        return Err(PdfError::InvalidInput(
            "Occurrence index is 1-indexed and must be >= 1".to_string(),
        ));
    }

    let pages = doc.get_pages();
    let target_pages: Vec<(u32, ObjectId)> = match options.page {
        Some(0) => {
            return Err(PdfError::InvalidInput(
                "Page number is 1-indexed and must be >= 1".to_string(),
            ));
        }
        Some(page_num) => {
            let page_id = pages.get(&page_num).copied().ok_or_else(|| {
                PdfError::InvalidInput(format!(
                    "Page {} is out of range (document has {} pages)",
                    page_num,
                    pages.len()
                ))
            })?;
            vec![(page_num, page_id)]
        }
        None => {
            let mut entries: Vec<(u32, ObjectId)> = pages.into_iter().collect();
            entries.sort_by_key(|(page_num, _)| *page_num);
            entries
        }
    };

    let match_case = options.match_case.unwrap_or(true);
    let mut match_count: usize = 0;
    let mut any_modified = false;

    for (page_num, page_obj_id) in target_pages {
        if options
            .occurrence
            .is_some_and(|target| match_count >= target as usize)
        {
            break;
        }

        let has_contents = doc
            .get_dictionary(page_obj_id)
            .map(|dict| dict.has(b"Contents"))
            .unwrap_or(false);
        if !has_contents {
            continue;
        }

        let content_data = doc.get_page_content(page_obj_id);
        if content_data.is_empty() {
            continue;
        }

        let mut content = Content::decode(&content_data).map_err(|e| {
            PdfError::ParseError(format!(
                "Failed to decode content stream on page {}: {}",
                page_num, e
            ))
        })?;

        let mut page_modified = false;
        for op in &mut content.operations {
            if options
                .occurrence
                .is_some_and(|target| match_count >= target as usize)
            {
                break;
            }

            match op.operator.as_str() {
                "Tj" | "'" => {
                    if let Some(Object::String(bytes, _)) = op.operands.first_mut()
                        && replace_in_pdf_string(
                            bytes,
                            &options.find,
                            &options.replace,
                            match_case,
                            options.occurrence,
                            &mut match_count,
                        )
                    {
                        page_modified = true;
                    }
                }
                "\"" => {
                    if let Some(Object::String(bytes, _)) = op.operands.get_mut(2)
                        && replace_in_pdf_string(
                            bytes,
                            &options.find,
                            &options.replace,
                            match_case,
                            options.occurrence,
                            &mut match_count,
                        )
                    {
                        page_modified = true;
                    }
                }
                "TJ" => {
                    if let Some(Object::Array(items)) = op.operands.first_mut()
                        && replace_in_tj_array(
                            items,
                            &options.find,
                            &options.replace,
                            match_case,
                            options.occurrence,
                            &mut match_count,
                        )
                    {
                        page_modified = true;
                    }
                }
                _ => {}
            }
        }

        if page_modified {
            let encoded = content.encode().map_err(|e| {
                PdfError::Other(format!(
                    "Failed to encode content stream on page {}: {}",
                    page_num, e
                ))
            })?;

            let mut stream = Stream::new(Dictionary::new(), encoded);
            let _ = stream.compress();
            let stream_id = doc.add_object(Object::Stream(stream));
            if let Ok(page_dict) = doc.get_dictionary_mut(page_obj_id) {
                page_dict.set("Contents", Object::Reference(stream_id));
            }
            any_modified = true;
        }
    }

    Ok(any_modified)
}

fn select_match_ranges(
    ranges: Vec<(usize, usize)>,
    occurrence: Option<u32>,
    match_count: &mut usize,
) -> Vec<(usize, usize)> {
    match occurrence {
        None => {
            *match_count += ranges.len();
            ranges
        }
        Some(target) => {
            let target_idx = target as usize;
            let mut selected = Vec::new();
            for range in ranges {
                *match_count += 1;
                if *match_count == target_idx {
                    selected.push(range);
                }
            }
            selected
        }
    }
}

fn replace_in_pdf_string(
    bytes: &mut Vec<u8>,
    find: &str,
    replace: &str,
    match_case: bool,
    occurrence: Option<u32>,
    match_count: &mut usize,
) -> bool {
    if bytes.is_empty() || find.is_empty() {
        return false;
    }

    let ranges = select_match_ranges(
        find_match_ranges(bytes, find, match_case),
        occurrence,
        match_count,
    );
    if ranges.is_empty() {
        return false;
    }

    let replacement = replace.as_bytes();
    let mut updated = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    for (start, end) in ranges {
        updated.extend_from_slice(&bytes[cursor..start]);
        updated.extend_from_slice(replacement);
        cursor = end;
    }
    updated.extend_from_slice(&bytes[cursor..]);
    *bytes = updated;
    true
}

/// Replaces matching occurrences across `TJ` arrays, supporting both self-contained string
/// elements and words split by small intra-word kerning adjustments (e.g. `[(W) -15 (rold)]`).
fn replace_in_tj_array(
    items: &mut Vec<Object>,
    find: &str,
    replace: &str,
    match_case: bool,
    occurrence: Option<u32>,
    match_count: &mut usize,
) -> bool {
    if items.is_empty() || find.is_empty() {
        return false;
    }

    // Large negative kerning (<= -120 thousandths of text space) typically marks a word space,
    // so only coalesce across small intra-word kerning adjustments.
    const INTRA_WORD_KERN_THRESHOLD: f64 = 120.0;

    let mut new_items: Vec<Object> = Vec::with_capacity(items.len());
    let mut modified = false;
    let mut i = 0;

    while i < items.len() {
        if occurrence.is_some_and(|target| *match_count >= target as usize) {
            if modified {
                new_items.extend_from_slice(&items[i..]);
            }
            break;
        }

        if !matches!(items[i], Object::String(_, _)) {
            new_items.push(items[i].clone());
            i += 1;
            continue;
        }

        // Collect a maximal contiguous run of String elements separated only by small kerning numbers.
        let run_start = i;
        let mut run_end = i + 1;
        while run_end < items.len() {
            match &items[run_end] {
                Object::String(_, _) => run_end += 1,
                Object::Integer(n)
                    if (*n as f64).abs() < INTRA_WORD_KERN_THRESHOLD
                        && run_end + 1 < items.len()
                        && matches!(items[run_end + 1], Object::String(_, _)) =>
                {
                    run_end += 1;
                }
                Object::Real(r)
                    if (*r as f64).abs() < INTRA_WORD_KERN_THRESHOLD
                        && run_end + 1 < items.len()
                        && matches!(items[run_end + 1], Object::String(_, _)) =>
                {
                    run_end += 1;
                }
                _ => break,
            }
        }

        let replaced_run = replace_in_tj_run(
            &items[run_start..run_end],
            find,
            replace,
            match_case,
            occurrence,
            match_count,
        );
        match replaced_run {
            Some(updated) => {
                new_items.extend(updated);
                modified = true;
            }
            None => {
                new_items.extend_from_slice(&items[run_start..run_end]);
            }
        }
        i = run_end;
    }

    if modified {
        *items = new_items;
    }
    modified
}

struct TjSegment {
    run_idx: usize,
    start: usize,
    end: usize,
    format: StringFormat,
}

fn replace_in_tj_run(
    run: &[Object],
    find: &str,
    replace: &str,
    match_case: bool,
    occurrence: Option<u32>,
    match_count: &mut usize,
) -> Option<Vec<Object>> {
    let string_count = run
        .iter()
        .filter(|obj| matches!(obj, Object::String(_, _)))
        .count();
    if string_count == 1 {
        let mut out = run.to_vec();
        for obj in &mut out {
            if let Object::String(bytes, _) = obj {
                if replace_in_pdf_string(bytes, find, replace, match_case, occurrence, match_count)
                {
                    return Some(out);
                }
            }
        }
        return None;
    }

    let mut concatenated = Vec::new();
    let mut segments = Vec::with_capacity(string_count);
    for (idx, obj) in run.iter().enumerate() {
        if let Object::String(bytes, fmt) = obj {
            let start = concatenated.len();
            concatenated.extend_from_slice(bytes);
            let end = concatenated.len();
            segments.push(TjSegment {
                run_idx: idx,
                start,
                end,
                format: *fmt,
            });
        }
    }

    let ranges = select_match_ranges(
        find_match_ranges(&concatenated, find, match_case),
        occurrence,
        match_count,
    );
    if ranges.is_empty() {
        return None;
    }

    let mut drop_kern_after = vec![false; segments.len()];
    for &(m_start, m_end) in &ranges {
        for (seg_idx, seg) in segments.iter().enumerate().take(segments.len().saturating_sub(1)) {
            if m_start < seg.end && m_end > seg.end {
                drop_kern_after[seg_idx] = true;
            }
        }
    }

    let replace_bytes = replace.as_bytes();
    let mut rebuilt_segments: Vec<Option<(Vec<u8>, StringFormat)>> =
        Vec::with_capacity(segments.len());
    let mut match_idx = 0;

    for seg in &segments {
        let mut cursor = seg.start;
        let mut seg_out = Vec::new();

        while match_idx < ranges.len() && ranges[match_idx].1 <= seg.start {
            match_idx += 1;
        }

        while match_idx < ranges.len() {
            let (m_start, m_end) = ranges[match_idx];
            if m_start >= seg.end {
                break;
            }

            if m_start >= seg.start {
                if m_start > cursor {
                    seg_out.extend_from_slice(&concatenated[cursor..m_start]);
                }
                seg_out.extend_from_slice(replace_bytes);
            }

            let overlap_end = m_end.min(seg.end);
            if overlap_end > cursor {
                cursor = overlap_end;
            }

            if m_end <= seg.end {
                match_idx += 1;
            } else {
                break;
            }
        }

        if cursor < seg.end {
            seg_out.extend_from_slice(&concatenated[cursor..seg.end]);
        }

        if seg_out.is_empty() {
            rebuilt_segments.push(None);
        } else {
            rebuilt_segments.push(Some((seg_out, seg.format)));
        }
    }

    let mut out = Vec::with_capacity(run.len());
    let mut seg_cursor = 0;
    for (run_idx, item) in run.iter().enumerate() {
        if seg_cursor < segments.len() && segments[seg_cursor].run_idx == run_idx {
            if let Some((bytes, fmt)) = rebuilt_segments[seg_cursor].take() {
                out.push(Object::String(bytes, fmt));
            }
            seg_cursor += 1;
        } else {
            // Preserve kerning numbers between segments unless the boundary itself was replaced.
            let prev_seg_idx = seg_cursor.saturating_sub(1);
            if seg_cursor > 0 && drop_kern_after[prev_seg_idx] {
                continue;
            }
            out.push(item.clone());
        }
    }

    if out.is_empty() {
        let fmt = segments.first().map(|s| s.format).unwrap_or(StringFormat::Literal);
        out.push(Object::String(Vec::new(), fmt));
    }

    Some(out)
}

fn find_match_ranges(bytes: &[u8], needle: &str, match_case: bool) -> Vec<(usize, usize)> {
    if bytes.is_empty() || needle.is_empty() {
        return Vec::new();
    }

    if let Ok(haystack) = std::str::from_utf8(bytes) {
        return find_utf8_match_ranges(haystack, needle, match_case);
    }

    if !needle.is_ascii() {
        return Vec::new();
    }

    let needle_bytes = needle.as_bytes();
    if bytes.len() < needle_bytes.len() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut idx = 0;
    while idx + needle_bytes.len() <= bytes.len() {
        let slice = &bytes[idx..idx + needle_bytes.len()];
        let matched = if match_case {
            slice == needle_bytes
        } else {
            slice.eq_ignore_ascii_case(needle_bytes)
        };
        if matched {
            ranges.push((idx, idx + needle_bytes.len()));
            idx += needle_bytes.len();
        } else {
            idx += 1;
        }
    }
    ranges
}

fn find_utf8_match_ranges(haystack: &str, needle: &str, match_case: bool) -> Vec<(usize, usize)> {
    if match_case {
        return haystack
            .match_indices(needle)
            .map(|(start, matched)| (start, start + matched.len()))
            .collect();
    }

    if needle.is_ascii() {
        let h_bytes = haystack.as_bytes();
        let n_bytes = needle.as_bytes();
        if h_bytes.len() < n_bytes.len() {
            return Vec::new();
        }
        let mut ranges = Vec::new();
        let mut idx = 0;
        while idx + n_bytes.len() <= h_bytes.len() {
            if h_bytes[idx..idx + n_bytes.len()].eq_ignore_ascii_case(n_bytes) {
                ranges.push((idx, idx + n_bytes.len()));
                idx += n_bytes.len();
            } else {
                idx += 1;
            }
        }
        return ranges;
    }

    let h_chars: Vec<(usize, char)> = haystack.char_indices().collect();
    let n_lower: Vec<Vec<char>> = needle
        .chars()
        .map(|c| c.to_lowercase().collect())
        .collect();
    if h_chars.len() < n_lower.len() || n_lower.is_empty() {
        return Vec::new();
    }

    let h_lower: Vec<Vec<char>> = h_chars
        .iter()
        .map(|(_, c)| c.to_lowercase().collect())
        .collect();

    let mut ranges = Vec::new();
    let mut idx = 0;
    while idx + n_lower.len() <= h_chars.len() {
        if h_lower[idx..idx + n_lower.len()] == n_lower[..] {
            let start_byte = h_chars[idx].0;
            let end_byte = if idx + n_lower.len() < h_chars.len() {
                h_chars[idx + n_lower.len()].0
            } else {
                haystack.len()
            };
            ranges.push((start_byte, end_byte));
            idx += n_lower.len();
        } else {
            idx += 1;
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::content::Operation;
    use lopdf::dictionary;

    fn build_multipage_text_pdf() -> Document {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();

        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
            "Encoding" => "WinAnsiEncoding",
        });

        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        });

        let page1_content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new("Td", vec![Object::Real(72.0), Object::Real(720.0)]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal("Welcome to PaperPiot documentation")],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let mut s1 = Stream::new(Dictionary::new(), page1_content.encode().unwrap());
        let _ = s1.compress();
        let c1_id = doc.add_object(Object::Stream(s1));

        let page2_content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new("Td", vec![Object::Real(72.0), Object::Real(720.0)]),
                Operation::new(
                    "TJ",
                    vec![Object::Array(vec![
                        Object::string_literal("Page two has "),
                        Object::string_literal("Paper"),
                        Object::Integer(-15),
                        Object::string_literal("Piot"),
                        Object::string_literal(" and paperpiot typo."),
                    ])],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let c2_id = doc.add_object(Object::Stream(Stream::new(
            Dictionary::new(),
            page2_content.encode().unwrap(),
        )));

        let page3_content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new("Td", vec![Object::Real(72.0), Object::Real(720.0)]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal("Untouched appendix with PaperPiot note.")],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let c3_id = doc.add_object(Object::Stream(Stream::new(
            Dictionary::new(),
            page3_content.encode().unwrap(),
        )));

        let p1_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => c1_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        let p2_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => c2_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        let p3_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => c3_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });

        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![p1_id.into(), p2_id.into(), p3_id.into()],
                "Count" => 3,
            }),
        );

        let info_id = doc.add_object(dictionary! {
            "Title" => Object::string_literal("MultiPage Typo Fixture"),
            "Author" => Object::string_literal("PaperPilot QA"),
        });

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        doc.trailer.set("Root", catalog_id);
        doc.trailer.set("Info", info_id);
        doc
    }

    fn doc_to_bytes(doc: &mut Document) -> Vec<u8> {
        let mut bytes = Vec::new();
        doc.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn test_pdf_edit_text_empty_document() {
        let options = PdfEditTextOptions {
            find: "foo".to_string(),
            replace: "bar".to_string(),
            page: None,
            match_case: None,
            occurrence: None,
        };
        let bytes = vec![];
        let result = pdf_edit_text_bytes(&bytes, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_pdf_edit_text_empty_find_rejected() {
        let mut doc = build_multipage_text_pdf();
        let bytes = doc_to_bytes(&mut doc);
        let options = PdfEditTextOptions::default();
        let err = pdf_edit_text_bytes(&bytes, &options).unwrap_err();
        assert!(matches!(err, PdfError::InvalidInput(_)));
    }

    #[test]
    fn test_pdf_edit_text_all_pages_and_verify_with_lopdf_and_pdf_oxide() {
        let mut doc = build_multipage_text_pdf();
        let input_bytes = doc_to_bytes(&mut doc);

        let options = PdfEditTextOptions {
            find: "PaperPiot".to_string(),
            replace: "PaperPilot".to_string(),
            page: None,
            match_case: Some(true),
            occurrence: None,
        };
        let out_bytes = pdf_edit_text_bytes(&input_bytes, &options).unwrap();

        // 1. Verify via lopdf
        let edited_lopdf = Document::load_mem(&out_bytes).unwrap();
        assert_eq!(edited_lopdf.get_pages().len(), 3);

        let p1_text = edited_lopdf.extract_text(&[1]).unwrap();
        let p2_text = edited_lopdf.extract_text(&[2]).unwrap();
        let p3_text = edited_lopdf.extract_text(&[3]).unwrap();

        assert!(p1_text.contains("PaperPilot"), "Page 1 text: {p1_text}");
        assert!(!p1_text.contains("PaperPiot"), "Page 1 text: {p1_text}");

        // Page 2 had split TJ [("Paper") -15 ("Piot")] AND lowercase "paperpiot"
        assert!(p2_text.contains("PaperPilot"), "Page 2 text: {p2_text}");
        assert!(!p2_text.contains("PaperPiot"), "Page 2 text: {p2_text}");
        assert!(
            p2_text.contains("paperpiot"),
            "Case-sensitive mode must keep lowercase paperpiot: {p2_text}"
        );

        assert!(p3_text.contains("PaperPilot"), "Page 3 text: {p3_text}");

        // Verify metadata and page MediaBox structure preserved
        let info_ref = edited_lopdf.trailer.get(b"Info").unwrap().as_reference().unwrap();
        let info_dict = edited_lopdf.get_dictionary(info_ref).unwrap();
        assert_eq!(
            String::from_utf8_lossy(info_dict.get(b"Title").unwrap().as_str().unwrap()),
            "MultiPage Typo Fixture"
        );
        assert_eq!(
            String::from_utf8_lossy(info_dict.get(b"Author").unwrap().as_str().unwrap()),
            "PaperPilot QA"
        );

        // 2. Verify via pdf_oxide
        let mut oxide_pdf = pdf_oxide::api::Pdf::from_bytes(out_bytes).unwrap();
        assert_eq!(oxide_pdf.page_count().unwrap(), 3);
        let oxide_p1 = oxide_pdf.to_markdown(0).unwrap();
        let oxide_p2 = oxide_pdf.to_markdown(1).unwrap();
        let oxide_p3 = oxide_pdf.to_markdown(2).unwrap();
        assert!(oxide_p1.contains("PaperPilot"), "pdf_oxide page 1: {oxide_p1}");
        assert!(oxide_p2.contains("PaperPilot"), "pdf_oxide page 2: {oxide_p2}");
        assert!(oxide_p3.contains("PaperPilot"), "pdf_oxide page 3: {oxide_p3}");
        assert!(!oxide_p1.contains("PaperPiot"));
    }

    #[test]
    fn test_pdf_edit_text_single_page_preserves_other_pages() {
        let doc = build_multipage_text_pdf();
        let orig_p1_id = *doc.get_pages().get(&1).unwrap();
        let orig_p1_contents = doc.get_dictionary(orig_p1_id).unwrap().get(b"Contents").unwrap().clone();
        let orig_p3_id = *doc.get_pages().get(&3).unwrap();
        let orig_p3_contents = doc.get_dictionary(orig_p3_id).unwrap().get(b"Contents").unwrap().clone();

        let mut lopdf_doc = LopdfDocument::from_inner_with_bytes(doc, vec![1, 2, 3]);
        let op = PdfEditTextOperation::new(PdfEditTextOptions {
            find: "PaperPiot".to_string(),
            replace: "PaperPilot".to_string(),
            page: Some(2),
            match_case: Some(false),
            occurrence: None,
        });
        op.execute(&mut lopdf_doc).unwrap();
        assert!(lopdf_doc.source_bytes().is_none());

        let edited = &mut lopdf_doc.inner;
        let new_p1_contents = edited.get_dictionary(orig_p1_id).unwrap().get(b"Contents").unwrap().clone();
        let new_p3_contents = edited.get_dictionary(orig_p3_id).unwrap().get(b"Contents").unwrap().clone();
        assert_eq!(orig_p1_contents, new_p1_contents, "Untouched page 1 Contents ref changed");
        assert_eq!(orig_p3_contents, new_p3_contents, "Untouched page 3 Contents ref changed");

        let p1_text = edited.extract_text(&[1]).unwrap();
        let p2_text = edited.extract_text(&[2]).unwrap();
        let p3_text = edited.extract_text(&[3]).unwrap();
        assert!(p1_text.contains("PaperPiot"), "Page 1 should remain unchanged: {p1_text}");
        assert!(p2_text.contains("PaperPilot"), "Page 2 should be replaced: {p2_text}");
        assert!(!p2_text.to_lowercase().contains("paperpiot"), "Case-insensitive should replace both occurrences on page 2: {p2_text}");
        assert!(p3_text.contains("PaperPiot"), "Page 3 should remain unchanged: {p3_text}");

        let saved = doc_to_bytes(edited);
        let mut oxide = pdf_oxide::api::Pdf::from_bytes(saved).unwrap();
        assert_eq!(oxide.page_count().unwrap(), 3);
        assert!(oxide.to_markdown(0).unwrap().contains("PaperPiot"));
        assert!(oxide.to_markdown(1).unwrap().contains("PaperPilot"));
        assert!(oxide.to_markdown(2).unwrap().contains("PaperPiot"));
    }

    #[test]
    fn test_pdf_edit_text_invalid_page_number() {
        let mut doc = build_multipage_text_pdf();
        let bytes = doc_to_bytes(&mut doc);

        let err_zero = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "PaperPiot".to_string(),
                replace: "PaperPilot".to_string(),
                page: Some(0),
                match_case: None,
                occurrence: None,
            },
        )
        .unwrap_err();
        assert!(matches!(err_zero, PdfError::InvalidInput(_)));

        let err_oob = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "PaperPiot".to_string(),
                replace: "PaperPilot".to_string(),
                page: Some(99),
                match_case: None,
                occurrence: None,
            },
        )
        .unwrap_err();
        assert!(matches!(err_oob, PdfError::InvalidInput(_)));
    }

    #[test]
    fn test_pdf_edit_text_preserves_word_spacing_hex_format_and_winansi_bytes() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
            "Encoding" => "WinAnsiEncoding",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                // Hex literal string
                Operation::new(
                    "Tj",
                    vec![Object::String(b"Helo hex".to_vec(), StringFormat::Hexadecimal)],
                ),
                // WinAnsi 0xE9 ('e' with acute) + typo
                Operation::new(
                    "Tj",
                    vec![Object::string_literal(b"Caf\xE9 Wrold".to_vec())],
                ),
                // Large -250 word-space kerning must NOT coalesce separate words
                Operation::new(
                    "TJ",
                    vec![Object::Array(vec![
                        Object::string_literal("end"),
                        Object::Integer(-250),
                        Object::string_literal("start"),
                    ])],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            }),
        );
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let bytes = doc_to_bytes(&mut doc);

        // Replace hex string + preserve StringFormat::Hexadecimal
        let out1 = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "Helo".to_string(),
                replace: "Hello".to_string(),
                page: Some(1),
                match_case: Some(true),
                occurrence: None,
            },
        )
        .unwrap();
        let d1 = Document::load_mem(&out1).unwrap();
        let c1 = Content::decode(&d1.get_page_content(*d1.get_pages().get(&1).unwrap())).unwrap();
        assert_eq!(
            c1.operations[2].operands[0],
            Object::String(b"Hello hex".to_vec(), StringFormat::Hexadecimal)
        );

        // Replace WinAnsi non-UTF8 string cleanly preserving 0xE9 byte
        let out2 = pdf_edit_text_bytes(
            &out1,
            &PdfEditTextOptions {
                find: "Wrold".to_string(),
                replace: "World".to_string(),
                page: None,
                match_case: Some(true),
                occurrence: None,
            },
        )
        .unwrap();
        let d2 = Document::load_mem(&out2).unwrap();
        let c2 = Content::decode(&d2.get_page_content(*d2.get_pages().get(&1).unwrap())).unwrap();
        assert_eq!(
            c2.operations[3].operands[0],
            Object::String(b"Caf\xE9 World".to_vec(), StringFormat::Literal)
        );

        // Verify -250 word space is NOT coalesced when searching "endstart"
        let out3 = pdf_edit_text_bytes(
            &out2,
            &PdfEditTextOptions {
                find: "endstart".to_string(),
                replace: "merged".to_string(),
                page: None,
                match_case: Some(true),
                occurrence: None,
            },
        )
        .unwrap();
        let d3 = Document::load_mem(&out3).unwrap();
        let c3 = Content::decode(&d3.get_page_content(*d3.get_pages().get(&1).unwrap())).unwrap();
        assert_eq!(
            c3.operations[4].operands[0],
            Object::Array(vec![
                Object::string_literal("end"),
                Object::Integer(-250),
                Object::string_literal("start"),
            ])
        );
    }

    #[test]
    fn test_pdf_edit_text_single_occurrence_replaces_only_target_match() {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        });
        let resources_id = doc.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id },
        });
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec![Object::Name(b"F1".to_vec()), Object::Real(12.0)]),
                Operation::new("Td", vec![Object::Real(72.0), Object::Real(720.0)]),
                Operation::new(
                    "Tj",
                    vec![Object::string_literal("First foo and second foo here.")],
                ),
                Operation::new(
                    "TJ",
                    vec![Object::Array(vec![
                        Object::string_literal("Third "),
                        Object::string_literal("fo"),
                        Object::Integer(-15),
                        Object::string_literal("o end"),
                    ])],
                ),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content.encode().unwrap()));
        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Resources" => resources_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        });
        doc.objects.insert(
            pages_id,
            Object::Dictionary(dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            }),
        );
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        let bytes = doc_to_bytes(&mut doc);

        // occurrence: Some(1) replaces ONLY the 1st "foo", leaving 2nd and 3rd untouched
        let out_first = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "foo".to_string(),
                replace: "bar".to_string(),
                page: Some(1),
                match_case: Some(true),
                occurrence: Some(1),
            },
        )
        .unwrap();
        let d_first = Document::load_mem(&out_first).unwrap();
        let c_first =
            Content::decode(&d_first.get_page_content(*d_first.get_pages().get(&1).unwrap()))
                .unwrap();
        assert_eq!(
            c_first.operations[3].operands[0],
            Object::string_literal("First bar and second foo here.")
        );
        assert_eq!(
            c_first.operations[4].operands[0],
            Object::Array(vec![
                Object::string_literal("Third "),
                Object::string_literal("fo"),
                Object::Integer(-15),
                Object::string_literal("o end"),
            ])
        );
        let mut oxide = pdf_oxide::api::Pdf::from_bytes(out_first).unwrap();
        let md = oxide.to_markdown(0).unwrap();
        assert!(md.contains("First bar and second foo here."), "pdf_oxide md: {md}");
        assert!(md.contains("Third foo end"), "pdf_oxide md: {md}");

        // occurrence: Some(2) replaces ONLY the 2nd "foo", leaving 1st and 3rd untouched
        let out_second = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "foo".to_string(),
                replace: "bar".to_string(),
                page: Some(1),
                match_case: Some(true),
                occurrence: Some(2),
            },
        )
        .unwrap();
        let d_second = Document::load_mem(&out_second).unwrap();
        let text_second = d_second.extract_text(&[1]).unwrap();
        assert!(
            text_second.contains("First foo and second bar here."),
            "2nd occurrence mismatch: {text_second}"
        );
        assert!(text_second.contains("Third foo end"), "{text_second}");

        // occurrence: Some(3) replaces ONLY the split TJ 3rd "foo", preserving first two
        let out_third = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "foo".to_string(),
                replace: "bar".to_string(),
                page: Some(1),
                match_case: Some(true),
                occurrence: Some(3),
            },
        )
        .unwrap();
        let d_third = Document::load_mem(&out_third).unwrap();
        let text_third = d_third.extract_text(&[1]).unwrap();
        assert!(
            text_third.contains("First foo and second foo here."),
            "3rd occurrence mismatch: {text_third}"
        );
        assert!(text_third.contains("Third bar end"), "{text_third}");

        // occurrence: Some(0) is rejected as InvalidInput
        let err_zero = pdf_edit_text_bytes(
            &bytes,
            &PdfEditTextOptions {
                find: "foo".to_string(),
                replace: "bar".to_string(),
                page: Some(1),
                match_case: Some(true),
                occurrence: Some(0),
            },
        )
        .unwrap_err();
        assert!(matches!(err_zero, PdfError::InvalidInput(_)));
    }
}
