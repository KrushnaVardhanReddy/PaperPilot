use paperpilot_core::error::{PdfError, OperationResult};
use lopdf::{Document, Object};
use std::collections::BTreeMap;

pub fn merge(buffers: Vec<Vec<u8>>) -> OperationResult<Vec<u8>> {
    if buffers.is_empty() {
        return Err(PdfError::InvalidInput("No buffers provided for merge".into()));
    }

    let mut documents = Vec::new();
    for (i, buffer) in buffers.iter().enumerate() {
        let doc = Document::load_mem(buffer)
            .map_err(|e| PdfError::ParseError(format!("Failed to parse document {i}: {e}")))?;
        documents.push(doc);
    }

    let mut merged_doc = documents.remove(0);

    for mut other_doc in documents {
        other_doc.renumber_objects();

        let max_id = merged_doc.max_id;
        let mut new_objects = BTreeMap::new();
        for (old_id, mut object) in other_doc.objects.clone() {
            let new_id = (old_id.0 + max_id, old_id.1);
            adjust_references(&mut object, max_id);
            new_objects.insert(new_id, object);
        }

        merged_doc.objects.extend(new_objects);
        merged_doc.max_id = merged_doc.objects.keys().map(|k| k.0).max().unwrap_or(0);

        let pages = other_doc.get_pages();

        let catalog_id = merged_doc.trailer.get(b"Root").and_then(|root| root.as_reference()).ok();
        let pages_id = match catalog_id {
            Some(cat_id) => {
                let catalog = merged_doc.get_object(cat_id).unwrap();
                if let Object::Dictionary(dict) = catalog {
                    if let Ok(pages) = dict.get(b"Pages") {
                        pages.as_reference().unwrap_or((merged_doc.max_id + 1, 0))
                    } else {
                        (merged_doc.max_id + 1, 0)
                    }
                } else {
                    (merged_doc.max_id + 1, 0)
                }
            }
            None => (merged_doc.max_id + 1, 0),
        };

        if !merged_doc.objects.contains_key(&pages_id) {
            merged_doc.max_id += 1;
            merged_doc.objects.insert(
                pages_id,
                Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                    (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
                    (b"Count".to_vec(), Object::Integer(0)),
                    (b"Kids".to_vec(), Object::Array(vec![])),
                ])),
            );

            if let Some(cat_id) = catalog_id {
                if let Ok(Object::Dictionary(dict)) = merged_doc.get_object_mut(cat_id) {
                    dict.set("Pages", Object::Reference(pages_id));
                }
            } else {
                let new_catalog_id = (merged_doc.max_id + 1, 0);
                merged_doc.max_id += 1;
                merged_doc.objects.insert(
                    new_catalog_id,
                    Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                        (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
                        (b"Pages".to_vec(), Object::Reference(pages_id)),
                    ])),
                );
                merged_doc.trailer.set("Root", Object::Reference(new_catalog_id));
            }
        }

        for (_, page_id) in pages {
            let adjusted_page_id = (page_id.0 + max_id, page_id.1);

            let pages_obj = merged_doc.get_object_mut(pages_id).unwrap();
            if let Object::Dictionary(dict) = pages_obj {
                if let Ok(Object::Array(kids)) = dict.get_mut(b"Kids") {
                    kids.push(Object::Reference(adjusted_page_id));
                }
                if let Ok(Object::Integer(count)) = dict.get_mut(b"Count") {
                    *count += 1;
                }
            }

            if let Ok(Object::Dictionary(dict)) = merged_doc.get_object_mut(adjusted_page_id) {
                dict.set("Parent", Object::Reference(pages_id));
            }
        }
    }

    let mut buffer = Vec::new();
    merged_doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

