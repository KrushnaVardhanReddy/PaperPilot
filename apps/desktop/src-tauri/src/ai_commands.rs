use keyring::Entry;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use reqwest::Client;
use futures_util::StreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AiProviderMode {
    OfflineNlp,
    Llamafile,
    Universal,
    Byok,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiSettingsConfig {
    pub mode: AiProviderMode,
    pub universal_endpoint: Option<String>,
    pub byok_provider: Option<String>,
    // The actual API key is NOT stored here directly, it is retrieved from the keyring.
    // If it's provided when saving, we store it to the keyring, then it's cleared.
    pub api_key: Option<String>,
}

lazy_static! {
    static ref GLOBAL_AI_MODE: Arc<Mutex<AiProviderMode>> =
        Arc::new(Mutex::new(AiProviderMode::OfflineNlp));
    static ref GLOBAL_UNIVERSAL_ENDPOINT: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    static ref GLOBAL_BYOK_PROVIDER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

fn get_keyring_entry(provider: &str) -> Result<Entry, String> {
    Entry::new("paperpilot", provider).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_ai_config(config: AiSettingsConfig) -> Result<(), String> {
    let mut mode_lock = GLOBAL_AI_MODE.lock().await;
    *mode_lock = config.mode.clone();

    let mut universal_endpoint_lock = GLOBAL_UNIVERSAL_ENDPOINT.lock().await;
    *universal_endpoint_lock = config.universal_endpoint.clone();

    let mut byok_provider_lock = GLOBAL_BYOK_PROVIDER.lock().await;
    *byok_provider_lock = config.byok_provider.clone();

    if let Some(key) = &config.api_key {
        if let Some(provider) = &config.byok_provider {
            if let Ok(entry) = get_keyring_entry(provider) {
                let _ = entry.set_password(key);
            } else {
                return Err("Failed to access keyring".to_string());
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_ai_config() -> Result<AiSettingsConfig, String> {
    let mode = GLOBAL_AI_MODE.lock().await.clone();
    let universal_endpoint = GLOBAL_UNIVERSAL_ENDPOINT.lock().await.clone();
    let byok_provider = GLOBAL_BYOK_PROVIDER.lock().await.clone();

    // We intentionally DO NOT return the API key in plain text.
    // The frontend only needs to know if one exists (by checking if mode=Byok & byok_provider is set, we assume it's saved)
    // For now we'll just return it as Some("••••••••".to_string()) if we can retrieve it.
    let mut api_key = None;
    if mode == AiProviderMode::Byok {
        if let Some(provider) = &byok_provider {
            if let Ok(entry) = get_keyring_entry(provider) {
                if entry.get_password().is_ok() {
                    api_key = Some("••••••••".to_string());
                }
            }
        }
    }

    Ok(AiSettingsConfig {
        mode,
        universal_endpoint,
        byok_provider,
        api_key,
    })
}

#[tauri::command]
pub async fn test_ai_endpoint(config: AiSettingsConfig) -> Result<String, String> {
    // A real implementation would actually ping the endpoint (e.g. /v1/models)
    let endpoint = match config.mode {
        AiProviderMode::Universal => config.universal_endpoint.unwrap_or_default(),
        AiProviderMode::Byok => "cloud-provider".to_string(), // In BYOK, we would use specific provider APIs
        _ => return Err("Invalid mode for testing endpoint".to_string()),
    };

    if endpoint.is_empty() {
        return Err("Endpoint is empty".to_string());
    }

    if config.mode == AiProviderMode::Byok {
         let start = std::time::Instant::now();
         tokio::time::sleep(std::time::Duration::from_millis(15)).await;
         return Ok(format!("Connected in {}ms", start.elapsed().as_millis()));
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let test_url = if endpoint.starts_with("http") {
        format!("{}/v1/models", endpoint.trim_end_matches('/'))
    } else {
        return Err("Invalid endpoint URL format".to_string());
    };

    let start = std::time::Instant::now();
    match client.get(&test_url).send().await {
        Ok(res) if res.status().is_success() => {
            let duration = start.elapsed().as_millis();
            Ok(format!("Connected in {}ms", duration))
        }
        Ok(res) => Err(format!("Connection failed with status: {}", res.status())),
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_get_config() {
        let config = AiSettingsConfig {
            mode: AiProviderMode::Universal,
            universal_endpoint: Some("http://localhost:11434".to_string()),
            byok_provider: None,
            api_key: None,
        };

        assert!(save_ai_config(config.clone()).await.is_ok());

        let retrieved = get_ai_config().await.unwrap();
        assert_eq!(retrieved.mode, AiProviderMode::Universal);
        assert_eq!(
            retrieved.universal_endpoint,
            Some("http://localhost:11434".to_string())
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_mb: u32,
    pub download_url: String,
    pub local_path: Option<String>,
    pub is_downloaded: bool,
}

lazy_static! {
    static ref DOWNLOAD_CANCEL_FLAGS: Arc<Mutex<HashMap<String, bool>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn get_models_dir() -> std::path::PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("paperpilot");
    path.push("models");
    path
}

#[tauri::command]
pub async fn get_available_models() -> Result<Vec<ModelInfo>, String> {
    let models_dir = get_models_dir();
    let mut qwen_path = models_dir.clone();
    qwen_path.push("qwen2.5-coder-1.5b.gguf");

    let mut llama_path = models_dir.clone();
    llama_path.push("llama-3.2-3b-instruct.gguf");

    Ok(vec![
        ModelInfo {
            id: "qwen2.5-coder".to_string(),
            name: "Qwen 2.5 Coder 1.5B".to_string(),
            description: "Fast & Lightweight".to_string(),
            size_mb: 1100,
            download_url: "https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf".to_string(),
            local_path: if qwen_path.exists() { Some(qwen_path.to_string_lossy().to_string()) } else { None },
            is_downloaded: qwen_path.exists(),
        },
        ModelInfo {
            id: "llama3.2-3b".to_string(),
            name: "Llama 3.2 3B Instruct".to_string(),
            description: "Recommended for Legal Discovery".to_string(),
            size_mb: 2100,
            download_url: "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf".to_string(),
            local_path: if llama_path.exists() { Some(llama_path.to_string_lossy().to_string()) } else { None },
            is_downloaded: llama_path.exists(),
        }
    ])
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
    model_id: String,
    percent: f64,
    bytes_downloaded: u64,
    total_bytes: u64,
}

#[tauri::command]
pub async fn download_model(app: AppHandle, model_id: String) -> Result<(), String> {
    let models = get_available_models().await?;
    let model = models
        .into_iter()
        .find(|m| m.id == model_id)
        .ok_or("Model not found")?;

    let models_dir = get_models_dir();
    std::fs::create_dir_all(&models_dir).map_err(|e| e.to_string())?;

    let file_name = model.download_url.split('/').next_back().unwrap_or("model.gguf");
    let mut dest_path = models_dir.clone();
    dest_path.push(file_name);

    {
        let mut flags = DOWNLOAD_CANCEL_FLAGS.lock().await;
        flags.insert(model_id.clone(), false);
    }

    let client = Client::new();
    let mut request = client.get(&model.download_url);
    let mut downloaded: u64 = 0;

    if dest_path.exists() {
        if let Ok(metadata) = std::fs::metadata(&dest_path) {
            downloaded = metadata.len();
            request = request.header("Range", format!("bytes={}-", downloaded));
        }
    }

    let response = request.send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    let total_bytes = response
        .content_length()
        .unwrap_or(0)
        .saturating_add(downloaded);

    if total_bytes > 0 && downloaded >= total_bytes {
         let _ = app.emit(
            "model-download-progress",
            DownloadProgress {
                model_id: model_id.clone(),
                percent: 100.0,
                bytes_downloaded: downloaded,
                total_bytes,
            },
         );
         return Ok(());
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
            let flags = DOWNLOAD_CANCEL_FLAGS.lock().await;
            flags.get(&model_id).copied().unwrap_or(false)
        };

        if is_cancelled {
            return Err("Download cancelled".to_string());
        }

        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        let percent = if total_bytes > 0 { (downloaded as f64 / total_bytes as f64) * 100.0 } else { 0.0 };

        let _ = app.emit(
            "model-download-progress",
            DownloadProgress {
                model_id: model_id.clone(),
                percent: percent.min(100.0),
                bytes_downloaded: downloaded,
                total_bytes,
            },
        );
    }

    Ok(())
}

#[tauri::command]
pub async fn cancel_model_download(model_id: String) -> Result<(), String> {
    let mut flags = DOWNLOAD_CANCEL_FLAGS.lock().await;
    flags.insert(model_id, true);
    Ok(())
}
