use colored::Colorize;

/// Print the HTTP status line, colour-coded by class.
pub fn print_status_line(status: u16, reason: &str, elapsed_ms: u128) {
    let label = format!("{status} {reason}");
    let coloured = match status {
        200..=299 => label.green().bold(),
        300..=399 => label.yellow().bold(),
        400..=499 => label.red().bold(),
        500..=599 => label.bright_red().bold(),
        _         => label.white().bold(),
    };
    println!("{coloured}  {}", format!("({elapsed_ms}ms)").dimmed());
}

/// Print response headers (verbose mode).
pub fn print_response_headers(headers: &[(String, String)]) {
    println!("{}", "─".repeat(60).dimmed());
    for (k, v) in headers {
        println!("{}  {}", k.cyan(), v);
    }
    println!("{}", "─".repeat(60).dimmed());
}

/// Print the outgoing request line (verbose mode).
pub fn print_request_line(method: &str, url: &str) {
    let arrow = "→".bright_blue().bold();
    let m = match method {
        "GET"    => method.bright_blue().bold(),
        "POST"   => method.bright_green().bold(),
        "PUT"    => method.bright_yellow().bold(),
        "PATCH"  => method.bright_magenta().bold(),
        "DELETE" => method.bright_red().bold(),
        _        => method.white().bold(),
    };
    println!("{arrow} {m}  {url}");
}

/// Print outgoing headers (verbose mode).
pub fn print_request_headers(headers: &[(String, String)]) {
    for (k, v) in headers {
        println!("  {}  {v}", k.dimmed());
    }
}

/// Pretty-print a response body, formatting JSON when detected.
pub fn print_pretty_body(body: &str, content_type: &str) {
    let is_json = content_type.contains("application/json")
        || content_type.contains("text/json");

    if is_json {
        match serde_json::from_str::<serde_json::Value>(body) {
            Ok(v) => {
                let pretty = serde_json::to_string_pretty(&v)
                    .unwrap_or_else(|_| body.to_string());
                println!("{}", colorize_json(&pretty));
            }
            Err(_) => println!("{body}"),
        }
    } else {
        println!("{body}");
    }
}

/// Minimal JSON syntax colouring (keys cyan, strings yellow).
fn colorize_json(json: &str) -> String {
    let mut out = String::with_capacity(json.len() * 2);
    for line in json.lines() {
        let trimmed = line.trim_start();
        let indent  = &line[..line.len() - trimmed.len()];

        // Key–value line: `"key": ...`
        if trimmed.starts_with('"') {
            if let Some(colon_pos) = trimmed.find("\": ") {
                let key   = &trimmed[..=colon_pos + 1]; // includes `": `
                let value = &trimmed[colon_pos + 3..];  // rest of line
                let coloured_value = if value.starts_with('"') {
                    value.yellow().to_string()
                } else {
                    value.to_string()
                };
                out.push_str(&format!("{indent}{}{coloured_value}\n", key.cyan()));
                continue;
            }
        }
        out.push_str(&format!("{line}\n"));
    }
    out.trim_end().to_string()
}

// ── status helpers ────────────────────────────────────────────────────────────

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

pub fn print_warning(msg: &str) {
    eprintln!("{} {}", "".yellow().bold(), msg.yellow());
}

pub fn print_info(msg: &str) {
    println!("{}", msg.dimmed());
}

pub fn print_header(title: &str) {
    println!("\n{}", title.bright_white().bold().underline());
}
