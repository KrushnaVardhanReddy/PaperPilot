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
