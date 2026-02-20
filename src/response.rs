use crate::cli::{Cli, OutputFormat};
use crate::display;
use crate::error::Result;
use reqwest::Response;

pub struct ParsedResponse {
    pub status:       u16,
    pub status_text:  String,
    pub headers:      Vec<(String, String)>,
    pub body:         String,
    pub content_type: String,
    pub size_bytes:   usize,
    pub elapsed_ms:   u128,
}

/// Consume a raw `reqwest::Response`, print it according to CLI flags, and
/// return the structured representation for further use (e.g. history).
pub async fn print_response(resp: Response, cli: &Cli, elapsed_ms: u128) -> Result<ParsedResponse> {
    let parsed = parse_response(resp, elapsed_ms).await?;

    if !cli.quiet {
        display::print_status_line(parsed.status, &parsed.status_text, elapsed_ms);
    }

    if cli.verbose && !cli.quiet {
        display::print_response_headers(&parsed.headers);
    }

    match cli.format {
        OutputFormat::Pretty => display::print_pretty_body(&parsed.body, &parsed.content_type),
        OutputFormat::Json   => println!("{}", parsed.body),
        OutputFormat::Plain  => print!("{}", parsed.body),
    }

    Ok(parsed)
}

pub async fn parse_response(resp: Response, elapsed_ms: u128) -> Result<ParsedResponse> {
    let status      = resp.status().as_u16();
    let status_text = resp.status().canonical_reason().unwrap_or("Unknown").to_string();

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let body       = resp.text().await.map_err(crate::error::AppError::Http)?;
    let size_bytes = body.len();

    Ok(ParsedResponse { status, status_text, headers, body, content_type, size_bytes, elapsed_ms })
}
