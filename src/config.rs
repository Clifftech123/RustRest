use crate::error::{AppError, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

const APP_NAME: &str = "rustrest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Default headers sent with every request.
    pub default_headers: HashMap<String, String>,

    /// Optional base URL prefix (e.g. `http://localhost:3000`).
    pub base_url: Option<String>,

    /// Request timeout in seconds.
    pub timeout_secs: u64,

    /// Whether to follow redirects.
    pub follow_redirects: bool,

    /// Maximum number of history entries to retain.
    pub max_history: usize,

    /// Directory where collections are stored.
    pub collections_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_headers: HashMap::new(),
            base_url: None,
            timeout_secs: 30,
            follow_redirects: true,
            max_history: 100,
            collections_dir: Self::default_collections_dir(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).map_err(AppError::Json)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn config_dir() -> PathBuf {
        config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(APP_NAME)
    }

    pub fn history_path() -> PathBuf {
        Self::config_dir().join("history.json")
    }

    fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    fn default_collections_dir() -> PathBuf {
        Self::config_dir().join("collections")
    }
}
