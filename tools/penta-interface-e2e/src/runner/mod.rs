pub mod api;
pub mod cli;
pub mod gateway;
pub mod mcp;

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Cli,
    Mcp,
    Api,
    Wasm,
    CloudflareEdge,
}

impl InterfaceType {
    pub fn label(&self) -> &'static str {
        match self {
            InterfaceType::Cli => "💻 CLI",
            InterfaceType::Mcp => "🤖 MCP",
            InterfaceType::Api => "🌐 REST API",
            InterfaceType::Wasm => "⚡ WASM (Browser)",
            InterfaceType::CloudflareEdge => "☁️ Cloudflare Edge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityTier {
    Simple,
    Medium,
    Complex,
    Negative,
}

impl ComplexityTier {
    pub fn label(&self) -> &'static str {
        match self {
            ComplexityTier::Simple => "Simple (Tier 1)",
            ComplexityTier::Medium => "Medium (Tier 2)",
            ComplexityTier::Complex => "Complex (Tier 3)",
            ComplexityTier::Negative => "Negative",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestExecutionResult {
    pub tool_id: String,
    pub interface: InterfaceType,
    pub tier: ComplexityTier,
    pub data_sent: String,
    pub assertion_checked: String,
    pub expected_result: String,
    pub actual_result: String,
    pub passed: bool,
    pub latency_ms: f64,
}

pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn get_cli_bin() -> PathBuf {
    let release = PathBuf::from("target/release/paperpilot-cli");
    if release.exists() {
        release
    } else {
        PathBuf::from("target/debug/paperpilot-cli")
    }
}

pub fn get_mcp_bin() -> PathBuf {
    let release = PathBuf::from("target/release/paperpilot-mcp");
    if release.exists() {
        release
    } else {
        PathBuf::from("target/debug/paperpilot-mcp")
    }
}
