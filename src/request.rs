use crate::cli::Command;
use crate::config::AppConfig;
use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method:  HttpMethod,
    pub url:     String,
    pub headers: HashMap<String, String>,
    pub query:   HashMap<String, String>,
    pub body:    Option<RequestBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get    => write!(f, "GET"),
            HttpMethod::Post   => write!(f, "POST"),
            HttpMethod::Put    => write!(f, "PUT"),
            HttpMethod::Patch  => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head   => write!(f, "HEAD"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Text(String),
    File(String),
}

impl HttpRequest {
    /// Build an `HttpRequest` from a parsed CLI `Command`.
    pub fn from_command(cmd: &Command, config: &AppConfig) -> Result<Self> {
        match cmd {
            Command::Get { url, headers, query } => {
                let mut req = Self::new(HttpMethod::Get, url, config)?;
                req.apply_headers(headers)?;
                req.apply_query(query)?;
                Ok(req)
            }
            Command::Post { url, headers, body, form } => {
                let mut req = Self::new(HttpMethod::Post, url, config)?;
                req.apply_headers(headers)?;
                req.body = resolve_body(body, form)?;
                Ok(req)
            }
            Command::Put { url, headers, body } => {
                let mut req = Self::new(HttpMethod::Put, url, config)?;
                req.apply_headers(headers)?;
                req.body = resolve_body(body, &[])?;
                Ok(req)
            }
            Command::Patch { url, headers, body } => {
                let mut req = Self::new(HttpMethod::Patch, url, config)?;
                req.apply_headers(headers)?;
                req.body = resolve_body(body, &[])?;
                Ok(req)
            }
            Command::Delete { url, headers } => {
                let mut req = Self::new(HttpMethod::Delete, url, config)?;
                req.apply_headers(headers)?;
                Ok(req)
            }
            Command::Head { url, headers } => {
                let mut req = Self::new(HttpMethod::Head, url, config)?;
                req.apply_headers(headers)?;
                Ok(req)
            }
            _ => Err(AppError::Other("Not an HTTP command".into())),
        }
    }

    fn new(method: HttpMethod, raw_url: &str, config: &AppConfig) -> Result<Self> {
        Ok(Self {
            method,
            url: resolve_url(raw_url, config.base_url.as_deref())?,
            headers: config.default_headers.clone(),
            query: HashMap::new(),
            body: None,
        })
    }

    fn apply_headers(&mut self, raw: &[String]) -> Result<()> {
        for h in raw {
            let (k, v) = parse_header(h)?;
            self.headers.insert(k, v);
        }
        Ok(())
    }

    fn apply_query(&mut self, raw: &[String]) -> Result<()> {
        for kv in raw {
            let (k, v) = kv.split_once('=').ok_or_else(|| {
                AppError::Other(format!("Invalid query param '{kv}' — expected key=value"))
            })?;
            self.query.insert(k.to_string(), v.to_string());
        }
        Ok(())
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn resolve_url(url: &str, base: Option<&str>) -> Result<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Url::parse(url)?;
        return Ok(url.to_string());
    }
    match base {
        Some(b) => Ok(Url::parse(b)?.join(url)?.to_string()),
        None => {
            let with_scheme = format!("http://{url}");
            Url::parse(&with_scheme)?;
            Ok(with_scheme)
        }
    }
}

fn parse_header(raw: &str) -> Result<(String, String)> {
    raw.split_once(':')
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .ok_or_else(|| AppError::InvalidHeader { header: raw.to_string() })
}

fn resolve_body(body: &Option<String>, form: &[String]) -> Result<Option<RequestBody>> {
    if !form.is_empty() {
        let mut map = HashMap::new();
        for kv in form {
            let (k, v) = kv.split_once('=').ok_or_else(|| {
                AppError::Other(format!("Invalid form field '{kv}' — expected key=value"))
            })?;
            map.insert(k.to_string(), v.to_string());
        }
        return Ok(Some(RequestBody::Form(map)));
    }
    match body {
        None => Ok(None),
        Some(s) if s.starts_with('@') => Ok(Some(RequestBody::File(s[1..].to_string()))),
        Some(s) => match serde_json::from_str(s) {
            Ok(v)  => Ok(Some(RequestBody::Json(v))),
            Err(_) => Ok(Some(RequestBody::Text(s.clone()))),
        },
    }
}
