use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesktopError {
    LockError(String),
    IoError(String),
    NetworkError(String),
    Unknown,
}

impl std::fmt::Display for DesktopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesktopError::LockError(msg) => write!(f, "Lock error: {}", msg),
            DesktopError::IoError(msg) => write!(f, "IO error: {}", msg),
            DesktopError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DesktopError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for DesktopError {}