fn adjust_references(object: &mut Object, offset: u32) {
    match object {
        Object::Reference(id) => {
            id.0 += offset;
        }
        Object::Array(arr) => {
            for obj in arr.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        Object::Dictionary(dict) => {
            for (_, obj) in dict.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        Object::Stream(stream) => {
            for (_, obj) in stream.dict.iter_mut() {
                adjust_references(obj, offset);
            }
        }
        _ => {}
    }
}

pub fn rotate(input_bytes: &[u8], angle: u16, pages_str: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let pages_to_rotate = parse_pages(pages_str, pages.len() as u32)?;

    for page_num in pages_to_rotate {
        if let Some(&page_id) = pages.get(&page_num) {
            if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
                dict.set("Rotate", Object::Integer(angle as i64));
            }
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

fn parse_pages(pages_str: &str, total_pages: u32) -> OperationResult<Vec<u32>> {
    if pages_str.eq_ignore_ascii_case("all") {
        return Ok((1..=total_pages).collect());
    }

    let mut pages = Vec::new();
    for part in pages_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if let Some((start, end)) = part.split_once('-') {
            let start = start.parse::<u32>()
                .map_err(|_| PdfError::InvalidInput(format!("Invalid page range start: {}", start)))?;
            let end = end.parse::<u32>()
                .map_err(|_| PdfError::InvalidInput(format!("Invalid page range end: {}", end)))?;

            if start < 1 || end > total_pages || start > end {
                 return Err(PdfError::InvalidInput(format!("Invalid page range: {}-{}", start, end)));
            }
            pages.extend(start..=end);
        } else {
            let page = part.parse::<u32>()
                .map_err(|_| PdfError::InvalidInput(format!("Invalid page number: {}", part)))?;
            if page < 1 || page > total_pages {
                 return Err(PdfError::InvalidInput(format!("Invalid page number: {}", page)));
            }
            pages.push(page);
        }
    }
    Ok(pages)
}

pub fn split(input_bytes: &[u8], ranges: &str) -> OperationResult<Vec<Vec<u8>>> {
    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    // Parse ranges into lists of pages for each output document
    let mut split_groups = Vec::new();
    for part in ranges.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let pages_in_group = parse_pages(part, total_pages)?;
        split_groups.push(pages_in_group);
    }

    if split_groups.is_empty() {
        return Err(PdfError::InvalidInput("No valid split ranges provided".into()));
    }

    let mut output_buffers = Vec::new();
    for group in split_groups {
        let mut part_doc = doc.clone();

        let mut pages_to_delete = Vec::new();
        for p in 1..=total_pages {
            if !group.contains(&p) {
                pages_to_delete.push(p);
            }
        }

        if !pages_to_delete.is_empty() {
            part_doc.delete_pages(&pages_to_delete);
        }

        let mut buffer = Vec::new();
        part_doc.save_to(&mut buffer)
            .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

        output_buffers.push(buffer);
    }

    Ok(output_buffers)
}

pub fn compress(input_bytes: &[u8]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    doc.compress();

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn encrypt(input_bytes: &[u8], password: &str) -> OperationResult<Vec<u8>> {
    Err(PdfError::UnsupportedOperation("encrypt not implemented for wasm".into()))
}

pub fn watermark(input_bytes: &[u8], text: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let font_id = doc.add_object(Object::Dictionary(lopdf::Dictionary::from_iter(vec![
        (b"Type".to_vec(), Object::Name(b"Font".to_vec())),
        (b"Subtype".to_vec(), Object::Name(b"Type1".to_vec())),
        (b"BaseFont".to_vec(), Object::Name(b"Helvetica".to_vec())),
    ])));

    let mut pages_to_update = Vec::new();
    for (_page_number, object_id) in doc.get_pages() {
        pages_to_update.push(object_id);
    }

    for object_id in pages_to_update {
        if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object_mut(object_id) {
            let mut new_resources = None;
            if let Ok(res) = page_dict.get_mut(b"Resources") {
                if let lopdf::Object::Dictionary(res_dict) = res {
                    let mut fonts = res_dict
                        .get(b"Font")
                        .and_then(|f| f.as_dict())
                        .cloned()
                        .unwrap_or_else(|_| lopdf::Dictionary::new());
                    fonts.set("F1", lopdf::Object::Reference(font_id));
                    res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                }
            } else {
                let mut fonts = lopdf::Dictionary::new();
                fonts.set("F1", lopdf::Object::Reference(font_id));
                let mut res_dict = lopdf::Dictionary::new();
                res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                new_resources = Some(lopdf::Object::Dictionary(res_dict));
            }
            if let Some(res) = new_resources {
                page_dict.set("Resources", res);
            }
        }

        let escaped_text = text.replace("(", "\\(").replace(")", "\\)");
        let content = format!(
            "q\nBT\n/F1 48 Tf\n1 0 0 1 100 100 Tm\n0.5 g\n({text}) Tj\nET\nQ\n",
            text = escaped_text
        );

        let content_stream = lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
        let stream_id = doc.add_object(content_stream);

        if let Ok(lopdf::Object::Dictionary(dict)) = doc.get_object_mut(object_id) {
            if let Ok(contents) = dict.get_mut(b"Contents") {
                match contents {
                    lopdf::Object::Reference(ref_id) => {
                        let old_ref = *ref_id;
                        *contents = lopdf::Object::Array(vec![
                            lopdf::Object::Reference(old_ref),
                            lopdf::Object::Reference(stream_id),
                        ]);
                    }
                    lopdf::Object::Array(arr) => {
                        arr.push(lopdf::Object::Reference(stream_id));
                    }
                    _ => {}
                }
            } else {
                dict.set("Contents", lopdf::Object::Reference(stream_id));
            }
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}
pub fn delete_pages(input_bytes: &[u8], pages_str: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;
    let pages_to_delete = parse_pages(pages_str, total_pages)?;

    doc.delete_pages(&pages_to_delete);

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn extract_pages(input_bytes: &[u8], pages_str: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;
    let target_pages = parse_pages(pages_str, total_pages)?;

    let mut pages_to_delete = Vec::new();
    for p in 1..=total_pages {
        if !target_pages.contains(&p) {
            pages_to_delete.push(p);
        }
    }

    doc.delete_pages(&pages_to_delete);

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn reorder_pages(input_bytes: &[u8], new_order: &[u32]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    if new_order.len() != total_pages as usize {
        return Err(PdfError::InvalidInput(format!("New order length {} does not match total pages {}", new_order.len(), total_pages)));
    }

    let mut seen = std::collections::HashSet::new();
    for &page_num in new_order {
        if page_num < 1 || page_num > total_pages {
             return Err(PdfError::InvalidInput(format!("Invalid page number in new order: {}", page_num)));
        }
        if !seen.insert(page_num) {
            return Err(PdfError::InvalidInput(format!("Duplicate page number in new order: {}", page_num)));
        }
    }

    // Build the new Kids array
    let mut new_kids = Vec::new();
    for page_num in new_order {
        if let Some(&page_id) = pages.get(page_num) {
            new_kids.push(lopdf::Object::Reference(page_id));
        }
    }

    // Find the Pages object and update Kids
    if let Ok(catalog_id) = doc.trailer.get(b"Root").and_then(|root| root.as_reference()) {
        if let Ok(lopdf::Object::Dictionary(catalog)) = doc.get_object(catalog_id) {
            if let Ok(pages_id) = catalog.get(b"Pages").and_then(|p| p.as_reference()) {
                 if let Ok(lopdf::Object::Dictionary(pages_dict)) = doc.get_object_mut(pages_id) {
                     pages_dict.set("Kids", lopdf::Object::Array(new_kids));
                 }
            }
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn crop(input_bytes: &[u8], left: f32, bottom: f32, right: f32, top: f32) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let crop_box_array = vec![
        lopdf::Object::Real(left),
        lopdf::Object::Real(bottom),
        lopdf::Object::Real(right),
        lopdf::Object::Real(top),
    ];

    for (_, page_id) in doc.get_pages() {
        if let Ok(lopdf::Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            dict.set("CropBox", lopdf::Object::Array(crop_box_array.clone()));
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn flatten(input_bytes: &[u8]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    if let Ok(catalog_id) = doc.trailer.get(b"Root").and_then(|root| root.as_reference()) {
        if let Ok(lopdf::Object::Dictionary(catalog)) = doc.get_object_mut(catalog_id) {
            catalog.remove(b"AcroForm");
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}

pub fn set_metadata(
    input_bytes: &[u8],
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    // Get or create the Info dictionary reference
    let info_id = if let Ok(info_ref) = doc.trailer.get(b"Info").and_then(|info| info.as_reference()) {
        info_ref
    } else {
        let new_info_id = doc.add_object(lopdf::Object::Dictionary(lopdf::Dictionary::new()));
        doc.trailer.set("Info", lopdf::Object::Reference(new_info_id));
        new_info_id
    };

    if let Ok(lopdf::Object::Dictionary(info_dict)) = doc.get_object_mut(info_id) {
        if let Some(t) = title {
            info_dict.set("Title", lopdf::Object::String(t.into_bytes(), lopdf::StringFormat::Literal));
        }
        if let Some(a) = author {
            info_dict.set("Author", lopdf::Object::String(a.into_bytes(), lopdf::StringFormat::Literal));
        }
        if let Some(s) = subject {
            info_dict.set("Subject", lopdf::Object::String(s.into_bytes(), lopdf::StringFormat::Literal));
        }
        if let Some(k) = keywords {
            info_dict.set("Keywords", lopdf::Object::String(k.into_bytes(), lopdf::StringFormat::Literal));
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(buffer)
}
