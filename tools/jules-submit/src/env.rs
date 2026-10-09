use anyhow::{Context, Result};
use std::env;

/// Attempts to load the `JULES_API_KEY` from the environment.
/// If it is not set, tries to load `.env.local` or `.env` from the repo root
/// before checking the environment again.
pub fn get_jules_api_key() -> Result<String> {
    if let Ok(key) = env::var("JULES_API_KEY") {
        if !key.trim().is_empty() {
            return Ok(key);
        }
    }

    // Try finding `.env.local` or `.env` in repository root
    // Assuming the tool is run from repo root or inside tools/jules-submit
    let current_dir = env::current_dir().context("Failed to get current directory")?;

    // Simple heuristic: look for `Cargo.toml` with `[workspace]` or `apps` dir.
    // Let's just traverse up to find repo root.
    let mut repo_root = current_dir.clone();
    loop {
        if repo_root.join("prompts").exists() && repo_root.join("scripts").exists() {
            break;
        }
        if !repo_root.pop() {
            repo_root = current_dir; // fallback
            break;
        }
    }

    let env_local = repo_root.join(".env.local");
    let env_file = repo_root.join(".env");

    if env_local.exists() {
        let _ = dotenvy::from_filename(&env_local);
    } else if env_file.exists() {
        let _ = dotenvy::from_filename(&env_file);
    } else {
        // Fallback to default dotenv behavior
        let _ = dotenvy::dotenv();
    }

    env::var("JULES_API_KEY").context("JULES_API_KEY is not set in environment or .env files")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_env_loading() {
        // We just test that the function doesn't panic.
        // It might return an error if JULES_API_KEY is not set in the CI environment,
        // which is expected. We just want to ensure the logic runs.
        let original = env::var("JULES_API_KEY");
        env::set_var("JULES_API_KEY", "test_key_123");
        let result = get_jules_api_key();
        assert_eq!(result.unwrap(), "test_key_123");

        if let Ok(val) = original {
            env::set_var("JULES_API_KEY", val);
        } else {
            env::remove_var("JULES_API_KEY");
        }
    }
}
