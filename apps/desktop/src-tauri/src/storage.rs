use std::path::PathBuf;

pub fn get_app_data_root() -> PathBuf {
    // 1. Portable Mode: Check for relative ./data/ next to executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let portable_dir = exe_dir.join("data");
            if portable_dir.exists() && portable_dir.is_dir() {
                return portable_dir;
            }
        }
    }

    // 2. Environment Variable Override
    if let Ok(env_override) = std::env::var("PAPERPILOT_DATA_DIR") {
        return PathBuf::from(env_override);
    }

    // 3. Standard Local System App Data Fallback
    dirs::data_local_dir()
        .map(|d| d.join("paperpilot"))
        .unwrap_or_else(|| PathBuf::from(".data"))
}

pub fn is_portable_mode() -> bool {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let portable_dir = exe_dir.join("data");
            if portable_dir.exists() && portable_dir.is_dir() {
                return true;
            }
        }
    }
    false
}

lazy_static::lazy_static! {
    static ref GLOBAL_STORAGE: std::sync::Arc<tokio::sync::Mutex<Option<paperpilot_storage::StorageEngine>>> =
        std::sync::Arc::new(tokio::sync::Mutex::new(None));
}

pub async fn get_storage_engine() -> Result<paperpilot_storage::StorageEngine, String> {
    let mut lock = GLOBAL_STORAGE.lock().await;
    if let Some(ref engine) = *lock {
        return Ok(engine.clone());
    }

    let root = get_app_data_root();
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let db_path = root.join("paperpilot.db");

    let engine = paperpilot_storage::StorageEngine::open_local(&db_path)
        .await
        .map_err(|e| format!("Failed to open libSQL storage: {}", e))?;

    engine
        .run_migrations()
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    *lock = Some(engine.clone());
    Ok(engine)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex to ensure environment variables are not tested concurrently
    lazy_static::lazy_static! {
        pub(crate) static ref ENV_LOCK: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_is_portable_mode() {
        // Since we can't easily fake current_exe in a simple test without
        // affecting the test runner's environment globally, we just ensure it doesn't panic.
        let _ = is_portable_mode();
    }

    #[test]
    fn test_get_app_data_root_env_override() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var("PAPERPILOT_DATA_DIR", "/tmp/paperpilot_test_data");

        let path = get_app_data_root();

        // Only assert if it's not overridden by portable mode (which takes precedence)
        if !is_portable_mode() {
            assert_eq!(path, PathBuf::from("/tmp/paperpilot_test_data"));
        }

        std::env::remove_var("PAPERPILOT_DATA_DIR");
    }

    #[tokio::test]
    async fn test_get_storage_engine() {
        let engine = get_storage_engine().await;
        assert!(engine.is_ok());
    }
}
