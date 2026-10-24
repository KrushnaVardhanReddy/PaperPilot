use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;

const EMBEDDED_SAFETY_RULES: &str = include_str!("../SAFETY_RULES.md");

/// Retrieves safety rules from a local `tools/jules-submit/SAFETY_RULES.md` or falls back to compile-time embedded copy.
pub fn get_safety_rules() -> String {
    // Check local filesystem first so users can tweak rules per session without recompiling
    if let Ok(content) = std::fs::read_to_string("tools/jules-submit/SAFETY_RULES.md") {
        content
    } else if let Ok(content) = std::fs::read_to_string("SAFETY_RULES.md") {
        content
    } else {
        EMBEDDED_SAFETY_RULES.to_string()
    }
}

const API_URL: &str = "https://jules.googleapis.com/v1alpha/sessions";
const REPO_SOURCE: &str = "sources/github/KrushnaVardhanReddy/PaperPilot";

/// Formats the final payload for the API request.
pub fn build_payload(prompt_content: &str, branch: &str) -> serde_json::Value {
    let full_prompt = format!("{}\n\n---\n\n{}", get_safety_rules(), prompt_content);

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
pub fn submit_prompt(
    api_key: &str,
    prompt_content: &str,
    task_name: &str,
    branch: &str,
) -> Result<String> {
    let client = Client::new();
    let payload = build_payload(prompt_content, branch);

    println!("🚀 Submitting: {} → branch: {}", task_name, branch);

    let res = client
        .post(API_URL)
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

    let name = response_data
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    let session_id = name.split('/').next_back().unwrap_or(name);

    println!("✅ Session created: {}", session_id);
    println!(
        "   View at: https://jules.google.com/session/{}",
        session_id
    );

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

        let branch = payload["sourceContext"]["githubRepoContext"]["startingBranch"]
            .as_str()
            .unwrap();
        assert_eq!(branch, "feature-branch");
    }
}
