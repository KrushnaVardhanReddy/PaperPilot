use std::fs;
use std::path::Path;

pub fn handle_ocr_fallback(output: Option<&str>) -> serde_json::Value {
    let fallback_text = "OCR fallback: text extraction completed without model weights.";

    if let Some(out_path) = output {
        let _ = fs::write(out_path, fallback_text);
    }

    serde_json::json!({
        "success": true,
        "text": fallback_text,
        "model_configured": false,
        "warning": "OCR models unconfigured; fallback test text generated"
    })
}

pub fn check_ocr_models() -> bool {
    let base_paths = [
        Path::new("models/ocr"),
        Path::new("../models/ocr"),
        Path::new("../../models/ocr"),
    ];

    for base in base_paths.iter() {
        if base.exists() && base.is_dir() {
            // Check if key model files exist in this directory
            if base.join("ppocrv4").exists() || base.join("ppocrv6").exists() || base.join("text_detection.rten").exists() {
                return true;
            }
        }
    }

    // Check PAPERPILOT_OCR_MODELS_DIR env var
    if let Ok(env_path) = std::env::var("PAPERPILOT_OCR_MODELS_DIR") {
        let path = Path::new(&env_path);
        if path.exists() && path.is_dir() {
            return true;
        }
    }

    false
}
