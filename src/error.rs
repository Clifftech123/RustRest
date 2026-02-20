use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Collection '{name}' not found")]
    CollectionNotFound { name: String },

    #[error("Invalid header '{header}' — expected 'Key: Value'")]
    InvalidHeader { header: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

/// Crate-wide Result alias.
pub type Result<T> = std::result::Result<T, AppError>;
