use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Cli,
    Mcp,
    Api,
    Wasm,
    CloudflareEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityTier {
    Simple,
    Medium,
    Complex,
    Negative,
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
