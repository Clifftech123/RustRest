use crate::config::AppConfig;
use crate::error::{AppError, Result};
use crate::request::{HttpMethod, HttpRequest, RequestBody};
use reqwest::Client;
use std::time::Duration;

pub struct HttpClient {
    inner: Client,
}

impl HttpClient {
    /// Create a client configured from `AppConfig`.
    pub fn new(config: &AppConfig) -> Result<Self> {
        Self::with_options(config.timeout_secs, config.follow_redirects, false)
    }

    /// Create a client with explicit options (used by the binary entry point).
    pub fn with_options(timeout_secs: u64, follow_redirects: bool, insecure: bool) -> Result<Self> {
        let redirect_policy = if follow_redirects {
            reqwest::redirect::Policy::limited(10)
        } else {
            reqwest::redirect::Policy::none()
        };

        let mut builder = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .redirect(redirect_policy)
            .user_agent(concat!("rustrest/", env!("CARGO_PKG_VERSION")));

        if insecure {
            builder = builder.danger_accept_invalid_certs(true);
        }

        Ok(Self { inner: builder.build().map_err(AppError::Http)? })
    }

    /// Execute an `HttpRequest` and return the raw reqwest response.
    pub async fn send(&self, req: HttpRequest) -> Result<reqwest::Response> {
        let mut builder = match req.method {
            HttpMethod::Get    => self.inner.get(&req.url),
            HttpMethod::Post   => self.inner.post(&req.url),
            HttpMethod::Put    => self.inner.put(&req.url),
            HttpMethod::Patch  => self.inner.patch(&req.url),
            HttpMethod::Delete => self.inner.delete(&req.url),
            HttpMethod::Head   => self.inner.head(&req.url),
        };

        for (key, value) in &req.headers {
            builder = builder.header(key, value);
        }

        if !req.query.is_empty() {
            builder = builder.query(&req.query.iter().collect::<Vec<_>>());
        }

        builder = match req.body {
            Some(RequestBody::Json(v))  => builder.json(&v),
            Some(RequestBody::Form(map)) => builder.form(&map),
            Some(RequestBody::Text(s))  => builder.header("Content-Type", "text/plain").body(s),
            Some(RequestBody::File(path)) => {
                let bytes = std::fs::read(&path)?;
                builder.body(bytes)
            }
            None => builder,
        };

        builder.send().await.map_err(AppError::Http)
    }
}
