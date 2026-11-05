use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Locates the `agy` or `gemini` CLI executable on the system.
pub fn find_local_agent_bin() -> Result<String> {
    // 1. Check AGY_BIN env var
    if let Ok(bin) = std::env::var("AGY_BIN") {
        if Path::new(&bin).exists() {
            return Ok(bin);
        }
    }

    // 2. Check standard Antigravity / Gemini paths
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/krushna".to_string());
    let candidate_paths = [
        format!("{}/.gemini/bin/agy", home),
        format!("{}/.local/bin/agy", home),
        "/usr/local/bin/agy".to_string(),
        "/usr/bin/agy".to_string(),
    ];

    for path in &candidate_paths {
        if Path::new(path).exists() {
            return Ok(path.clone());
        }
    }

    // 3. Fallback to which agy or gemini
    if let Ok(output) = Command::new("which").arg("agy").output() {
        if output.status.success() {
            let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !found.is_empty() {
                return Ok(found);
            }
        }
    }

    anyhow::bail!(
        "Local Gemini/Antigravity CLI ('agy') not found. Please install it or set AGY_BIN."
    )
}

/// Executes a prompt using the local `agy` (Gemini CLI) pair programmer.
pub fn run_local_agent(
    prompt_content: &str,
    task_name: &str,
    interactive: bool,
    dry_run: bool,
) -> Result<()> {
    let agy_bin = find_local_agent_bin()?;

    println!("🤖 [Local Pair Programmer] Target agent binary: {}", agy_bin);
    println!("📋 Task: {}", task_name);

    if dry_run {
        println!("[⏸️  DRY RUN] Would execute local Gemini CLI with prompt:\n{}", prompt_content);
        return Ok(());
    }

    let mut cmd = Command::new(&agy_bin);

    if interactive {
        println!("🚀 Launching interactive pair programming session with Gemini CLI...");
        cmd.arg("-i").arg(prompt_content);
    } else {
        println!("🚀 Executing non-interactive autonomous pair programming turn...");
        cmd.arg("-p")
            .arg(prompt_content)
            .arg("--dangerously-skip-permissions");
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to spawn local agent: {}", agy_bin))?;

    if !status.success() {
        anyhow::bail!("Local agent exited with error code: {:?}", status.code());
    }

    println!("✅ Local agent execution completed successfully.");
    Ok(())
}
