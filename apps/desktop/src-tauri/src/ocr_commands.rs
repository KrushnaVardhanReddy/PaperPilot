use futures_util::StreamExt;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: String,
    pub size_mb: f64,
    pub download_urls: Vec<String>,
    pub local_path: Option<String>,
    pub is_downloaded: bool,
}

#[derive(Clone, Serialize)]
struct OcrDownloadProgress {
    model_id: String,
    percent: f64,
    bytes_downloaded: u64,
    total_bytes: u64,
}

lazy_static! {
    static ref OCR_DOWNLOAD_CANCEL_FLAGS: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn get_ocr_models_dir(tier_id: &str) -> std::path::PathBuf {
    if let Ok(override_dir) = std::env::var("PAPERPILOT_OCR_MODELS_DIR") {
        let mut path = std::path::PathBuf::from(override_dir);
        path.push(tier_id);
        return path;
    }

    // Try workspace fallback if we are in development
    let workspace_path = std::path::PathBuf::from(format!("models/ocr/{}", tier_id));
    if workspace_path.exists() {
        return workspace_path;
    }

    let mut path = crate::storage::get_app_data_root();
    path.push("models");
    path.push("ocr");
    path.push(tier_id);
    path
}

fn check_is_downloaded(tier_id: &str) -> Option<String> {
    let dir = get_ocr_models_dir(tier_id);
    if dir.exists() {
        let det_path = dir.join("det.onnx");
        let rec_path = dir.join("rec.onnx");
        let dict_path = dir.join("dict.txt");

        if det_path.exists() && rec_path.exists() && dict_path.exists() {
            // Verify files are real models (> 1 KB), not dummy mock stubs
            let det_size = std::fs::metadata(&det_path).map(|m| m.len()).unwrap_or(0);
            let rec_size = std::fs::metadata(&rec_path).map(|m| m.len()).unwrap_or(0);
            let dict_size = std::fs::metadata(&dict_path).map(|m| m.len()).unwrap_or(0);

            if det_size > 1024 && rec_size > 1024 && dict_size > 100 {
                return Some(dir.to_string_lossy().to_string());
            }
        }
    }
    None
}

#[derive(Debug, Deserialize)]
struct RegistryJson {
    #[serde(default)]
    ocr_models: Vec<RegistryOcrModel>,
}

#[derive(Debug, Deserialize)]
struct RegistryOcrModel {
    id: String,
    name: String,
    description: String,
    tier: String,
    size_mb: f64,
    download_urls: Vec<String>,
}

#[tauri::command]
pub async fn get_available_ocr_models() -> Result<Vec<OcrModelInfo>, String> {
    // 1. Try fetching latest OCR model registry from GitHub repository
    let registry_url = "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/registry.json";
    let client = Client::builder()
        .user_agent("PaperPilot-Desktop/1.0")
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_else(|_| Client::new());

    let mut loaded_models = None;

    if let Ok(resp) = client.get(registry_url).send().await {
        if resp.status().is_success() {
            if let Ok(text) = resp.text().await {
                if let Ok(parsed) = serde_json::from_str::<RegistryJson>(&text) {
                    if !parsed.ocr_models.is_empty() {
                        loaded_models = Some(
                            parsed
                                .ocr_models
                                .into_iter()
                                .map(|m| OcrModelInfo {
                                    id: m.id,
                                    name: m.name,
                                    description: m.description,
                                    tier: m.tier,
                                    size_mb: m.size_mb,
                                    download_urls: m.download_urls,
                                    local_path: None,
                                    is_downloaded: false,
                                })
                                .collect(),
                        );
                    }
                }
            }
        }
    }

    // 2. Fallback to local models/registry.json
    if loaded_models.is_none() {
        for candidate in [
            std::path::PathBuf::from("models/registry.json"),
            std::path::PathBuf::from("../../models/registry.json"),
        ] {
            if candidate.exists() {
                if let Ok(content) = std::fs::read_to_string(&candidate) {
                    if let Ok(parsed) = serde_json::from_str::<RegistryJson>(&content) {
                        if !parsed.ocr_models.is_empty() {
                            loaded_models = Some(
                                parsed
                                    .ocr_models
                                    .into_iter()
                                    .map(|m| OcrModelInfo {
                                        id: m.id,
                                        name: m.name,
                                        description: m.description,
                                        tier: m.tier,
                                        size_mb: m.size_mb,
                                        download_urls: m.download_urls,
                                        local_path: None,
                                        is_downloaded: false,
                                    })
                                    .collect(),
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    // 3. Fallback to hardcoded defaults
    let mut models = loaded_models.unwrap_or_else(|| {
        vec![
            OcrModelInfo {
                id: "ppocrv6-tiny".to_string(),
                name: "PP-OCRv6 Tiny (Default)".to_string(),
                description: "Fast web/desktop execution, lightweight footprint, 50+ languages unified dict".to_string(),
                tier: "tiny".to_string(),
                size_mb: 15.5,
                download_urls: vec![
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocrv4/det.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocrv4/rec.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocr_keys_v1.txt".to_string(),
                ],
                local_path: None,
                is_downloaded: false,
            },
            OcrModelInfo {
                id: "ppocrv6-small".to_string(),
                name: "PP-OCRv6 Small".to_string(),
                description: "Higher precision detector & recognizer. Recommended for legal contracts, fine print, complex layouts".to_string(),
                tier: "small".to_string(),
                size_mb: 25.0,
                download_urls: vec![
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocrv4/det.onnx".to_string(),
                    "https://huggingface.co/SWHL/RapidOCR/resolve/main/PP-OCRv4/ch_PP-OCRv4_rec_server_infer.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocr_keys_v1.txt".to_string(),
                ],
                local_path: None,
                is_downloaded: false,
            },
            OcrModelInfo {
                id: "ppocrv6-medium".to_string(),
                name: "PP-OCRv6 Medium".to_string(),
                description: "Server-grade accuracy. Extreme precision, low-contrast scans".to_string(),
                tier: "medium".to_string(),
                size_mb: 134.0,
                download_urls: vec![
                    "https://huggingface.co/SWHL/RapidOCR/resolve/main/PP-OCRv4/ch_PP-OCRv4_det_server_infer.onnx".to_string(),
                    "https://huggingface.co/SWHL/RapidOCR/resolve/main/PP-OCRv4/ch_PP-OCRv4_rec_server_infer.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocr_keys_v1.txt".to_string(),
                ],
                local_path: None,
                is_downloaded: false,
            },
            OcrModelInfo {
                id: "ppocrv4-mobile".to_string(),
                name: "PP-OCRv4 Mobile".to_string(),
                description: "Legacy RapidOCR baseline. Compatibility with older single-language pipelines".to_string(),
                tier: "mobile".to_string(),
                size_mb: 15.5,
                download_urls: vec![
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocrv4/det.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocrv4/rec.onnx".to_string(),
                    "https://raw.githubusercontent.com/KrushnaVardhanReddy/PaperPilot/main/models/ocr/ppocr_keys_v1.txt".to_string(),
                ],
                local_path: None,
                is_downloaded: false,
            },
        ]
    });

    for m in &mut models {
        if let Some(path) = check_is_downloaded(&m.id) {
            m.local_path = Some(path);
            m.is_downloaded = true;
        }
    }

    Ok(models)
}

#[tauri::command]
pub async fn download_ocr_model(app: AppHandle, model_id: String) -> Result<(), String> {
    let models = get_available_ocr_models().await?;
    let model = models
        .into_iter()
        .find(|m| m.id == model_id)
        .ok_or("Model not found")?;

    let models_dir = get_ocr_models_dir(&model.id);
    std::fs::create_dir_all(&models_dir).map_err(|e| e.to_string())?;

    {
        let mut flags = OCR_DOWNLOAD_CANCEL_FLAGS.lock().await;
        flags.insert(model_id.clone(), false);
    }

    let client = Client::builder()
        .user_agent("PaperPilot-Desktop/1.0")
        .build()
        .map_err(|e| e.to_string())?;
    let _num_files = model.download_urls.len();

    let estimated_total_bytes = (model.size_mb * 1024.0 * 1024.0) as u64;
    let mut total_downloaded: u64 = 0;

    for url in &model.download_urls {
        // Map target file name explicitly so det, rec, and dict have canonical names
        let file_name = if url.ends_with(".txt") {
            "dict.txt"
        } else if url.contains("det") {
            "det.onnx"
        } else if url.contains("rec") {
            "rec.onnx"
        } else {
            url.split('/').next_back().unwrap_or("file")
        };
        let mut dest_path = models_dir.clone();
        dest_path.push(file_name);

        let mut request = client.get(url);

        if dest_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&dest_path) {
                let downloaded_for_file = metadata.len();
                if downloaded_for_file > 1024 {
                    request = request.header("Range", format!("bytes={}-", downloaded_for_file));
                    total_downloaded += downloaded_for_file;
                }
            }
        }

        let response = request.send().await.map_err(|e| e.to_string())?;

        if !response.status().is_success()
            && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
        {
            return Err(format!(
                "Failed to download model file from {}: HTTP {}",
                url,
                response.status()
            ));
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&dest_path)
            .await
            .map_err(|e| e.to_string())?;

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e: reqwest::Error| e.to_string())?;

            let is_cancelled = {
                let flags = OCR_DOWNLOAD_CANCEL_FLAGS.lock().await;
                flags.get(&model_id).copied().unwrap_or(false)
            };

            if is_cancelled {
                return Err("Download cancelled".to_string());
            }

            file.write_all(&chunk).await.map_err(|e| e.to_string())?;
            total_downloaded += chunk.len() as u64;

            let percent = if estimated_total_bytes > 0 {
                (total_downloaded as f64 / estimated_total_bytes as f64) * 100.0
            } else {
                0.0
            };

            let _ = app.emit(
                "ocr-download-progress",
                OcrDownloadProgress {
                    model_id: model_id.clone(),
                    percent: percent.min(100.0),
                    bytes_downloaded: total_downloaded,
                    total_bytes: estimated_total_bytes,
                },
            );
        }
    }

    // Emit final 100% just in case
    let _ = app.emit(
        "ocr-download-progress",
        OcrDownloadProgress {
            model_id: model_id.clone(),
            percent: 100.0,
            bytes_downloaded: total_downloaded,
            total_bytes: total_downloaded.max(estimated_total_bytes),
        },
    );

    Ok(())
}

#[tauri::command]
pub async fn cancel_ocr_model_download(model_id: String) -> Result<(), String> {
    let mut flags = OCR_DOWNLOAD_CANCEL_FLAGS.lock().await;
    flags.insert(model_id, true);
    Ok(())
}

// Global active model setting
lazy_static! {
    static ref ACTIVE_OCR_MODEL: Arc<std::sync::Mutex<String>> =
        Arc::new(std::sync::Mutex::new("ppocrv6-tiny".to_string()));
}

#[tauri::command]
pub async fn set_active_ocr_model(model_id: String) -> Result<(), String> {
    if let Ok(mut lock) = ACTIVE_OCR_MODEL.lock() {
        *lock = model_id;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_active_ocr_model() -> Result<String, String> {
    if let Ok(lock) = ACTIVE_OCR_MODEL.lock() {
        return Ok(lock.clone());
    }
    Ok("ppocrv6-tiny".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_active_ocr_model() {
        let res = set_active_ocr_model("ppocrv6-medium".to_string()).await;
        assert!(res.is_ok());

        let res2 = get_active_ocr_model().await;
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), "ppocrv6-medium");
    }

    #[tokio::test]
    async fn test_get_available_ocr_models() {
        let models = get_available_ocr_models().await.unwrap();
        assert_eq!(models.len(), 4);
        assert_eq!(models[0].id, "ppocrv6-tiny");
        assert_eq!(models[1].id, "ppocrv6-small");
        assert_eq!(models[2].id, "ppocrv6-medium");
        assert_eq!(models[3].id, "ppocrv4-mobile");
    }
}
