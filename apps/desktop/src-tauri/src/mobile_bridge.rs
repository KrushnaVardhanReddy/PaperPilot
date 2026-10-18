use base64::{engine::general_purpose::STANDARD, Engine as _};
use paperpilot_core::traits::{PdfDocument, PdfOperation};
use paperpilot_pdf::document::LopdfDocument;
use paperpilot_pdf::operations::{
    compress::CompressOperation, metadata::MetadataOperation, split::SplitOperation,
};
use serde_json::json;
use std::fs;
use uuid::Uuid;

#[tauri::command]
pub async fn process_mobile_buffer(
    file_name: String,
    input_bytes: Vec<u8>,
    operation: String,
    params: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let temp_dir = std::env::temp_dir();
    let sandbox_dir = temp_dir.join(format!("paperpilot_mobile_{}", Uuid::new_v4()));

    fs::create_dir_all(&sandbox_dir).map_err(|e| format!("Failed to create sandbox: {}", e))?;

    let safe_file_name = std::path::Path::new(&file_name)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "input.pdf".to_string());

    let input_path = sandbox_dir.join(&safe_file_name);
    fs::write(&input_path, &input_bytes)
        .map_err(|e| format!("Failed to write input bytes: {}", e))?;

    let output_file_name = format!("out_{}", safe_file_name);
    let output_path = sandbox_dir.join(&output_file_name);

    let mut doc = LopdfDocument::load(&input_path)
        .map_err(|e| format!("Failed to load PDF document: {:?}", e))?;

    // Dispatch operation
    match operation.as_str() {
        "merge" | "pdf_merge" => {
            // Check if multiple documents were passed in params.documents
            let mut merge_paths = Vec::new();
            if let Some(docs) = params.get("documents").and_then(|v| v.as_array()) {
                for (idx, item) in docs.iter().enumerate() {
                    let doc_name = item.get("name").and_then(|v| v.as_str()).unwrap_or("doc.pdf");
                    let safe_name = format!("{}_{}", idx, doc_name);
                    let doc_path = sandbox_dir.join(safe_name);
                    if let Some(b64) = item.get("dataBase64").and_then(|v| v.as_str()) {
                        let decoded = STANDARD
                            .decode(b64)
                            .map_err(|e| format!("Failed to decode document base64: {}", e))?;
                        fs::write(&doc_path, &decoded)
                            .map_err(|e| format!("Failed to write merge doc: {}", e))?;
                        merge_paths.push(doc_path);
                    }
                }
            } else if let Some(inputs) = params.get("inputs").and_then(|v| v.as_array()) {
                for item in inputs {
                    if let Some(p) = item.as_str() {
                        let pb = std::path::PathBuf::from(p);
                        if pb.exists() {
                            merge_paths.push(pb);
                        }
                    }
                }
            }

            if merge_paths.is_empty() {
                return Err("No documents provided for merge operation".to_string());
            }

            // LopdfDocument::load the first document, and merge the rest into it
            let first_path = merge_paths.remove(0);
            let mut merged_doc = LopdfDocument::load(&first_path)
                .map_err(|e| format!("Failed to load base PDF for merge: {:?}", e))?;

            if !merge_paths.is_empty() {
                let op = paperpilot_pdf::operations::merge::MergeOperation::new(merge_paths);
                op.execute(&mut merged_doc)
                    .map_err(|e| format!("Merge failed: {:?}", e))?;
            }

            merged_doc
                .save(&output_path)
                .map_err(|e| format!("Failed to save merged PDF: {:?}", e))?;
        }
        "compress" => {
            let quality = params
                .get("quality")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let op = CompressOperation::new(quality);
            op.execute(&mut doc)
                .map_err(|e| format!("Compression failed: {:?}", e))?;
            doc.save(&output_path)
                .map_err(|e| format!("Failed to save output: {:?}", e))?;
        }
        "metadata" => {
            let mut op = MetadataOperation::new();
            if let Some(t) = params.get("title").and_then(|v| v.as_str()) {
                op.title = Some(t.to_string());
            }
            if let Some(a) = params.get("author").and_then(|v| v.as_str()) {
                op.author = Some(a.to_string());
            }
            if let Some(s) = params.get("subject").and_then(|v| v.as_str()) {
                op.subject = Some(s.to_string());
            }
            if let Some(k) = params.get("keywords").and_then(|v| v.as_str()) {
                op.keywords = Some(k.to_string());
            }
            op.execute(&mut doc)
                .map_err(|e| format!("Metadata failed: {:?}", e))?;
            doc.save(&output_path)
                .map_err(|e| format!("Failed to save output: {:?}", e))?;
        }
        "split" => {
            let split_points = params
                .get("split_points")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_u64().map(|n| n as u32))
                        .collect::<Vec<u32>>()
                })
                .unwrap_or_default();

            let op = SplitOperation::new(sandbox_dir.clone(), split_points);
            op.execute(&mut doc)
                .map_err(|e| format!("Split failed: {:?}", e))?;

            // For split, return the first split file as the primary output for mobile bridge parity
            let first_split = sandbox_dir.join("split_1.pdf");
            if first_split.exists() {
                fs::rename(&first_split, &output_path)
                    .map_err(|e| format!("Failed to rename split output: {}", e))?;
            } else {
                return Err("Split operation produced no output".to_string());
            }
        }
        "rotate" | "pdf_rotate" => {
            let degrees = params
                .get("angle")
                .or_else(|| params.get("degrees"))
                .and_then(|v| v.as_u64())
                .unwrap_or(90) as u16;

            let pages = params.get("pages").and_then(|v| {
                if let Some(s) = v.as_str() {
                    Some(
                        s.split(',')
                            .filter_map(|p| p.trim().parse::<u32>().ok())
                            .collect::<Vec<u32>>(),
                    )
                } else if let Some(arr) = v.as_array() {
                    Some(
                        arr.iter()
                            .filter_map(|p| p.as_u64().map(|n| n as u32))
                            .collect::<Vec<u32>>(),
                    )
                } else {
                    None
                }
            });

            let op = paperpilot_pdf::operations::rotate::RotatePagesOperation::new(degrees, pages);
            op.execute(&mut doc)
                .map_err(|e| format!("Rotate failed: {:?}", e))?;
            doc.save(&output_path)
                .map_err(|e| format!("Failed to save output: {:?}", e))?;
        }
        "watermark" | "pdf_watermark" => {
            let text = params
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("CONFIDENTIAL")
                .to_string();
            let mut op = paperpilot_pdf::operations::watermark::WatermarkOperation::new(text);
            if let Some(angle) = params.get("angle").and_then(|v| v.as_f64()) {
                op.angle = angle as f32;
            }
            if let Some(opacity) = params.get("opacity").and_then(|v| v.as_f64()) {
                op.opacity = opacity as f32;
            }
            op.execute(&mut doc)
                .map_err(|e| format!("Watermark failed: {:?}", e))?;
            doc.save(&output_path)
                .map_err(|e| format!("Failed to save output: {:?}", e))?;
        }
        "images_to_pdf" => {
            // Drop loaded doc since we create a fresh PDF from the images
            drop(doc);

            let images_b64 = params
                .get("images")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "images_to_pdf requires 'images' array".to_string())?;

            let mut image_paths = Vec::new();
            for (idx, img_val) in images_b64.iter().enumerate() {
                let b64_str = img_val
                    .as_str()
                    .ok_or_else(|| "Image item must be a base64 string".to_string())?;
                let img_bytes = STANDARD
                    .decode(b64_str)
                    .map_err(|e| format!("Failed to decode base64 image: {}", e))?;
                let img_path = sandbox_dir.join(format!("page_{}.jpg", idx));
                fs::write(&img_path, &img_bytes)
                    .map_err(|e| format!("Failed to write page image: {}", e))?;
                image_paths.push(img_path);
            }

            let mut new_doc = LopdfDocument::new();
            let op =
                paperpilot_pdf::operations::images_to_pdf::ImagesToPdfOperation::new(image_paths);
            op.execute(&mut new_doc)
                .map_err(|e| format!("ImagesToPdf failed: {:?}", e))?;
            new_doc
                .save(&output_path)
                .map_err(|e| format!("Failed to save generated PDF: {:?}", e))?;
        }
        _ => return Err(format!("Unknown operation: {}", operation)),
    }

    let output_bytes =
        fs::read(&output_path).map_err(|e| format!("Failed to read output file: {}", e))?;
    let size_bytes = output_bytes.len();
    let data_base64 = STANDARD.encode(&output_bytes);

    // Clean up
    let _ = fs::remove_dir_all(&sandbox_dir);

    Ok(json!({
        "success": true,
        "fileName": output_file_name,
        "dataBase64": data_base64,
        "sizeBytes": size_bytes
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use paperpilot_pdf::document::LopdfDocument;

    #[tokio::test]
    async fn test_process_mobile_buffer_compress() {
        // Create a dummy pdf in memory
        let temp_dir = std::env::temp_dir();
        let dummy_path = temp_dir.join(format!("dummy_{}.pdf", Uuid::new_v4()));

        let mut doc = LopdfDocument::new();
        // just add a dummy page so it's a valid pdf
        let pages_id = doc.inner.new_object_id();
        doc.inner.objects.insert(
            pages_id,
            lopdf::Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                (b"Type".to_vec(), lopdf::Object::Name(b"Pages".to_vec())),
                (b"Count".to_vec(), lopdf::Object::Integer(0)),
                (b"Kids".to_vec(), lopdf::Object::Array(vec![])),
            ])),
        );
        let root_id = doc.inner.new_object_id();
        doc.inner.objects.insert(
            root_id,
            lopdf::Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                (b"Type".to_vec(), lopdf::Object::Name(b"Catalog".to_vec())),
                (b"Pages".to_vec(), lopdf::Object::Reference(pages_id)),
            ])),
        );
        doc.inner
            .trailer
            .set("Root", lopdf::Object::Reference(root_id));

        doc.save(&dummy_path).unwrap();
        let input_bytes = fs::read(&dummy_path).unwrap();
        fs::remove_file(&dummy_path).unwrap();

        let result = process_mobile_buffer(
            "test.pdf".to_string(),
            input_bytes,
            "compress".to_string(),
            json!({"quality": "low"}),
        )
        .await;

        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["success"], true);
        assert_eq!(val["fileName"], "out_test.pdf");
        assert!(val["dataBase64"].is_string());
        assert!(val["sizeBytes"].is_number());
    }
}
