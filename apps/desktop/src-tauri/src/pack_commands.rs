use crate::error::DesktopError;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeaturePackInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_mb: f64,
    pub is_installed: bool,
    pub is_enabled: bool,
    pub tools: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct PackDownloadProgress {
    pub pack_id: String,
    pub percent: f64,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
}

#[tauri::command]
pub fn is_portable_mode_cmd() -> Result<bool, DesktopError> {
    Ok(crate::storage::is_portable_mode())
}

fn get_packs_config_path() -> std::path::PathBuf {
    let mut path = crate::storage::get_app_data_root();
    fs::create_dir_all(&path).unwrap_or_default();
    path.push("packs.json");
    path
}

fn load_packs_state() -> HashMap<String, bool> {
    let path = get_packs_config_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(state) = serde_json::from_str::<HashMap<String, bool>>(&content) {
            return state;
        }
    }
    let mut default_state = HashMap::new();
    default_state.insert("core".to_string(), true);
    default_state
}

fn check_pack_downloaded(pack_id: &str) -> bool {
    if pack_id == "core" {
        return true;
    }
    let mut path = crate::storage::get_app_data_root();
    path.push(format!("{}.pack", pack_id));
    path.exists()
}

#[tauri::command]
pub fn get_feature_packs() -> Result<Vec<FeaturePackInfo>, DesktopError> {
    let state = load_packs_state();

    let mut packs = vec![
        FeaturePackInfo {
            id: "core".to_string(),
            name: "Core PDF Manipulation".to_string(),
            description: "Merge, Split, Rotate, Crop, Encrypt, Decrypt, Watermark, Compress, Info, Annotate".to_string(),
            size_mb: 0.0,
            is_installed: true, // Core is always installed
            is_enabled: true,   // Core is always enabled
            tools: vec![
                "Merge".to_string(), "Split".to_string(), "Rotate".to_string(), "Crop".to_string(),
                "Encrypt".to_string(), "Decrypt".to_string(), "Watermark".to_string(), "Compress".to_string(),
                "Info".to_string(), "Annotate".to_string()
            ],
        },
        FeaturePackInfo {
            id: "office".to_string(),
            name: "Office & Spreadsheets".to_string(),
            description: "office_to_markdown, office_extract_text, excel_to_json, excel_to_csv, DOCX/PPTX modernization".to_string(),
            size_mb: 12.0,
            is_installed: false,
            is_enabled: false,
            tools: vec![
                "office_to_markdown".to_string(), "office_extract_text".to_string(),
                "excel_to_json".to_string(), "excel_to_csv".to_string()
            ],
        },
        FeaturePackInfo {
            id: "ocr".to_string(),
            name: "Vision & Searchable OCR".to_string(),
            description: "PaddleOCR neural detector & recognizer (rusto-rs models), PDF text layer injection".to_string(),
            size_mb: 35.0,
            is_installed: false,
            is_enabled: false,
            tools: vec!["OCR Text Extraction".to_string(), "Searchable PDF Generation".to_string()],
        },
        FeaturePackInfo {
            id: "web".to_string(),
            name: "Web & URL Capture".to_string(),
            description: "Chromium-free pure-Rust web page archival & URL-to-PDF engine (browser_oxide)".to_string(),
            size_mb: 22.0,
            is_installed: false,
            is_enabled: false,
            tools: vec!["URL to PDF".to_string(), "Web Archival".to_string()],
        },
        FeaturePackInfo {
            id: "ai".to_string(),
            name: "Local AI Discovery Agent".to_string(),
            description: "Offline intent classification, RAG legal discovery assistant, local SLM".to_string(),
            size_mb: 1500.0,
            is_installed: false,
            is_enabled: false,
            tools: vec!["Intent Classification".to_string(), "RAG Assistant".to_string(), "Local SLM".to_string()],
        },
    ];

    for pack in &mut packs {
        pack.is_installed = check_pack_downloaded(&pack.id);
        pack.is_enabled = *state.get(&pack.id).unwrap_or(&false);
        if pack.id == "core" {
            pack.is_installed = true;
            pack.is_enabled = true;
        }
    }

    Ok(packs)
}

