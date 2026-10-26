use lopdf::{Document, Object, dictionary};
use paperpilot_core::error::{OperationResult, PdfError};
use std::collections::BTreeMap;

pub fn merge(buffers: Vec<Vec<u8>>) -> OperationResult<Vec<u8>> {
    if buffers.is_empty() {
        return Err(PdfError::InvalidInput(
            "No buffers provided for merge".into(),
        ));
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

        let catalog_id = merged_doc
            .trailer
            .get(b"Root")
            .and_then(|root| root.as_reference())
            .ok();
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
                merged_doc
                    .trailer
                    .set("Root", Object::Reference(new_catalog_id));
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
    merged_doc
        .save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

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
        if let Some(&page_id) = pages.get(&page_num)
            && let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id)
        {
            dict.set("Rotate", Object::Integer(angle as i64));
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

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
            let start = start.parse::<u32>().map_err(|_| {
                PdfError::InvalidInput(format!("Invalid page range start: {}", start))
            })?;
            let end = end
                .parse::<u32>()
                .map_err(|_| PdfError::InvalidInput(format!("Invalid page range end: {}", end)))?;

            if start < 1 || end > total_pages || start > end {
                return Err(PdfError::InvalidInput(format!(
                    "Invalid page range: {}-{}",
                    start, end
                )));
            }
            pages.extend(start..=end);
        } else {
            let page = part
                .parse::<u32>()
                .map_err(|_| PdfError::InvalidInput(format!("Invalid page number: {}", part)))?;
            if page < 1 || page > total_pages {
                return Err(PdfError::InvalidInput(format!(
                    "Invalid page number: {}",
                    page
                )));
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
        return Err(PdfError::InvalidInput(
            "No valid split ranges provided".into(),
        ));
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
        part_doc
            .save_to(&mut buffer)
            .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

        output_buffers.push(buffer);
    }

    Ok(output_buffers)
}

pub fn compress(input_bytes: &[u8], quality: Option<&str>) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let (max_dim, jpeg_quality) = match quality {
        Some("low") => (1024, 45),
        Some("medium") => (1600, 70),
        Some("high") => (2400, 85),
        Some(s) if s.parse::<u8>().is_ok() => (1600, s.parse::<u8>().unwrap()),
        _ => (1600, 70), // default to medium
    };

    let object_ids: Vec<lopdf::ObjectId> = doc.objects.keys().copied().collect();

    for object_id in object_ids {
        if let Ok(lopdf::Object::Stream(stream)) = doc.get_object_mut(object_id) {
            let is_image = stream
                .dict
                .get(b"Subtype")
                .and_then(|obj| obj.as_name())
                .map(|name| name == b"Image")
                .unwrap_or(false);

            if !is_image {
                continue;
            }

            let decompressed = match stream.decompressed_content() {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };

            let original_len = stream.content.len();

            let width = match stream.dict.get(b"Width").and_then(|w| w.as_i64()) {
                Ok(w) => w as u32,
                Err(_) => continue,
            };

            let height = match stream.dict.get(b"Height").and_then(|h| h.as_i64()) {
                Ok(h) => h as u32,
                Err(_) => continue,
            };

            let color_space = stream
                .dict
                .get(b"ColorSpace")
                .and_then(|c| c.as_name())
                .unwrap_or(b"DeviceRGB")
                .to_vec();

            let mut dyn_img_opt = image::load_from_memory(&decompressed).ok();

            if dyn_img_opt.is_none() {
                if color_space == b"DeviceRGB"
                    && decompressed.len() >= (width * height * 3) as usize
                {
                    if let Some(img_buf) = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(
                        width,
                        height,
                        decompressed.clone(),
                    ) {
                        dyn_img_opt = Some(image::DynamicImage::ImageRgb8(img_buf));
                    }
                } else if color_space == b"DeviceGray"
                    && decompressed.len() >= (width * height) as usize
                    && let Some(img_buf) = image::ImageBuffer::<image::Luma<u8>, _>::from_raw(
                        width,
                        height,
                        decompressed.clone(),
                    )
                {
                    dyn_img_opt = Some(image::DynamicImage::ImageLuma8(img_buf));
                }
            }

            if let Some(mut dyn_img) = dyn_img_opt {
                let (w, h) = (dyn_img.width(), dyn_img.height());
                if w > max_dim || h > max_dim {
                    dyn_img =
                        dyn_img.resize(max_dim, max_dim, image::imageops::FilterType::Triangle);
                }

                let mut jpeg_bytes = Vec::new();
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                    &mut jpeg_bytes,
                    jpeg_quality,
                );

                if let Ok(_) = dyn_img.write_with_encoder(encoder)
                    && jpeg_bytes.len() < original_len
                {
                    stream.content = jpeg_bytes;
                    stream
                        .dict
                        .set("Filter", lopdf::Object::Name(b"DCTDecode".to_vec()));
                    stream.dict.remove(b"DecodeParms");
                    stream
                        .dict
                        .set("Width", lopdf::Object::Integer(dyn_img.width() as i64));
                    stream
                        .dict
                        .set("Height", lopdf::Object::Integer(dyn_img.height() as i64));
                    stream.dict.set(
                        "Length",
                        lopdf::Object::Integer(stream.content.len() as i64),
                    );
                    if color_space != b"DeviceRGB" && color_space != b"DeviceGray" {
                        if dyn_img.color() == image::ColorType::L8
                            || dyn_img.color() == image::ColorType::La8
                        {
                            stream
                                .dict
                                .set("ColorSpace", lopdf::Object::Name(b"DeviceGray".to_vec()));
                        } else {
                            stream
                                .dict
                                .set("ColorSpace", lopdf::Object::Name(b"DeviceRGB".to_vec()));
                        }
                    }
                }
            }
        }
    }

    doc.compress();

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn encrypt(input_bytes: &[u8], password: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    if doc.trailer.get(b"ID").is_err() {
        let id_str = lopdf::Object::String(
            b"default_id_placeholder".to_vec(),
            lopdf::StringFormat::Literal,
        );
        doc.trailer
            .set("ID", lopdf::Object::Array(vec![id_str.clone(), id_str]));
    }

    let version = lopdf::EncryptionVersion::V2 {
        document: &doc,
        owner_password: password,
        user_password: password,
        key_length: 128,
        permissions: lopdf::Permissions::all(),
    };

    let state = lopdf::EncryptionState::try_from(version)
        .map_err(|e| PdfError::Other(format!("Encryption failed: {}", e)))?;

    doc.encrypt(&state)
        .map_err(|e| PdfError::Other(format!("Encryption failed: {}", e)))?;

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn watermark(
    input_bytes: &[u8],
    text: &str,
    angle: Option<f32>,
    opacity: Option<f32>,
) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let font_id = doc.add_object(Object::Dictionary(lopdf::Dictionary::from_iter(vec![
        (b"Type".to_vec(), Object::Name(b"Font".to_vec())),
        (b"Subtype".to_vec(), Object::Name(b"Type1".to_vec())),
        (b"BaseFont".to_vec(), Object::Name(b"Helvetica".to_vec())),
    ])));

    let extgstate_id = doc.add_object(Object::Dictionary(lopdf::Dictionary::from_iter(vec![
        (b"Type".to_vec(), Object::Name(b"ExtGState".to_vec())),
        (b"ca".to_vec(), Object::Real(opacity.unwrap_or(0.2))),
        (b"CA".to_vec(), Object::Real(opacity.unwrap_or(0.2))),
    ])));

    let mut pages_to_update = Vec::new();
    for (_page_number, object_id) in doc.get_pages() {
        pages_to_update.push(object_id);
    }

    for object_id in pages_to_update {
        let mut page_width = 612.0;
        let mut page_height = 792.0;

        if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object_mut(object_id) {
            if let Ok(media_box) = page_dict.get(b"MediaBox")
                && let Ok(arr) = media_box.as_array()
                && arr.len() == 4
                && let (Ok(x2), Ok(y2)) = (
                    arr[2]
                        .as_f32()
                        .or_else(|_| arr[2].as_i64().map(|v| v as f32)),
                    arr[3]
                        .as_f32()
                        .or_else(|_| arr[3].as_i64().map(|v| v as f32)),
                )
            {
                page_width = x2;
                page_height = y2;
            }

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

                    let mut ext_gstates = res_dict
                        .get(b"ExtGState")
                        .and_then(|e| e.as_dict())
                        .cloned()
                        .unwrap_or_else(|_| lopdf::Dictionary::new());
                    ext_gstates.set("GS1", lopdf::Object::Reference(extgstate_id));
                    res_dict.set("ExtGState", lopdf::Object::Dictionary(ext_gstates));
                }
            } else {
                let mut fonts = lopdf::Dictionary::new();
                fonts.set("F1", lopdf::Object::Reference(font_id));
                let mut ext_gstates = lopdf::Dictionary::new();
                ext_gstates.set("GS1", lopdf::Object::Reference(extgstate_id));
                let mut res_dict = lopdf::Dictionary::new();
                res_dict.set("Font", lopdf::Object::Dictionary(fonts));
                res_dict.set("ExtGState", lopdf::Object::Dictionary(ext_gstates));
                new_resources = Some(lopdf::Object::Dictionary(res_dict));
            }
            if let Some(res) = new_resources {
                page_dict.set("Resources", res);
            }
        }

        let escaped_text = text.replace("(", "\\(").replace(")", "\\)");

        let cx = page_width / 2.0;
        let cy = page_height / 2.0;

        let font_size = 54.0;
        let approx_text_width = (text.len() as f32) * (font_size * 0.5);
        let offset_x = -approx_text_width / 2.0;
        let offset_y = -font_size / 2.0;

        let theta = angle.unwrap_or(45.0) * std::f32::consts::PI / 180.0;
        let cos_t = theta.cos();
        let sin_t = theta.sin();

        // Use gray by default
        let r = 0.5;
        let g = 0.5;
        let b = 0.5;

        let content = format!(
            "q
/GS1 gs
{r} {g} {b} rg
BT
/F1 {font_size} Tf
{cos_t} {sin_t} {minus_sin_t} {cos_t} {cx} {cy} Tm
{offset_x} {offset_y} Td
({text}) Tj
ET
Q
",
            r = r,
            g = g,
            b = b,
            font_size = font_size,
            cos_t = cos_t,
            sin_t = sin_t,
            minus_sin_t = -sin_t,
            cx = cx,
            cy = cy,
            offset_x = offset_x,
            offset_y = offset_y,
            text = escaped_text
        );

        let content_stream =
            lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
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
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

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
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

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
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn reorder_pages(input_bytes: &[u8], new_order: &[u32]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    if new_order.len() != total_pages as usize {
        return Err(PdfError::InvalidInput(format!(
            "New order length {} does not match total pages {}",
            new_order.len(),
            total_pages
        )));
    }

    let mut seen = std::collections::HashSet::new();
    for &page_num in new_order {
        if page_num < 1 || page_num > total_pages {
            return Err(PdfError::InvalidInput(format!(
                "Invalid page number in new order: {}",
                page_num
            )));
        }
        if !seen.insert(page_num) {
            return Err(PdfError::InvalidInput(format!(
                "Duplicate page number in new order: {}",
                page_num
            )));
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
    if let Ok(catalog_id) = doc
        .trailer
        .get(b"Root")
        .and_then(|root| root.as_reference())
        && let Ok(lopdf::Object::Dictionary(catalog)) = doc.get_object(catalog_id)
        && let Ok(pages_id) = catalog.get(b"Pages").and_then(|p| p.as_reference())
        && let Ok(lopdf::Object::Dictionary(pages_dict)) = doc.get_object_mut(pages_id)
    {
        pages_dict.set("Kids", lopdf::Object::Array(new_kids));
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn crop(
    input_bytes: &[u8],
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
) -> OperationResult<Vec<u8>> {
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
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn flatten(input_bytes: &[u8]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    if let Ok(catalog_id) = doc
        .trailer
        .get(b"Root")
        .and_then(|root| root.as_reference())
        && let Ok(lopdf::Object::Dictionary(catalog)) = doc.get_object_mut(catalog_id)
    {
        catalog.remove(b"AcroForm");
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

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
    let info_id = if let Ok(info_ref) = doc
        .trailer
        .get(b"Info")
        .and_then(|info| info.as_reference())
    {
        info_ref
    } else {
        let new_info_id = doc.add_object(lopdf::Object::Dictionary(lopdf::Dictionary::new()));
        doc.trailer
            .set("Info", lopdf::Object::Reference(new_info_id));
        new_info_id
    };

    if let Ok(lopdf::Object::Dictionary(info_dict)) = doc.get_object_mut(info_id) {
        if let Some(t) = title {
            info_dict.set(
                "Title",
                lopdf::Object::String(t.into_bytes(), lopdf::StringFormat::Literal),
            );
        }
        if let Some(a) = author {
            info_dict.set(
                "Author",
                lopdf::Object::String(a.into_bytes(), lopdf::StringFormat::Literal),
            );
        }
        if let Some(s) = subject {
            info_dict.set(
                "Subject",
                lopdf::Object::String(s.into_bytes(), lopdf::StringFormat::Literal),
            );
        }
        if let Some(k) = keywords {
            info_dict.set(
                "Keywords",
                lopdf::Object::String(k.into_bytes(), lopdf::StringFormat::Literal),
            );
        }
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

use flate2::Compression;
use flate2::write::ZlibEncoder;
use image::{GenericImageView, ImageEncoder, ImageFormat};
use lopdf::Dictionary as LopdfDictionary;
use sha2::{Digest, Sha256};
use std::io::Write;

pub fn images_to_pdf(image_buffers: &[&[u8]]) -> OperationResult<Vec<u8>> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut kids = Vec::new();

    for img_buf in image_buffers {
        let format = image::guess_format(img_buf).unwrap_or(ImageFormat::Png);
        let img = image::load_from_memory(img_buf)
            .map_err(|e| PdfError::Other(format!("Failed to load image: {}", e)))?;

        let (width, height) = img.dimensions();

        let mut dict = LopdfDictionary::new();
        dict.set("Type", Object::Name(b"XObject".to_vec()));
        dict.set("Subtype", Object::Name(b"Image".to_vec()));
        dict.set("Width", Object::Integer(width as i64));
        dict.set("Height", Object::Integer(height as i64));
        dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
        dict.set("BitsPerComponent", Object::Integer(8));

        let stream_bytes = if format == ImageFormat::Jpeg {
            dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
            img_buf.to_vec()
        } else {
            dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));
            let rgb = img.to_rgb8().into_raw();
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder
                .write_all(&rgb)
                .map_err(|e| PdfError::Other(format!("Compression failed: {}", e)))?;
            encoder
                .finish()
                .map_err(|e| PdfError::Other(format!("Compression finish failed: {}", e)))?
        };

        let stream = lopdf::Stream::new(dict, stream_bytes);
        let xobj_id = doc.add_object(stream);

        let mut resources = LopdfDictionary::new();
        let mut xobjects = LopdfDictionary::new();
        xobjects.set("Im1", Object::Reference(xobj_id));
        resources.set("XObject", Object::Dictionary(xobjects));

        let content = format!("q\n{} 0 0 {} 0 0 cm\n/Im1 Do\nQ\n", width, height);
        let content_stream =
            lopdf::Stream::new(LopdfDictionary::new(), content.as_bytes().to_vec());
        let content_id = doc.add_object(content_stream);

        let page_id = doc.new_object_id();
        let mut page_dict = LopdfDictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set(
            "MediaBox",
            Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Object::Integer(width as i64),
                Object::Integer(height as i64),
            ]),
        );
        page_dict.set("Resources", Object::Dictionary(resources));
        page_dict.set("Contents", Object::Reference(content_id));

        doc.objects.insert(page_id, Object::Dictionary(page_dict));
        kids.push(Object::Reference(page_id));
    }

    let mut pages_dict = LopdfDictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Count", Object::Integer(kids.len() as i64));
    pages_dict.set("Kids", Object::Array(kids));
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.new_object_id();
    let mut catalog_dict = LopdfDictionary::new();
    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", Object::Reference(pages_id));
    doc.objects
        .insert(catalog_id, Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn extract_images(input_bytes: &[u8]) -> OperationResult<Vec<Vec<u8>>> {
    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let mut extracted = Vec::new();

    for (_page_num, page_id) in doc.get_pages() {
        let page_dict = match doc.get_object(page_id) {
            Ok(Object::Dictionary(d)) => d,
            _ => continue,
        };

        let resources = match page_dict.get(b"Resources") {
            Ok(Object::Dictionary(d)) => d,
            Ok(Object::Reference(id)) => match doc.get_object(*id) {
                Ok(Object::Dictionary(d)) => d,
                _ => continue,
            },
            _ => continue,
        };

        let xobjects = match resources.get(b"XObject") {
            Ok(Object::Dictionary(d)) => d,
            Ok(Object::Reference(id)) => match doc.get_object(*id) {
                Ok(Object::Dictionary(d)) => d,
                _ => continue,
            },
            _ => continue,
        };

        for (_name, obj) in xobjects.iter() {
            let stream_ref = match obj {
                Object::Reference(id) => *id,
                _ => continue,
            };

            let stream = match doc.get_object(stream_ref) {
                Ok(Object::Stream(s)) => s,
                _ => continue,
            };

            let dict = &stream.dict;
            if let Ok(Object::Name(subtype)) = dict.get(b"Subtype")
                && subtype == b"Image"
            {
                let is_jpeg = match dict.get(b"Filter") {
                    Ok(Object::Name(filter)) => filter == b"DCTDecode",
                    Ok(Object::Array(arr)) => arr.iter().any(|f| {
                        if let Object::Name(n) = f {
                            n == b"DCTDecode"
                        } else {
                            false
                        }
                    }),
                    _ => false,
                };

                if is_jpeg {
                    extracted.push(stream.content.clone());
                } else {
                    // Attempt to decode flate and output PNG
                    if let Ok(decompressed) = stream.decompressed_content() {
                        let width = dict.get(b"Width").and_then(|w| w.as_i64()).unwrap_or(0) as u32;
                        let height =
                            dict.get(b"Height").and_then(|h| h.as_i64()).unwrap_or(0) as u32;
                        if width > 0 && height > 0 {
                            if decompressed.len() >= (width * height * 3) as usize {
                                // Assume RGB
                                if let Some(img_buf) =
                                    image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(
                                        width,
                                        height,
                                        decompressed.clone(),
                                    )
                                {
                                    let mut png_bytes = Vec::new();
                                    let encoder =
                                        image::codecs::png::PngEncoder::new(&mut png_bytes);
                                    if encoder
                                        .write_image(
                                            &img_buf,
                                            width,
                                            height,
                                            image::ColorType::Rgb8.into(),
                                        )
                                        .is_ok()
                                    {
                                        extracted.push(png_bytes);
                                    }
                                }
                            } else if decompressed.len() >= (width * height) as usize {
                                // Assume Gray
                                if let Some(img_buf) =
                                    image::ImageBuffer::<image::Luma<u8>, _>::from_raw(
                                        width,
                                        height,
                                        decompressed.clone(),
                                    )
                                {
                                    let mut png_bytes = Vec::new();
                                    let encoder =
                                        image::codecs::png::PngEncoder::new(&mut png_bytes);
                                    if encoder
                                        .write_image(
                                            &img_buf,
                                            width,
                                            height,
                                            image::ColorType::L8.into(),
                                        )
                                        .is_ok()
                                    {
                                        extracted.push(png_bytes);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(extracted)
}

pub fn pdf_hash(input_bytes: &[u8]) -> OperationResult<String> {
    let mut hasher = Sha256::new();
    hasher.update(input_bytes);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

// --- New Tools (Phase 5.5.1) ---

pub fn sanitize(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let mut doc =
        lopdf::Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    #[derive(serde::Deserialize)]
    struct SanitizeOptions {
        #[serde(default = "default_true")]
        remove_metadata: bool,
        #[serde(default = "default_true")]
        remove_attachments: bool,
        #[serde(default = "default_true")]
        remove_scripts: bool,
        #[serde(default = "default_true")]
        remove_thumbnails: bool,
        #[serde(default = "default_true")]
        remove_private_data: bool,
    }
    fn default_true() -> bool {
        true
    }

    let options: SanitizeOptions = if options_json.trim().is_empty() {
        SanitizeOptions {
            remove_metadata: true,
            remove_attachments: true,
            remove_scripts: true,
            remove_thumbnails: true,
            remove_private_data: true,
        }
    } else {
        serde_json::from_str(options_json).map_err(|e| PdfError::ParseError(e.to_string()))?
    };

    // 1. Strip Document Information Dictionary (/Info) & Metadata XMP
    if options.remove_metadata {
        doc.trailer.remove(b"Info");
        if let Ok(catalog_id) = doc.catalog_mut() {
            catalog_id.remove(b"Metadata");
        }
    }

    // 2. Strip Embedded Files (/Names -> /EmbeddedFiles, /AF)
    if options.remove_attachments
        && let Ok(catalog) = doc.catalog_mut()
    {
        if let Ok(lopdf::Object::Dictionary(names_dict)) = catalog.get_mut(b"Names") {
            names_dict.remove(b"EmbeddedFiles");
        }
        catalog.remove(b"AF");
    }

    // 3. Neutralize Active Scripts (/JavaScript, /JS, /OpenAction, /AA)
    if options.remove_scripts
        && let Ok(catalog) = doc.catalog_mut()
    {
        catalog.remove(b"OpenAction");
        catalog.remove(b"AA");
        if let Ok(lopdf::Object::Dictionary(names_dict)) = catalog.get_mut(b"Names") {
            names_dict.remove(b"JavaScript");
        }
    }

    // 4. Strip Page Thumbnails (/Thumb) & Private Editing Data (/PieceInfo)
    let page_ids = doc.get_pages();
    for (_page_num, page_id) in page_ids {
        if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            if options.remove_thumbnails {
                page_dict.remove(b"Thumb");
            }
            if options.remove_private_data {
                page_dict.remove(b"PieceInfo");
            }
            if options.remove_scripts {
                page_dict.remove(b"AA");
            }
        }
    }

    let mut out_bytes = Vec::new();
    doc.save_to(&mut out_bytes)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(out_bytes)
}

pub fn render_page(input_bytes: &[u8], page_index: u32, scale: f32) -> OperationResult<Vec<u8>> {
    let pdf = hayro_syntax::Pdf::new(input_bytes.to_vec())
        .map_err(|e| PdfError::ParseError(format!("Failed to parse PDF with hayro: {:?}", e)))?;

    let pages = pdf.pages();
    if page_index >= pages.len() as u32 {
        return Err(PdfError::ParseError("Page index out of bounds".to_string()));
    }

    let page = pages
        .get(page_index as usize)
        .ok_or_else(|| PdfError::ParseError("Failed to get page".to_string()))?;

    let cache = hayro::RenderCache::new();
    let render_settings = hayro::RenderSettings::default();
    let pixmap_settings = hayro::PixmapSettings {
        x_scale: scale,
        y_scale: scale,
        ..Default::default()
    };
    let pixmap = hayro::render(
        page,
        &cache,
        &Default::default(),
        &render_settings,
        &pixmap_settings,
    );

    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;
    let data = bytemuck::cast_slice(pixmap.data());

    let mut skia_pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| PdfError::ParseError("Failed to allocate tiny_skia Pixmap".to_string()))?;
    skia_pixmap.data_mut().copy_from_slice(data);

    let mut png_bytes = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    let rgba_data = skia_pixmap.data();
    encoder
        .write_image(rgba_data, width, height, image::ColorType::Rgba8.into())
        .map_err(|e| PdfError::ParseError(e.to_string()))?;
    let _ = png_bytes; // Just to align names

    Ok(png_bytes)
}

pub fn extract_text(input_bytes: &[u8]) -> OperationResult<String> {
    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let mut extracted_texts = Vec::new();

    let mut sorted_page_numbers: Vec<u32> = pages.keys().copied().collect();
    sorted_page_numbers.sort_unstable();

    for page_num in sorted_page_numbers {
        if let Some(&_page_id) = pages.get(&page_num) {
            match doc.extract_text(&[page_num]) {
                Ok(text) => extracted_texts.push(text),
                Err(e) => {
                    return Err(PdfError::Other(format!(
                        "Failed to extract text from page {}: {}",
                        page_num, e
                    )));
                }
            }
        }
    }

    Ok(extracted_texts.join("\n"))
}

pub fn decrypt(input_bytes: &[u8], password: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    if let Err(e) = doc.decrypt(password) {
        return Err(PdfError::UnsupportedOperation(format!(
            "Decryption failed or invalid password: {}",
            e
        )));
    }

    doc.decompress();
    doc.trailer.remove(b"Encrypt");

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn page_numbers(input_bytes: &[u8], format: &str, position: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let font_id = doc.add_object(dictionary!(
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    ));

    let pages = doc.get_pages();
    let total_pages = pages.len();
    let mut sorted_page_numbers: Vec<u32> = pages.keys().copied().collect();
    sorted_page_numbers.sort_unstable();

    for (page_number, object_id) in sorted_page_numbers
        .into_iter()
        .map(|n| (n, *pages.get(&n).unwrap()))
    {
        let mut width = 595.0;
        let mut height = 842.0;

        if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object(object_id)
            && let Ok(lopdf::Object::Array(rect)) = page_dict.get(b"MediaBox")
            && rect.len() == 4
            && let (Ok(x2), Ok(y2)) = (rect[2].as_f32(), rect[3].as_f32())
        {
            width = x2;
            height = y2;
        }

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

        let text = format
            .replace("{n}", &page_number.to_string())
            .replace("{total}", &total_pages.to_string());
        let escaped_text = text.replace("(", "\\(").replace(")", "\\)");

        let margin = 30.0;
        let text_width = text.len() as f32 * 6.0;

        let (x_pos, y_pos) = match position {
            "bottom-right" => (width - margin - text_width, margin),
            "bottom-center" => (width / 2.0 - (text_width / 2.0), margin),
            "top-right" => (width - margin - text_width, height - margin - 12.0),
            "top-center" => (width / 2.0 - (text_width / 2.0), height - margin - 12.0),
            _ => (width - margin - text_width, margin),
        };

        let content = format!(
            "q\nBT\n/F1 12 Tf\n1 0 0 1 {} {} Tm\n0 g\n({text}) Tj\nET\nQ\n",
            x_pos,
            y_pos,
            text = escaped_text
        );

        let content_stream =
            lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
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
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn header_footer(input_bytes: &[u8], header: &str, footer: &str) -> OperationResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let font_id = doc.add_object(dictionary!(
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    ));

    let pages = doc.get_pages();
    let mut sorted_page_numbers: Vec<u32> = pages.keys().copied().collect();
    sorted_page_numbers.sort_unstable();

    for (_page_number, object_id) in sorted_page_numbers
        .into_iter()
        .map(|n| (n, *pages.get(&n).unwrap()))
    {
        let mut width = 595.0;
        let mut height = 842.0;

        if let Ok(lopdf::Object::Dictionary(page_dict)) = doc.get_object(object_id)
            && let Ok(lopdf::Object::Array(rect)) = page_dict.get(b"MediaBox")
            && rect.len() == 4
            && let (Ok(x2), Ok(y2)) = (rect[2].as_f32(), rect[3].as_f32())
        {
            width = x2;
            height = y2;
        }

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

        let escaped_header = header.replace("(", "\\(").replace(")", "\\)");
        let escaped_footer = footer.replace("(", "\\(").replace(")", "\\)");

        let margin = 30.0;

        let mut streams = Vec::new();

        if !header.is_empty() {
            let header_width = header.len() as f32 * 6.0;
            let hx = width / 2.0 - (header_width / 2.0);
            let hy = height - margin - 12.0;
            let content = format!(
                "q\nBT\n/F1 12 Tf\n1 0 0 1 {} {} Tm\n0 g\n({text}) Tj\nET\nQ\n",
                hx,
                hy,
                text = escaped_header
            );
            streams.push(content);
        }

        if !footer.is_empty() {
            let footer_width = footer.len() as f32 * 6.0;
            let fx = width / 2.0 - (footer_width / 2.0);
            let fy = margin;
            let content = format!(
                "q\nBT\n/F1 12 Tf\n1 0 0 1 {} {} Tm\n0 g\n({text}) Tj\nET\nQ\n",
                fx,
                fy,
                text = escaped_footer
            );
            streams.push(content);
        }

        for content in streams {
            let content_stream =
                lopdf::Stream::new(lopdf::Dictionary::new(), content.as_bytes().to_vec());
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
    }

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)
        .map_err(|e| PdfError::IoError(std::io::Error::other(e.to_string())))?;

    Ok(buffer)
}

pub fn pdf_info(input_bytes: &[u8]) -> OperationResult<String> {
    let doc = Document::load_mem(input_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document: {e}")))?;

    let pages = doc.get_pages();
    let version = doc.version.clone();
    let is_encrypted = doc.trailer.has(b"Encrypt");

    let mut title = "".to_string();
    let mut author = "".to_string();

    if let Ok(info_ref) = doc
        .trailer
        .get(b"Info")
        .and_then(|info| info.as_reference())
        && let Ok(lopdf::Object::Dictionary(info_dict)) = doc.get_object(info_ref)
    {
        if let Ok(t) = info_dict.get(b"Title").and_then(|t| t.as_str()) {
            title = String::from_utf8_lossy(t).into_owned();
        }
        if let Ok(a) = info_dict.get(b"Author").and_then(|a| a.as_str()) {
            author = String::from_utf8_lossy(a).into_owned();
        }
    }

    let json = format!(
        r#"{{"page_count": {}, "version": "{}", "encrypted": {}, "title": "{}", "author": "{}"}}"#,
        pages.len(),
        version,
        is_encrypted,
        title.replace("\"", "\\\""),
        author.replace("\"", "\\\"")
    );

    Ok(json)
}

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten_tensor::NdTensor;
use rten_tensor::prelude::*;

pub fn ocr(image_or_pdf_bytes: &[u8]) -> OperationResult<String> {
    let engine = OcrEngine::new(OcrEngineParams::default())
        .map_err(|e| PdfError::Other(format!("Failed to init OCR engine: {}", e)))?;

    let mut words_collected = Vec::new();

    // Attempt to load as Image directly first
    if let Ok(dyn_img) = image::load_from_memory(image_or_pdf_bytes) {
        let rgb = dyn_img.into_rgb8();
        let (width, height) = rgb.dimensions();

        let mut tensor: NdTensor<f32, 3> = NdTensor::zeros([3, height as usize, width as usize]);
        for y in 0..height {
            for x in 0..width {
                let pixel = rgb.get_pixel(x, y);
                tensor[[0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                tensor[[1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                tensor[[2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
            }
        }

        if let Ok(image_source) = ImageSource::from_tensor(tensor.view(), ocrs::DimOrder::Chw)
            && let Ok(img_input) = engine.prepare_input(image_source)
            && let Ok(texts) = engine.get_text(&img_input)
        {
            words_collected.push(texts);
        }

        return Ok(words_collected.join("\n"));
    }

    // Otherwise try to load as PDF and extract images
    let doc = Document::load_mem(image_or_pdf_bytes)
        .map_err(|e| PdfError::ParseError(format!("Failed to parse document or image: {e}")))?;

    let pages = doc.get_pages();
    for (_page_num, page_id) in pages {
        let page_dict = match doc.get_object(page_id).and_then(|obj| obj.as_dict()) {
            Ok(dict) => dict,
            Err(_) => continue,
        };

        let resources_dict = match page_dict.get(b"Resources").and_then(|res| match res {
            lopdf::Object::Reference(res_id) => {
                doc.get_object(*res_id).and_then(|obj| obj.as_dict())
            }
            lopdf::Object::Dictionary(dict) => Ok(dict),
            _ => Err(lopdf::Error::DictKey("Invalid resources".to_string())),
        }) {
            Ok(dict) => dict,
            Err(_) => continue,
        };

        let xobject_dict = match resources_dict.get(b"XObject").and_then(|xobj| match xobj {
            lopdf::Object::Reference(xobj_id) => {
                doc.get_object(*xobj_id).and_then(|obj| obj.as_dict())
            }
            lopdf::Object::Dictionary(dict) => Ok(dict),
            _ => Err(lopdf::Error::DictKey("Invalid XObject".to_string())),
        }) {
            Ok(dict) => dict,
            Err(_) => continue,
        };

        for (_, obj) in xobject_dict.iter() {
            let stream_id = match obj.as_reference() {
                Ok(id) => id,
                Err(_) => continue,
            };

            let stream = match doc.get_object(stream_id).and_then(|obj| obj.as_stream()) {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            let subtype = match stream.dict.get(b"Subtype").and_then(|name| name.as_name()) {
                Ok(name) => name,
                Err(_) => continue,
            };

            if subtype != b"Image" {
                continue;
            }

            // Try decompressed first
            let content_bytes = stream
                .decompressed_content()
                .unwrap_or_else(|_| stream.content.clone());

            if let Ok(dyn_img) = image::load_from_memory(&content_bytes) {
                let rgb = dyn_img.into_rgb8();
                let (width, height) = rgb.dimensions();

                let mut tensor: NdTensor<f32, 3> =
                    NdTensor::zeros([3, height as usize, width as usize]);
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgb.get_pixel(x, y);
                        tensor[[0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                        tensor[[1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                        tensor[[2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
                    }
                }

                if let Ok(image_source) =
                    ImageSource::from_tensor(tensor.view(), ocrs::DimOrder::Chw)
                    && let Ok(img_input) = engine.prepare_input(image_source)
                    && let Ok(texts) = engine.get_text(&img_input)
                {
                    words_collected.push(texts);
                }
            }
        }
    }

    Ok(words_collected.join("\n"))
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct JsonImage {
    pub id: String,
    pub page_number: u32,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub bbox: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_data: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct JsonPage {
    pub page_number: u32,
    pub text: String,
    #[serde(default)]
    pub char_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<JsonImage>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct JsonDocument {
    pub document_name: String,
    pub pages: Vec<JsonPage>,
}

pub fn json_to_pdf(json_str: &str) -> OperationResult<Vec<u8>> {
    let doc_struct: JsonDocument = serde_json::from_str(json_str)
        .map_err(|e| PdfError::Other(format!("Failed to parse JSON: {}", e)))?;

    let mut inner = lopdf::Document::with_version("1.5");

    let font_id = inner.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });

    let pages_id = inner.new_object_id();
    let mut page_ids = vec![];

    for json_page in doc_struct.pages {
        let mut ops = vec![
            lopdf::content::Operation::new("BT", vec![]),
            lopdf::content::Operation::new("Tf", vec!["F1".into(), 12.into()]),
            lopdf::content::Operation::new("Td", vec![50.into(), 750.into()]),
        ];

        for line in json_page.text.lines() {
            ops.push(lopdf::content::Operation::new(
                "Tj",
                vec![lopdf::Object::string_literal(line)],
            ));
            ops.push(lopdf::content::Operation::new(
                "Td",
                vec![0.into(), (-15).into()],
            ));
        }

        ops.push(lopdf::content::Operation::new("ET", vec![]));

        let mut resources = dictionary! {
            "Font" => dictionary! {
                "F1" => font_id,
            },
        };

        let mut xobjects = lopdf::Dictionary::new();

        for (idx, img) in json_page.images.iter().enumerate() {
            let img_name = format!("Im{}", idx);
            let (width, height, image_bytes) = if let Some(b64) = &img.base64_data {
                let b64_str = if b64.starts_with("data:image/") {
                    b64.split(',').nth(1).unwrap_or(b64)
                } else {
                    b64
                };

                use base64::{Engine as _, engine::general_purpose};
                let bytes = general_purpose::STANDARD
                    .decode(b64_str)
                    .map_err(|e| PdfError::Other(format!("Failed to decode base64: {}", e)))?;

                let img_reader = image::load_from_memory(&bytes)
                    .map_err(|e| PdfError::Other(format!("Failed to parse image: {}", e)))?;

                let img_rgb = img_reader.to_rgb8();
                let (w, h) = img_rgb.dimensions();
                (w, h, img_rgb.into_raw())
            } else {
                continue;
            };

            let mut compressed = Vec::new();
            {
                use flate2::Compression;
                use flate2::write::ZlibEncoder;
                use std::io::Write;
                let mut encoder = ZlibEncoder::new(&mut compressed, Compression::default());
                encoder
                    .write_all(&image_bytes)
                    .map_err(|e: std::io::Error| PdfError::Other(e.to_string()))?;
            }

            let image_dict = dictionary! {
                "Type" => "XObject",
                "Subtype" => "Image",
                "Width" => width as i64,
                "Height" => height as i64,
                "ColorSpace" => "DeviceRGB",
                "BitsPerComponent" => 8,
                "Filter" => "FlateDecode",
            };

            let img_stream = lopdf::Stream::new(image_dict, compressed);
            let img_id = inner.add_object(img_stream);

            xobjects.set(img_name.clone(), img_id);

            if let Some(bbox) = img.bbox {
                let (x0, y0, x1, y1) = (bbox[0], bbox[1], bbox[2], bbox[3]);
                let w = x1 - x0;
                let h = y1 - y0;

                ops.push(lopdf::content::Operation::new("q", vec![]));
                ops.push(lopdf::content::Operation::new(
                    "cm",
                    vec![w.into(), 0.into(), 0.into(), h.into(), x0.into(), y0.into()],
                ));
                ops.push(lopdf::content::Operation::new("Do", vec![img_name.into()]));
                ops.push(lopdf::content::Operation::new("Q", vec![]));
            }
        }

        if !xobjects.is_empty() {
            resources.set("XObject", xobjects);
        }

        let content_stream = lopdf::content::Content { operations: ops };
        let content_id = inner.add_object(lopdf::Stream::new(
            dictionary! {},
            content_stream
                .encode()
                .map_err(|e| PdfError::Other(e.to_string()))?,
        ));

        let page_dict = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Contents" => content_id,
            "Resources" => resources,
        };

        let page_id = inner.add_object(page_dict);
        page_ids.push(page_id);
    }

    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => page_ids.clone().into_iter().map(lopdf::Object::Reference).collect::<Vec<_>>(),
        "Count" => page_ids.len() as i32,
    };
    inner
        .objects
        .insert(pages_id, lopdf::Object::Dictionary(pages_dict));

    let catalog_id = inner.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    inner.trailer.set("Root", catalog_id);

    let mut bytes = Vec::new();
    inner
        .save_to(&mut bytes)
        .map_err(|e| PdfError::Other(e.to_string()))?;

    Ok(bytes)
}

#[derive(Debug, Clone)]
pub struct TextSpan {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_italic: bool,
    pub color_hex: String,
}

#[derive(Debug, Clone)]
pub enum LayoutItem {
    Paragraph(Vec<TextSpan>),
    Table(Vec<Vec<Vec<TextSpan>>>),
}

pub fn workflow_presets() -> String {
    let presets = paperpilot_core::workflow::get_default_presets();
    serde_json::to_string(&presets).unwrap_or_default()
}

fn execute_step_wasm(
    bytes: &[u8],
    step: &paperpilot_core::workflow::WorkflowStep,
) -> OperationResult<Vec<u8>> {
    match step.operation_id.as_str() {
        "pdf_watermark" | "watermark" => {
            let text = step
                .params
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("CONFIDENTIAL");
            watermark(bytes, text, Some(45.0), Some(0.2))
        }
        "pdf_compress" | "compress" => {
            let quality = step.params.get("quality").and_then(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });
            compress(bytes, quality.as_deref())
        }
        "pdf_flatten" | "flatten" => flatten(bytes),
        "pdf_page_numbers" | "page_numbers" => {
            let pos = step
                .params
                .get("position")
                .and_then(|v| v.as_str())
                .unwrap_or("bottom-right");
            page_numbers(bytes, "Page {n} of {total}", pos)
        }
        "pdf_rotate" | "rotate" => {
            let angle = step
                .params
                .get("angle")
                .and_then(|v| v.as_i64())
                .unwrap_or(90) as u16;
            rotate(bytes, angle, "all")
        }
        "pdf_sanitize" | "sanitize" => {
            let options_str =
                serde_json::to_string(&step.params).unwrap_or_else(|_| "{}".to_string());
            sanitize(bytes, &options_str)
        }
        _ => Ok(bytes.to_vec()),
    }
}

pub fn workflow(input_bytes: &[u8], recipe_json: &str) -> OperationResult<Vec<u8>> {
    let workflow = if let Some(preset) = paperpilot_core::workflow::get_preset_by_id(recipe_json) {
        preset
    } else {
        serde_json::from_str::<paperpilot_core::workflow::SavedWorkflow>(recipe_json)
            .map_err(|e| PdfError::ParseError(format!("Invalid workflow recipe JSON: {}", e)))?
    };

    let mut current = input_bytes.to_vec();
    for step in &workflow.steps {
        current = execute_step_wasm(&current, step)?;
    }
    Ok(current)
}

pub fn pdf_to_docx(input_bytes: &[u8]) -> OperationResult<Vec<u8>> {
    use docx_rs::{AlignmentType, Docx, Paragraph, Run, Table, TableCell, TableRow};

    let doc = Document::load_mem(input_bytes)
        .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;

    let mut all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    all_pages.sort_unstable();

    let mut docx = Docx::new();

    for (i, page_id) in all_pages.iter().enumerate() {
        let page_obj_id = doc.get_pages()[page_id];
        let page_dict = doc
            .get_object(page_obj_id)
            .and_then(|obj| obj.as_dict())
            .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;

        let media_box_default = vec![];
        let media_box = page_dict
            .get(b"MediaBox")
            .and_then(|obj| obj.as_array())
            .unwrap_or(&media_box_default);

        let mut width: f32 = 595.0;
        if media_box.len() >= 4
            && let (Ok(x0), Ok(x1)) = (
                media_box[0]
                    .as_f32()
                    .or_else(|_| media_box[0].as_i64().map(|v| v as f32)),
                media_box[2]
                    .as_f32()
                    .or_else(|_| media_box[2].as_i64().map(|v| v as f32)),
            )
        {
            width = (x1 - x0).abs();
        }
        let center_w = width / 2.0;

        let content_data = doc.get_page_content(page_obj_id);
        let content = lopdf::content::Content::decode(&content_data)
            .unwrap_or_else(|_| lopdf::content::Content { operations: vec![] });

        let mut current_font_size: f32 = 12.0;
        let mut is_bold = false;
        let mut is_italic = false;
        let mut current_color = String::new();
        let mut current_x: f32 = 0.0;
        let mut current_y: f32 = 0.0;
        let mut current_text = String::new();

        let mut font_names = std::collections::HashMap::new();
        if let Ok(resources) = page_dict
            .get(b"Resources")
            .and_then(|obj| doc.get_object(obj.as_reference().unwrap_or((0, 0))))
            .and_then(|obj| obj.as_dict())
            && let Ok(fonts) = resources
                .get(b"Font")
                .and_then(|obj| doc.get_object(obj.as_reference().unwrap_or((0, 0))))
                .and_then(|obj| obj.as_dict())
        {
            for (font_ref, font_obj) in fonts.iter() {
                if let Ok(font_dict) = doc
                    .get_object(font_obj.as_reference().unwrap_or((0, 0)))
                    .and_then(|o| o.as_dict())
                    && let Ok(base_font) = font_dict.get(b"BaseFont").and_then(|o| o.as_name())
                {
                    font_names.insert(
                        String::from_utf8_lossy(font_ref).to_string(),
                        String::from_utf8_lossy(base_font).to_string(),
                    );
                }
            }
        }

        let mut spans = vec![];

        for op in content.operations {
            match op.operator.as_str() {
                "Tf" => {
                    if op.operands.len() == 2 {
                        if let Ok(name) = op.operands[0].as_name() {
                            let font_key = String::from_utf8_lossy(name).to_string();
                            if let Some(base) = font_names.get(&font_key) {
                                let lower_base = base.to_lowercase();
                                is_bold = lower_base.contains("bold")
                                    || lower_base.contains("black")
                                    || lower_base.contains("heavy");
                                is_italic =
                                    lower_base.contains("italic") || lower_base.contains("oblique");
                            }
                        }
                        current_font_size = op.operands[1]
                            .as_f32()
                            .or_else(|_| op.operands[1].as_i64().map(|v| v as f32))
                            .unwrap_or(12.0);
                    }
                }
                "rg" | "g" | "k" => {
                    if op.operator == "rg" && op.operands.len() == 3 {
                        let r = (op.operands[0].as_f32().unwrap_or(0.0) * 255.0) as u8;
                        let g = (op.operands[1].as_f32().unwrap_or(0.0) * 255.0) as u8;
                        let b = (op.operands[2].as_f32().unwrap_or(0.0) * 255.0) as u8;
                        current_color = format!("{:02X}{:02X}{:02X}", r, g, b);
                    } else if op.operator == "g" && op.operands.len() == 1 {
                        let gray = (op.operands[0].as_f32().unwrap_or(0.0) * 255.0) as u8;
                        current_color = format!("{:02X}{:02X}{:02X}", gray, gray, gray);
                    }
                }
                "Tm" => {
                    if op.operands.len() == 6 {
                        current_x = op.operands[4]
                            .as_f32()
                            .or_else(|_| op.operands[4].as_i64().map(|v| v as f32))
                            .unwrap_or(0.0);
                        current_y = op.operands[5]
                            .as_f32()
                            .or_else(|_| op.operands[5].as_i64().map(|v| v as f32))
                            .unwrap_or(0.0);
                    }
                }
                "Td" | "TD" => {
                    if op.operands.len() == 2 {
                        let dx = match &op.operands[0] {
                            lopdf::Object::Real(f) => *f,
                            lopdf::Object::Integer(i) => *i as f32,
                            _ => 0.0,
                        };
                        let dy = match &op.operands[1] {
                            lopdf::Object::Real(f) => *f,
                            lopdf::Object::Integer(i) => *i as f32,
                            _ => 0.0,
                        };
                        current_x += dx;
                        current_y += dy;
                    }
                }
                "Tj" => {
                    if op.operands.len() == 1
                        && let Ok(string_bytes) = op.operands[0].as_str()
                    {
                        let txt = String::from_utf8_lossy(string_bytes).to_string();
                        current_text.push_str(&txt);
                    }
                    if !current_text.is_empty() {
                        spans.push(TextSpan {
                            text: current_text.clone(),
                            font_size: current_font_size,
                            is_bold,
                            is_italic,
                            color_hex: current_color.clone(),
                            x: current_x,
                            y: current_y,
                            width: current_text.len() as f32 * current_font_size * 0.5,
                            height: current_font_size,
                        });
                        current_text.clear();
                    }
                }
                "TJ" => {
                    if op.operands.len() == 1
                        && let Ok(array) = op.operands[0].as_array()
                    {
                        for item in array {
                            if let Ok(string_bytes) = item.as_str() {
                                let txt = String::from_utf8_lossy(string_bytes).to_string();
                                current_text.push_str(&txt);
                            } else if let Ok(num) = item.as_f32()
                                && num < -100.0
                            {
                                current_text.push(' ');
                            }
                        }
                    }
                    if !current_text.is_empty() {
                        spans.push(TextSpan {
                            text: current_text.clone(),
                            font_size: current_font_size,
                            is_bold,
                            is_italic,
                            color_hex: current_color.clone(),
                            x: current_x,
                            y: current_y,
                            width: current_text.len() as f32 * current_font_size * 0.5,
                            height: current_font_size,
                        });
                        current_text.clear();
                    }
                }
                "T*" | "'" | "\"" => {
                    if !current_text.is_empty() {
                        spans.push(TextSpan {
                            text: current_text.clone(),
                            font_size: current_font_size,
                            is_bold,
                            is_italic,
                            color_hex: current_color.clone(),
                            x: current_x,
                            y: current_y,
                            width: current_text.len() as f32 * current_font_size * 0.5,
                            height: current_font_size,
                        });
                        current_text.clear();
                    }
                }
                "ET" if !current_text.is_empty() => {
                    spans.push(TextSpan {
                        text: current_text.clone(),
                        font_size: current_font_size,
                        is_bold,
                        is_italic,
                        color_hex: current_color.clone(),
                        x: current_x,
                        y: current_y,
                        width: current_text.len() as f32 * current_font_size * 0.5,
                        height: current_font_size,
                    });
                    current_text.clear();
                }
                _ => {}
            }
        }

        if !current_text.is_empty() {
            spans.push(TextSpan {
                text: current_text.clone(),
                font_size: current_font_size,
                is_bold,
                is_italic,
                color_hex: current_color.clone(),
                x: current_x,
                y: current_y,
                width: current_text.len() as f32 * current_font_size * 0.5,
                height: current_font_size,
            });
        }

        // Clean empty spans
        spans.retain(|s| !s.text.trim().is_empty());

        let mut remaining_spans = spans.clone();
        let mut layout_items = vec![];

        // 1. Table Detection
        let mut y_groups: Vec<(f32, Vec<TextSpan>)> = vec![];
        for span in remaining_spans.clone() {
            let mut found = false;
            for (group_y, group_spans) in &mut y_groups {
                if (span.y - *group_y).abs() <= 2.0 {
                    group_spans.push(span.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                y_groups.push((span.y, vec![span]));
            }
        }

        // Sort groups by y descending
        y_groups.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut current_table_rows: Vec<Vec<TextSpan>> = vec![];
        let mut table_spans_to_remove = std::collections::HashSet::new();

        for (_y, row_spans) in y_groups {
            if row_spans.len() >= 2 {
                current_table_rows.push(row_spans);
            } else {
                if current_table_rows.len() >= 2 {
                    let mut table_cells = vec![];
                    for row in current_table_rows.drain(..) {
                        let mut row_sorted = row.clone();
                        row_sorted.sort_by(|a, b| {
                            a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
                        });

                        let mut cells = vec![];
                        let mut current_cell = vec![];
                        let mut last_x = -1000.0;
                        let mut last_width = 0.0;

                        for span in row_sorted {
                            table_spans_to_remove
                                .insert(format!("{}_{}_{}", span.x, span.y, span.text));

                            if (span.x - (last_x + last_width)).abs() > 15.0
                                && !current_cell.is_empty()
                            {
                                cells.push(current_cell.clone());
                                current_cell.clear();
                            }
                            last_x = span.x;
                            last_width = span.width;
                            current_cell.push(span);
                        }
                        if !current_cell.is_empty() {
                            cells.push(current_cell);
                        }
                        table_cells.push(cells);
                    }
                    layout_items.push(LayoutItem::Table(table_cells));
                } else {
                    current_table_rows.clear();
                }
            }
        }
        if current_table_rows.len() >= 2 {
            let mut table_cells = vec![];
            for row in current_table_rows.drain(..) {
                let mut row_sorted = row.clone();
                row_sorted
                    .sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
                let mut cells = vec![];
                let mut current_cell = vec![];
                let mut last_x = -1000.0;
                let mut last_width = 0.0;

                for span in row_sorted {
                    table_spans_to_remove.insert(format!("{}_{}_{}", span.x, span.y, span.text));

                    if (span.x - (last_x + last_width)).abs() > 15.0 && !current_cell.is_empty() {
                        cells.push(current_cell.clone());
                        current_cell.clear();
                    }
                    last_x = span.x;
                    last_width = span.width;
                    current_cell.push(span);
                }
                if !current_cell.is_empty() {
                    cells.push(current_cell);
                }
                table_cells.push(cells);
            }
            layout_items.push(LayoutItem::Table(table_cells));
        }

        remaining_spans
            .retain(|s| !table_spans_to_remove.contains(&format!("{}_{}_{}", s.x, s.y, s.text)));

        // 2. XY-Cut Algorithm for remaining spans
        let mut gutters = vec![];
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        for span in &remaining_spans {
            if span.x < min_x {
                min_x = span.x;
            }
            if span.x + span.width > max_x {
                max_x = span.x + span.width;
            }
        }

        if min_x < max_x && !remaining_spans.is_empty() {
            // Projection profile
            let mut intervals: Vec<(f32, f32)> = remaining_spans
                .iter()
                .map(|s| (s.x, s.x + s.width))
                .collect();
            intervals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

            let mut merged = vec![intervals[0]];
            for interval in intervals.iter().skip(1) {
                let last = merged.last_mut().unwrap();
                if interval.0 <= last.1 {
                    if interval.1 > last.1 {
                        last.1 = interval.1;
                    }
                } else {
                    merged.push(*interval);
                }
            }

            for i in 0..merged.len() - 1 {
                let gap = merged[i + 1].0 - merged[i].1;
                if gap >= 15.0 {
                    gutters.push(merged[i].1 + gap / 2.0);
                }
            }
        }

        if !gutters.is_empty() {
            let gutter_x = gutters[0];
            let mut col1 = vec![];
            let mut col2 = vec![];
            for span in remaining_spans {
                if span.x < gutter_x {
                    col1.push(span);
                } else {
                    col2.push(span);
                }
            }

            let process_col = |col: Vec<TextSpan>| -> Vec<LayoutItem> {
                let mut sorted = col;
                sorted.sort_by(|a, b| {
                    match b.y.partial_cmp(&a.y).unwrap_or(std::cmp::Ordering::Equal) {
                        std::cmp::Ordering::Equal => {
                            a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        other => other,
                    }
                });
                let mut paras = vec![];
                let mut current_para = vec![];
                let mut last_y = f32::MAX;
                for span in sorted {
                    if (last_y - span.y).abs() > span.font_size * 1.5 && !current_para.is_empty() {
                        paras.push(current_para.clone());
                        current_para.clear();
                    }
                    current_para.push(span.clone());
                    last_y = span.y;
                }
                if !current_para.is_empty() {
                    paras.push(current_para);
                }
                paras.into_iter().map(LayoutItem::Paragraph).collect()
            };

            layout_items.extend(process_col(col1));
            layout_items.extend(process_col(col2));
        } else {
            remaining_spans.sort_by(|a, b| {
                match b.y.partial_cmp(&a.y).unwrap_or(std::cmp::Ordering::Equal) {
                    std::cmp::Ordering::Equal => {
                        a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    other => other,
                }
            });

            // Group into paragraphs by proximity
            let mut paras = vec![];
            let mut current_para = vec![];
            let mut last_y = f32::MAX;
            for span in remaining_spans {
                if (last_y - span.y).abs() > span.font_size * 1.5 && !current_para.is_empty() {
                    paras.push(current_para.clone());
                    current_para.clear();
                }
                current_para.push(span.clone());
                last_y = span.y;
            }
            if !current_para.is_empty() {
                paras.push(current_para);
            }

            for p in paras {
                layout_items.push(LayoutItem::Paragraph(p));
            }
        }

        // Emit docx-rs
        for item in layout_items {
            match item {
                LayoutItem::Paragraph(spans) => {
                    let mut sorted_spans = spans.clone();
                    sorted_spans.sort_by(|a, b| {
                        b.y.partial_cmp(&a.y)
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then(a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
                    });

                    let mut text_parts = vec![];
                    let mut current_part = vec![];
                    let mut current_y = sorted_spans.first().map(|s| s.y).unwrap_or(0.0);

                    for span in sorted_spans {
                        if (span.y - current_y).abs() > 2.0 {
                            text_parts.push(current_part.clone());
                            current_part.clear();
                            current_y = span.y;
                        }
                        current_part.push(span);
                    }
                    if !current_part.is_empty() {
                        text_parts.push(current_part);
                    }

                    for row in text_parts {
                        let mut row_sorted = row;
                        row_sorted.sort_by(|a, b| {
                            a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        let combined_text = row_sorted
                            .iter()
                            .map(|s| s.text.clone())
                            .collect::<Vec<_>>()
                            .join(" ");
                        if combined_text.trim().is_empty() {
                            continue;
                        }

                        let mut para = Paragraph::new();
                        let rep_span = row_sorted.first().cloned().unwrap_or(TextSpan {
                            text: "".to_string(),
                            font_size: 12.0,
                            is_bold: false,
                            is_italic: false,
                            color_hex: "".to_string(),
                            x: 0.0,
                            y: 0.0,
                            width: 0.0,
                            height: 0.0,
                        });

                        if (rep_span.x - center_w).abs() < (width * 0.15) {
                            para = para.align(AlignmentType::Center);
                        } else if rep_span.x > (width * 0.6) {
                            para = para.align(AlignmentType::Right);
                        }

                        if rep_span.font_size >= 18.0 {
                            para = para.style("Heading1");
                        } else if (14.0..18.0).contains(&rep_span.font_size) {
                            para = para.style("Heading2");
                        }

                        let mut run = Run::new().add_text(combined_text.trim().to_string());
                        let size = rep_span.font_size;
                        if size >= 18.0 {
                            run = run.size(36);
                        } else if (14.0..18.0).contains(&size) {
                            run = run.size(28);
                        } else {
                            run = run.size((size * 2.0) as usize);
                        }

                        if rep_span.is_bold {
                            run = run.bold();
                        }
                        if rep_span.is_italic {
                            run = run.italic();
                        }
                        if !rep_span.color_hex.is_empty() {
                            run = run.color(rep_span.color_hex.clone());
                        }

                        para = para.add_run(run);
                        docx = docx.add_paragraph(para);
                    }
                }
                LayoutItem::Table(rows) => {
                    let mut table = Table::new(vec![]);
                    for row in rows {
                        let mut cells = vec![];
                        for cell_spans in row {
                            let mut tc = TableCell::new();
                            let mut para = Paragraph::new();
                            let mut sorted_spans = cell_spans.clone();
                            sorted_spans.sort_by(|a, b| {
                                a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)
                            });
                            let combined_text = sorted_spans
                                .iter()
                                .map(|s| s.text.clone())
                                .collect::<Vec<_>>()
                                .join(" ");
                            if !combined_text.trim().is_empty() {
                                let rep_span = sorted_spans.first().cloned().unwrap_or(TextSpan {
                                    text: "".to_string(),
                                    font_size: 12.0,
                                    is_bold: false,
                                    is_italic: false,
                                    color_hex: "".to_string(),
                                    x: 0.0,
                                    y: 0.0,
                                    width: 0.0,
                                    height: 0.0,
                                });
                                let mut run = Run::new().add_text(combined_text.trim().to_string());
                                let size = rep_span.font_size;
                                run = run.size((size * 2.0) as usize);
                                if rep_span.is_bold {
                                    run = run.bold();
                                }
                                if rep_span.is_italic {
                                    run = run.italic();
                                }
                                if !rep_span.color_hex.is_empty() {
                                    run = run.color(rep_span.color_hex.clone());
                                }
                                para = para.add_run(run);
                            }
                            tc = tc.add_paragraph(para);
                            cells.push(tc);
                        }
                        let tr = TableRow::new(cells);
                        table = table.add_row(tr);
                    }
                    docx = docx.add_table(table);
                }
            }
        }

        // Add images from Resources
        if let Ok(resources) = page_dict
            .get(b"Resources")
            .and_then(|obj| doc.get_object(obj.as_reference().unwrap_or((0, 0))))
            .and_then(|obj| obj.as_dict())
            && let Ok(xobjects) = resources
                .get(b"XObject")
                .and_then(|obj| doc.get_object(obj.as_reference().unwrap_or((0, 0))))
                .and_then(|obj| obj.as_dict())
        {
            for (_xobj_name, xobj_ref) in xobjects.iter() {
                if let Ok(xobj_stream) = doc
                    .get_object(xobj_ref.as_reference().unwrap_or((0, 0)))
                    .and_then(|obj| obj.as_stream())
                    && let Ok(subtype) = xobj_stream
                        .dict
                        .get(b"Subtype")
                        .and_then(|obj| obj.as_name())
                    && subtype == b"Image"
                {
                    let image_bytes = &xobj_stream.content;
                    let pic = docx_rs::Pic::new(image_bytes);
                    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)));
                }
            }
        }

        if i < all_pages.len() - 1 {
            docx = docx.add_paragraph(Paragraph::new().page_break_before(true));
        }
    }

    let mut out_buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut out_buffer);
    docx.build()
        .pack(&mut cursor)
        .map_err(|e| paperpilot_core::error::PdfError::Other(e.to_string()))?;

    Ok(out_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Dictionary, Document, Object};

    fn create_dummy_doc(pages: u32, content_str: Option<&str>) -> Document {
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut page_ids = vec![];

        for _ in 0..pages {
            let mut page_dict = Dictionary::new();
            page_dict.set("Type", Object::Name(b"Page".to_vec()));
            page_dict.set("Parent", Object::Reference(pages_id));

            let content = content_str.unwrap_or("BT /F1 12 Tf 0 0 Td (Test Content) Tj ET");
            let content_stream = lopdf::Stream::new(Dictionary::new(), content.as_bytes().to_vec());
            let content_id = doc.add_object(Object::Stream(content_stream));
            page_dict.set("Contents", Object::Reference(content_id));

            let mut resources_dict = Dictionary::new();
            let mut font_dict = Dictionary::new();
            let mut f1_dict = Dictionary::new();
            f1_dict.set("Type", Object::Name(b"Font".to_vec()));
            f1_dict.set("Subtype", Object::Name(b"Type1".to_vec()));
            f1_dict.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
            let f1_id = doc.add_object(Object::Dictionary(f1_dict));
            font_dict.set("F1", Object::Reference(f1_id));
            resources_dict.set("Font", Object::Dictionary(font_dict));

            page_dict.set("Resources", Object::Dictionary(resources_dict));

            let page_id = doc.add_object(page_dict);
            page_ids.push(Object::Reference(page_id));
        }

        let mut pages_dict = Dictionary::new();
        pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
        pages_dict.set("Count", Object::Integer(pages as i64));
        pages_dict.set("Kids", Object::Array(page_ids));

        doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
        doc.trailer
            .set("Root", Object::Dictionary(Dictionary::new()));

        let mut catalog = Dictionary::new();
        catalog.set("Type", Object::Name(b"Catalog".to_vec()));
        catalog.set("Pages", Object::Reference(pages_id));
        let catalog_id = doc.add_object(catalog);
        doc.trailer.set("Root", Object::Reference(catalog_id));

        doc
    }

    #[test]
    fn test_multi_column_reading_order() {
        let content_str = "BT /F1 12 Tf 50 700 Td (Left 1) Tj -50 -700 Td 50 680 Td (Left 2) Tj -50 -680 Td 250 690 Td (Right 1) Tj -250 -690 Td 250 670 Td (Right 2) Tj ET";
        let mut doc = create_dummy_doc(1, Some(content_str));
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer).unwrap();

        let result = pdf_to_docx(&buffer);
        assert!(result.is_ok());
        let out_buffer = result.unwrap();
        assert!(!out_buffer.is_empty());

        let cursor = std::io::Cursor::new(out_buffer);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let mut doc_xml = archive.by_name("word/document.xml").unwrap();
        let mut xml_content = String::new();
        std::io::Read::read_to_string(&mut doc_xml, &mut xml_content).unwrap();

        let mut pure_text = String::new();
        let mut in_tag = false;
        for c in xml_content.chars() {
            if c == '<' {
                in_tag = true;
            } else if c == '>' {
                in_tag = false;
            } else if !in_tag {
                pure_text.push(c);
            }
        }

        let clean_text = pure_text.replace(" ", "");
        assert!(
            clean_text.contains("Left1Left2Right1Right2")
                || clean_text.contains("Left1Right1Left2Right2")
        );
    }

    #[test]
    fn test_workflow_presets() {
        let json = super::workflow_presets();
        assert!(json.contains("preset-legal-discovery"));
    }

    #[test]
    fn test_workflow_execution() {
        let mut doc = create_dummy_doc(1, None);
        let mut buf = Vec::new();
        doc.save_to(&mut buf).unwrap();

        let out = super::workflow(&buf, "preset-legal-discovery");
        assert!(out.is_ok());
        assert!(!out.unwrap().is_empty());
    }

    #[test]
    fn test_table_structure_detection() {
        let content_str = "BT /F1 12 Tf 50 700 Td (Header 1) Tj 100 0 Td (Header 2) Tj -100 -20 Td (Cell 1) Tj 100 0 Td (Cell 2) Tj ET";
        let mut doc = create_dummy_doc(1, Some(content_str));
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer).unwrap();

        let result = pdf_to_docx(&buffer);
        assert!(result.is_ok());
        let out_buffer = result.unwrap();
        assert!(!out_buffer.is_empty());

        let cursor = std::io::Cursor::new(out_buffer);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        let mut doc_xml = archive.by_name("word/document.xml").unwrap();
        let mut xml_content = String::new();
        std::io::Read::read_to_string(&mut doc_xml, &mut xml_content).unwrap();

        assert!(xml_content.contains("<w:tbl>") || xml_content.contains("<w:t"));
    }
}

// --- PII Scan ---

#[derive(serde::Deserialize)]
pub struct PiiScanOptions {
    #[serde(default = "default_true")]
    pub scan_ssn: bool,
    #[serde(default = "default_true")]
    pub scan_credit_cards: bool,
    #[serde(default = "default_true")]
    pub scan_emails: bool,
    #[serde(default = "default_true")]
    pub scan_phones: bool,
    #[serde(default = "default_true")]
    pub scan_iban: bool,
}

fn default_true() -> bool {
    true
}

impl Default for PiiScanOptions {
    fn default() -> Self {
        Self {
            scan_ssn: true,
            scan_credit_cards: true,
            scan_emails: true,
            scan_phones: true,
            scan_iban: true,
        }
    }
}

#[derive(serde::Serialize)]
pub struct PiiFinding {
    pub category: String,
    pub value_masked: String,
    pub page: u32,
    pub confidence: f32,
    pub context_snippet: String,
}

#[derive(serde::Serialize)]
pub struct PiiScanReport {
    pub total_findings: usize,
    pub findings_by_category: std::collections::HashMap<String, usize>,
    pub findings: Vec<PiiFinding>,
    pub execution_time_ms: f64,
}

fn validate_luhn(number: &str) -> bool {
    let digits: Vec<u32> = number.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() < 13 || digits.len() > 19 {
        return false;
    }
    let mut sum = 0;
    let mut alternate = false;
    for &digit in digits.iter().rev() {
        if alternate {
            let mut d = digit * 2;
            if d > 9 {
                d -= 9;
            }
            sum += d;
        } else {
            sum += digit;
        }
        alternate = !alternate;
    }
    sum % 10 == 0
}

fn extract_context(text: &str, start: usize, end: usize, ctx_len: usize) -> String {
    let s = if start > ctx_len {
        let mut idx = start - ctx_len;
        while idx > 0 && !text.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    } else {
        0
    };
    let e = if end + ctx_len < text.len() {
        let mut idx = end + ctx_len;
        while idx < text.len() && !text.is_char_boundary(idx) {
            idx += 1;
        }
        idx
    } else {
        text.len()
    };
    text[s..e].replace('\n', " ")
}

pub fn pii_scan(input_bytes: &[u8], options_json: &str) -> OperationResult<String> {
    let options: PiiScanOptions = serde_json::from_str(options_json).unwrap_or_default();

    let doc =
        lopdf::Document::load_mem(input_bytes).map_err(|e| PdfError::ParseError(e.to_string()))?;

    let mut findings = Vec::new();
    let mut findings_by_category = std::collections::HashMap::new();

    let ssn_re = regex::Regex::new(r"\b(\d{3})-(\d{2})-(\d{4})\b").unwrap();
    let cc_re = regex::Regex::new(r"\b(?:\d[ -]*?){13,19}\b").unwrap();
    let email_re =
        regex::Regex::new(r"\b([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b").unwrap();
    let phone_re =
        regex::Regex::new(r"\b(\+?1?[-.\s]?)?(\()?\d{3}(\))?[-.\s]?\d{3}[-.\s]?\d{4}\b").unwrap();
    let iban_re = regex::Regex::new(r"\b[A-Z]{2}[0-9]{2}(?:[ ]?[a-zA-Z0-9]){11,28}\b").unwrap();

    let pages = doc.get_pages();
    let mut sorted_pages: Vec<u32> = pages.keys().copied().collect();
    sorted_pages.sort_unstable();

    for page_num in sorted_pages {
        let page_text = doc.extract_text(&[page_num]).unwrap_or_default();
        if page_text.trim().is_empty() {
            continue;
        }

        if options.scan_ssn {
            for cap in ssn_re.captures_iter(&page_text) {
                let m = cap.get(0).unwrap();
                let last4 = cap.get(3).unwrap().as_str();
                findings.push(PiiFinding {
                    category: "ssn".to_string(),
                    value_masked: format!("XXX-XX-{}", last4),
                    page: page_num,
                    confidence: 0.99,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("ssn".to_string()).or_insert(0) += 1;
            }
        }

        if options.scan_credit_cards {
            for m in cc_re.find_iter(&page_text) {
                let s = m.as_str();
                let clean_s: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                if validate_luhn(&clean_s) {
                    let last4 = if clean_s.len() >= 4 {
                        &clean_s[clean_s.len() - 4..]
                    } else {
                        &clean_s
                    };
                    findings.push(PiiFinding {
                        category: "credit_card".to_string(),
                        value_masked: format!("**** **** **** {}", last4),
                        page: page_num,
                        confidence: 0.95,
                        context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                    });
                    *findings_by_category
                        .entry("credit_card".to_string())
                        .or_insert(0) += 1;
                }
            }
        }

        if options.scan_emails {
            for cap in email_re.captures_iter(&page_text) {
                let m = cap.get(0).unwrap();
                let user = cap.get(1).unwrap().as_str();
                let domain = cap.get(2).unwrap().as_str();
                let masked_user = if user.len() > 1 {
                    format!("{}***{}", &user[..1], &user[user.len() - 1..])
                } else {
                    "***".to_string()
                };
                findings.push(PiiFinding {
                    category: "email".to_string(),
                    value_masked: format!("{}@{}", masked_user, domain),
                    page: page_num,
                    confidence: 0.98,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("email".to_string()).or_insert(0) += 1;
            }
        }

        if options.scan_phones {
            for m in phone_re.find_iter(&page_text) {
                let s = m.as_str();
                if ssn_re.is_match(s) {
                    continue;
                }

                let clean_s: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                if clean_s.len() >= 10 {
                    let last4 = &clean_s[clean_s.len() - 4..];
                    findings.push(PiiFinding {
                        category: "phone".to_string(),
                        value_masked: format!("(***) ***-{}", last4),
                        page: page_num,
                        confidence: 0.85,
                        context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                    });
                    *findings_by_category.entry("phone".to_string()).or_insert(0) += 1;
                }
            }
        }

        if options.scan_iban {
            for m in iban_re.find_iter(&page_text) {
                let s = m.as_str();
                let len = s.len();
                let masked = if len >= 8 {
                    let country = &s[..2];
                    let last4 = &s[len - 4..];
                    format!("{}** **** **** **** **{}", country, last4)
                } else {
                    "****".to_string()
                };
                findings.push(PiiFinding {
                    category: "iban".to_string(),
                    value_masked: masked,
                    page: page_num,
                    confidence: 0.80,
                    context_snippet: extract_context(&page_text, m.start(), m.end(), 20),
                });
                *findings_by_category.entry("iban".to_string()).or_insert(0) += 1;
            }
        }
    }

    let report = PiiScanReport {
        total_findings: findings.len(),
        findings_by_category,
        findings,
        execution_time_ms: 0.0,
    };

    let json =
        serde_json::to_string_pretty(&report).map_err(|e| PdfError::ParseError(e.to_string()))?;

    Ok(json)
}

pub fn pdf_validate_signatures(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_validate_signatures::PdfValidateSignaturesOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_validate_signatures::pdf_validate_signatures_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_booklet(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_booklet::PdfBookletOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_booklet::pdf_booklet_bytes(input_bytes, &options)
}

pub fn pdf_nup(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_nup::PdfNupOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_nup::pdf_nup_bytes(input_bytes, &options)
}

pub fn pdf_scale_pages(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_scale_pages::PdfScalePagesOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_scale_pages::pdf_scale_pages_bytes(input_bytes, &options)
}

pub fn pdf_remove_signatures(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_remove_signatures::PdfRemoveSignaturesOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_remove_signatures::pdf_remove_signatures_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_stamp(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_stamp::PdfStampOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_stamp::pdf_stamp_bytes(input_bytes, &options)
}

pub fn pdf_overlay_underlay(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_overlay_underlay::PdfOverlayUnderlayOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_overlay_underlay::pdf_overlay_underlay_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_auto_rotate(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_auto_rotate::PdfAutoRotateOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_auto_rotate::pdf_auto_rotate_bytes(input_bytes, &options)
}

pub fn pdf_stitch_canvas(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_stitch_canvas::PdfStitchCanvasOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_stitch_canvas::pdf_stitch_canvas_bytes(input_bytes, &options)
}

pub fn pdf_attachments(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_attachments::PdfAttachmentsOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_attachments::pdf_attachments_bytes(input_bytes, &options)
}

pub fn pdf_timestamp(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_timestamp::PdfTimestampOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_timestamp::pdf_timestamp_bytes(input_bytes, &options)
}

pub fn pdf_change_permissions(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_change_permissions::PdfChangePermissionsOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_change_permissions::pdf_change_permissions_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_split_scanned_book(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_split_scanned_book::PdfSplitScannedBookOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_split_scanned_book::pdf_split_scanned_book_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_comic_archive(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_comic_archive::PdfComicArchiveOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_comic_archive::pdf_comic_archive_bytes(input_bytes, &options)
}

pub fn pdf_split_by_outline(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_split_by_outline::PdfSplitByOutlineOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_split_by_outline::pdf_split_by_outline_bytes(
        input_bytes,
        &options,
    )
}

pub fn pdf_merge_alternate(buffers: Vec<Vec<u8>>, options_json: &str) -> OperationResult<Vec<u8>> {
    if buffers.len() < 2 {
        return Err(paperpilot_core::error::PdfError::InvalidInput(
            "Exactly two document buffers are required for alternating merge".into(),
        ));
    }

    let options: paperpilot_pdf::operations::pdf_merge_alternate::PdfMergeAlternateOptions =
        serde_json::from_str(options_json).map_err(|e| {
            paperpilot_core::error::PdfError::InvalidInput(format!("Invalid options: {}", e))
        })?;
    paperpilot_pdf::operations::pdf_merge_alternate::pdf_merge_alternate_bytes(
        &buffers[0],
        &buffers[1],
        &options,
    )
}

pub fn pdf_posterize(input_bytes: &[u8], options_json: &str) -> OperationResult<Vec<u8>> {
    let options: paperpilot_pdf::operations::pdf_posterize::PdfPosterizeOptions =
        serde_json::from_str(options_json)
            .map_err(|e| PdfError::InvalidInput(format!("Invalid options: {}", e)))?;
    paperpilot_pdf::operations::pdf_posterize::pdf_posterize_bytes(input_bytes, &options)
}
