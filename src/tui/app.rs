#![allow(dead_code)]

use crate::client::HttpClient;
use crate::config::AppConfig;
use crate::request::{HttpMethod, HttpRequest};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusArea {
    MethodSelector,
    UrlInput,
    HeadersInput,
    BodyInput,
    ResponseView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    /// Navigate between panels.
    Normal,
    /// Edit the focused input.
    Insert,
}

pub struct App {
    pub mode:   AppMode,
    pub focus:  FocusArea,

    // ── outgoing request ─────────────────────────────────────────────────────
    pub method:      HttpMethod,
    pub url:         String,
    pub headers_raw: String,
    pub body_raw:    String,

    // ── last response ────────────────────────────────────────────────────────
    pub response_text: String,
    pub status_code:   Option<u16>,
    pub elapsed_ms:    Option<u128>,

    pub is_loading:    bool,
    pub error_message: Option<String>,

    pub config: AppConfig,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode:          AppMode::Normal,
            focus:         FocusArea::UrlInput,
            method:        HttpMethod::Get,
            url:           String::new(),
            headers_raw:   String::new(),
            body_raw:      String::new(),
            response_text: String::new(),
            status_code:   None,
            elapsed_ms:    None,
            is_loading:    false,
            error_message: None,
            config:        AppConfig::load().unwrap_or_default(),
        }
    }

    /// Cycle through HTTP methods.
    pub fn next_method(&mut self) {
        self.method = match self.method {
            HttpMethod::Get    => HttpMethod::Post,
            HttpMethod::Post   => HttpMethod::Put,
            HttpMethod::Put    => HttpMethod::Patch,
            HttpMethod::Patch  => HttpMethod::Delete,
            HttpMethod::Delete => HttpMethod::Head,
            HttpMethod::Head   => HttpMethod::Get,
        };
    }

    /// Move focus to the next panel.
    pub fn next_focus(&mut self) {
        self.focus = match self.focus {
            FocusArea::MethodSelector => FocusArea::UrlInput,
            FocusArea::UrlInput       => FocusArea::HeadersInput,
            FocusArea::HeadersInput   => FocusArea::BodyInput,
            FocusArea::BodyInput      => FocusArea::ResponseView,
            FocusArea::ResponseView   => FocusArea::MethodSelector,
        };
    }

    /// Send the current request and populate the response pane.
    pub async fn send_request(&mut self) -> Result<()> {
        self.is_loading    = true;
        self.error_message = None;

        let req = HttpRequest {
            method:  self.method.clone(),
            url:     self.url.clone(),
            headers: HashMap::new(),
            query:   HashMap::new(),
            body:    None,
        };

        let client = HttpClient::new(&self.config)?;
        let start  = std::time::Instant::now();

        match client.send(req).await {
            Ok(resp) => {
                self.status_code = Some(resp.status().as_u16());
                self.elapsed_ms  = Some(start.elapsed().as_millis());
                let body = resp.text().await.unwrap_or_default();
                self.response_text = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| serde_json::to_string_pretty(&v).ok())
                    .unwrap_or(body);
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
            }
        }

        self.is_loading = false;
        Ok(())
    }
}
