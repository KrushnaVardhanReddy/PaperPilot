use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesktopError {
    LockError(String),
}

impl std::fmt::Display for DesktopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesktopError::LockError(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl std::error::Error for DesktopError {}
