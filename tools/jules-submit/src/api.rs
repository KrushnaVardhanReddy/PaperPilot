use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;

const SAFETY_RULES: &str = r#"MANDATORY RULES — VIOLATION = REJECTED PR:
1. NEVER stub, mock, or TODO existing implementation code. Write real, working code only.
2. Commit message must start with "jules: " prefix.
3. Use clean module design and idiomatic Rust architecture. No spaghetti code.
4. NEVER hardcode secrets, API keys, or local paths — always read from environment variables.
5. UNIT TESTS REQUIRED: For every Rust file you create or modify, you MUST write accompanying unit tests (e.g. `#[cfg(test)]` modules) with high coverage.
6. When asking any questions, YOU MUST prefix your question with the Task Number and Title (e.g., "[P2_T1: MCP Foundation]").
7. WIKI & DOCS REQUIREMENT: For any newly introduced feature, module, engine, or architecture component, you MUST create or update the relevant documentation in the `wiki/` directory (e.g. `wiki/XX-Module-Name.md`) reflecting the new architecture, usage, and key components.
8. PERIODIC PROGRESS UPDATES: You MUST proactively send a concise progress update message to the session conversation every 10 to 15 minutes or after completing each logical milestone/step (e.g., "Step 2/4 completed: tests passed, now modifying X..."), so the team is never left without visibility into ongoing progress.

Project: PaperPilot
Tech stack:
  - Core: Rust (Cargo)
  - PDF libraries: lopdf, pdfium-render
  - CLI parser: clap
  - MCP: rmcp
  - Desktop: Tauri 2, Svelte 5, TypeScript, Vite

Critical Rules:
- Rust code must use typed errors (no string panics).
- Unit tests are required for all operations.
- Do NOT use Electron; use Tauri 2.
- The AI should not directly manipulate document bytes, only use MCP tool operations.
- Proactively send progress updates in chat every 10-15 minutes."#;

const API_URL: &str = "https://jules.googleapis.com/v1alpha/sessions";
const REPO_SOURCE: &str = "sources/github/KrushnaVardhanReddy/PaperPilot";

/// Formats the final payload for the API request.
pub fn build_payload(prompt_content: &str, branch: &str) -> serde_json::Value {
    let full_prompt = format!("{}\n\n---\n\n{}", SAFETY_RULES, prompt_content);

    json!({
        "prompt": full_prompt,
        "sourceContext": {
            "source": REPO_SOURCE,
            "githubRepoContext": {
                "startingBranch": branch
            }
        }
    })
}

/// Submits the prompt to Jules API and returns the session ID.
pub fn submit_prompt(api_key: &str, prompt_content: &str, task_name: &str, branch: &str) -> Result<String> {
    let client = Client::new();
    let payload = build_payload(prompt_content, branch);

    println!("🚀 Submitting: {} → branch: {}", task_name, branch);

    let res = client.post(API_URL)
        .header("Content-Type", "application/json")
        .header("x-goog-api-key", api_key)
        .json(&payload)
        .send()
        .context("Failed to send HTTP request")?;

    if !res.status().is_success() {
        let status = res.status();
        let error_body = res.text().unwrap_or_default();
        anyhow::bail!("❌ HTTP {}: {}", status, error_body);
    }

    let response_data: serde_json::Value = res.json().context("Failed to parse JSON response")?;

    let name = response_data.get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    let session_id = name.split('/').last().unwrap_or(name);

    println!("✅ Session created: {}", session_id);
    println!("   View at: https://jules.google.com/session/{}", session_id);

    Ok(session_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_construction() {
        let payload = build_payload("Test Prompt", "feature-branch");

        let prompt_str = payload["prompt"].as_str().unwrap();
        assert!(prompt_str.contains("Test Prompt"));
        assert!(prompt_str.contains("MANDATORY RULES — VIOLATION = REJECTED PR"));

        let branch = payload["sourceContext"]["githubRepoContext"]["startingBranch"].as_str().unwrap();
        assert_eq!(branch, "feature-branch");
    }
}
