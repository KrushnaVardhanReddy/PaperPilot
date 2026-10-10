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

pub fn watermark(input_bytes: &[u8], text: &str, angle: Option<f32>, opacity: Option<f32>) -> OperationResult<Vec<u8>> {
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
            if let Ok(media_box) = page_dict.get(b"MediaBox") {
                if let Ok(arr) = media_box.as_array() {
                    if arr.len() == 4 {
                        if let (Ok(x2), Ok(y2)) = (arr[2].as_f32().or_else(|_| arr[2].as_i64().map(|v| v as f32)), arr[3].as_f32().or_else(|_| arr[3].as_i64().map(|v| v as f32))) {
                            page_width = x2;
                            page_height = y2;
                        }
                    }
                }
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

        let escaped_text = text.replace("(", "\(").replace(")", "\)");

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
        let r = 0.5; let g = 0.5; let b = 0.5;

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
