use crate::cli::CollectionAction;
use crate::config::AppConfig;
use crate::display;
use crate::error::{AppError, Result};
use crate::request::HttpRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub name:        String,
    pub description: Option<String>,
    pub requests:    HashMap<String, SavedRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedRequest {
    pub name:        String,
    pub description: Option<String>,
    pub request:     HttpRequest,
}

// ── public API ────────────────────────────────────────────────────────────────

pub fn handle_action(action: &CollectionAction, config: &AppConfig) -> Result<()> {
    match action {
        CollectionAction::List                     => list(config),
        CollectionAction::Show { name }            => show(name, config),
        CollectionAction::Run { collection, request } => run(collection, request, config),
        CollectionAction::Delete { name }          => delete(name, config),
    }
}

pub fn load(config: &AppConfig, name: &str) -> Result<Collection> {
    let path = collection_path(config, name);
    if !path.exists() {
        return Err(AppError::CollectionNotFound { name: name.to_string() });
    }
    let raw = std::fs::read_to_string(&path)?;
    serde_json::from_str(&raw).map_err(AppError::Json)
}

pub fn save(config: &AppConfig, col: &Collection) -> Result<()> {
    let dir = &config.collections_dir;
    std::fs::create_dir_all(dir)?;
    std::fs::write(
        collection_path(config, &col.name),
        serde_json::to_string_pretty(col)?,
    )?;
    Ok(())
}

// ── private helpers ───────────────────────────────────────────────────────────

fn collection_path(config: &AppConfig, name: &str) -> PathBuf {
    config.collections_dir.join(format!("{name}.json"))
}

fn list(config: &AppConfig) -> Result<()> {
    let dir = &config.collections_dir;
    if !dir.exists() {
        display::print_info("No collections found. Create one at ~/.config/rustrest/collections/");
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "json"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    if entries.is_empty() {
        display::print_info("No collections found.");
        return Ok(());
    }

    display::print_header("Collections");
    for entry in entries {
        let path = entry.path();
        if let Some(stem) = path.file_stem() {
            println!("  • {}", stem.to_string_lossy());
        }
    }
    Ok(())
}

fn show(name: &str, config: &AppConfig) -> Result<()> {
    let col = load(config, name)?;
    display::print_header(&format!("Collection: {}", col.name));
    if let Some(desc) = &col.description {
        println!("  {desc}");
    }
    println!();
    let mut keys: Vec<_> = col.requests.keys().collect();
    keys.sort();
    for key in keys {
        let r = &col.requests[key];
        println!(
            "  {:20}  {} {}",
            key.bright_cyan_display(),
            r.request.method,
            r.request.url
        );
    }
    Ok(())
}

trait BrightCyanDisplay {
    fn bright_cyan_display(&self) -> colored::ColoredString;
}
impl BrightCyanDisplay for str {
    fn bright_cyan_display(&self) -> colored::ColoredString {
        use colored::Colorize;
        self.bright_cyan()
    }
}

fn run(collection: &str, request_name: &str, config: &AppConfig) -> Result<()> {
    let col = load(config, collection)?;
    let saved = col.requests.get(request_name).ok_or_else(|| {
        AppError::Other(format!(
            "Request '{request_name}' not found in collection '{collection}'"
        ))
    })?;
    display::print_info(&format!(
        "Loaded: {} {}",
        saved.request.method, saved.request.url
    ));
    // Execution is delegated to the caller (main.rs) via the returned request.
    // Phase 2: wire this up properly.
    let _ = saved;
    Ok(())
}

fn delete(name: &str, config: &AppConfig) -> Result<()> {
    let path = collection_path(config, name);
    if !path.exists() {
        return Err(AppError::CollectionNotFound { name: name.to_string() });
    }
    std::fs::remove_file(&path)?;
    display::print_success(&format!("Deleted collection '{name}'"));
    Ok(())
}