fn is_valid_pack_id(pack_id: &str) -> bool {
    !pack_id.is_empty()
        && pack_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[tauri::command]
pub async fn download_feature_pack(
    app: tauri::AppHandle,
    pack_id: String,
) -> Result<(), DesktopError> {
    if !is_valid_pack_id(&pack_id) {
        return Err(DesktopError::InvalidInput("Invalid pack ID".to_string()));
    }

    if pack_id == "core" {
        return Ok(());
    }

    let mut dest_path = crate::storage::get_app_data_root();
    tokio::fs::create_dir_all(&dest_path)
        .await
        .map_err(|e| DesktopError::IoError(e.to_string()))?;
    dest_path.push(format!("{}.pack", pack_id));

    let url = format!(
        "https://huggingface.co/paperpilot/packs/resolve/main/{}.pack",
        pack_id
    );
    let client = Client::new();
    let mut request = client.get(&url);

    let mut total_downloaded: u64 = 0;
    if dest_path.exists() {
        if let Ok(metadata) = tokio::fs::metadata(&dest_path).await {
            let downloaded_for_file = metadata.len();
            request = request.header("Range", format!("bytes={}-", downloaded_for_file));
            total_downloaded += downloaded_for_file;
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| DesktopError::NetworkError(e.to_string()))?;

    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
    {
        return Err(DesktopError::NetworkError(format!(
            "Failed to download feature pack: HTTP {}",
            response.status()
        )));
    }

    let total_size = response.content_length().unwrap_or(0) + total_downloaded;

    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&dest_path)
        .await
        .map_err(|e| DesktopError::IoError(e.to_string()))?;

    let mut stream = response.bytes_stream();
    use tokio::io::AsyncWriteExt;

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| DesktopError::NetworkError(e.to_string()))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| DesktopError::IoError(e.to_string()))?;
        total_downloaded += chunk.len() as u64;

        let percent = if total_size > 0 {
            (total_downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };
        let _ = app.emit(
            "pack-download-progress",
            PackDownloadProgress {
                pack_id: pack_id.clone(),
                percent: percent.min(100.0),
                bytes_downloaded: total_downloaded,
                total_bytes: total_size,
            },
        );
    }

    let _ = app.emit(
        "pack-download-progress",
        PackDownloadProgress {
            pack_id: pack_id.clone(),
            percent: 100.0,
            bytes_downloaded: total_downloaded,
            total_bytes: total_downloaded.max(total_size),
        },
    );

    set_feature_pack_enabled(pack_id, true)?;

    Ok(())
}

#[tauri::command]
pub fn set_feature_pack_enabled(pack_id: String, enabled: bool) -> Result<(), DesktopError> {
    if !is_valid_pack_id(&pack_id) {
        return Err(DesktopError::InvalidInput("Invalid pack ID".to_string()));
    }

    let mut state = load_packs_state();
    state.insert(pack_id, enabled);

    // Core is always true
    state.insert("core".to_string(), true);

    let path = get_packs_config_path();
    let content = serde_json::to_string_pretty(&state).map_err(|_| DesktopError::Unknown)?;
    fs::write(path, content).map_err(|e| DesktopError::IoError(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_portable_mode_cmd() {
        // Just verify it doesn't panic
        let _ = is_portable_mode_cmd();
    }

    #[test]
    fn test_get_feature_packs() {
        let packs = get_feature_packs().expect("Failed to get packs");
        assert_eq!(packs.len(), 5);
        let core = packs.iter().find(|p| p.id == "core").unwrap();
        assert!(core.is_installed);
        assert!(core.is_enabled);
    }

    #[test]
    fn test_set_feature_pack_enabled_isolated() {
        let _guard = crate::storage::tests::ENV_LOCK.lock().unwrap();
        // Using a safe isolated environment variable for tests
        let safe_data_dir =
            std::env::temp_dir().join(format!("safe_test_data_{}", std::process::id()));
        std::fs::create_dir_all(&safe_data_dir).unwrap();
        std::env::set_var("PAPERPILOT_DATA_DIR", safe_data_dir.to_str().unwrap());

        let res = set_feature_pack_enabled("test_pack".to_string(), true);
        assert!(res.is_ok());

        let state = load_packs_state();
        assert_eq!(state.get("test_pack"), Some(&true));

        std::env::remove_var("PAPERPILOT_DATA_DIR");
        let _ = std::fs::remove_dir_all(&safe_data_dir);
    }
}
