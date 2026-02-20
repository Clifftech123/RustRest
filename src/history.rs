use crate::config::AppConfig;
use crate::display;
use crate::error::Result;
use crate::request::HttpRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id:         u64,
    pub timestamp:  DateTime<Utc>,
    pub request:    HttpRequest,
    pub status:     u16,
    pub elapsed_ms: u128,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    pub fn load(_config: &AppConfig) -> Result<Self> {
        let path = AppConfig::history_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)?;
        serde_json::from_str(&raw).map_err(crate::error::AppError::Json)
    }

    pub fn save(&self, _config: &AppConfig) -> Result<()> {
        let path = AppConfig::history_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn push(&mut self, entry: HistoryEntry, max: usize) {
        self.entries.insert(0, entry);
        self.entries.truncate(max);
    }
}

// ── public API ────────────────────────────────────────────────────────────────

/// Append a completed request to the persistent history file.
pub fn record(req: &HttpRequest, status: u16, elapsed_ms: u128, config: &AppConfig) -> Result<()> {
    let mut history = History::load(config)?;
    let next_id = history.entries.first().map(|e| e.id + 1).unwrap_or(1);
    history.push(
        HistoryEntry {
            id: next_id,
            timestamp: Utc::now(),
            request: req.clone(),
            status,
            elapsed_ms,
        },
        config.max_history,
    );
    history.save(config)
}

/// Print the last `limit` history entries (or clear them).
pub fn print_history(limit: usize, clear: bool, config: &AppConfig) -> Result<()> {
    if clear {
        let empty = History::default();
        empty.save(config)?;
        display::print_success("History cleared.");
        return Ok(());
    }

    let history = History::load(config)?;
    if history.entries.is_empty() {
        display::print_info("No history found.");
        return Ok(());
    }

    display::print_header("Request History");
    for entry in history.entries.iter().take(limit) {
        let ts     = entry.timestamp.format("%Y-%m-%d %H:%M:%S");
        let status = use_colored_status(entry.status);
        println!(
            "  #{:<4} [{}]  {} {}  {}  ({}ms)",
            entry.id, ts, entry.request.method, entry.request.url,
            status, entry.elapsed_ms,
        );
    }
    Ok(())
}

fn use_colored_status(code: u16) -> String {
    use colored::Colorize;
    let s = code.to_string();
    match code {
        200..=299 => s.green().to_string(),
        300..=399 => s.yellow().to_string(),
        400..=499 => s.red().to_string(),
        500..=599 => s.bright_red().to_string(),
        _         => s,
    }
}
