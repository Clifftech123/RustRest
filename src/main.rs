use anyhow::Context;
use clap::Parser;
use rustrest::{
    cli::{Cli, Command, ConfigAction},
    client, collections, config, display, history, request, response, tui,
};
use std::time::Instant;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        display::print_error(&e.to_string());
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = config::AppConfig::load().context("Failed to load configuration")?;

    match &cli.command {
        // ── HTTP commands ────────────────────────────────────────────────────
        Command::Get { .. }
        | Command::Post { .. }
        | Command::Put { .. }
        | Command::Patch { .. }
        | Command::Delete { .. }
        | Command::Head { .. } => {
            let http = client::HttpClient::with_options(cli.timeout, true, cli.insecure)?;
            let req  = request::HttpRequest::from_command(&cli.command, &cfg)?;

            if cli.verbose {
                display::print_request_line(&req.method.to_string(), &req.url);
                let hdrs: Vec<_> = req.headers.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                display::print_request_headers(&hdrs);
            }

            let start   = Instant::now();
            let resp    = http.send(req.clone()).await?;
            let elapsed = start.elapsed().as_millis();
            let status  = resp.status().as_u16();

            response::print_response(resp, &cli, elapsed).await?;
            history::record(&req, status, elapsed, &cfg)?;
        }

        // ── collections ──────────────────────────────────────────────────────
        Command::Collection { action } => {
            collections::handle_action(action, &cfg)?;
        }

        // ── history ──────────────────────────────────────────────────────────
        Command::History { limit, clear } => {
            history::print_history(*limit, *clear, &cfg)?;
        }

        // ── config ───────────────────────────────────────────────────────────
        Command::Config { action } => match action {
            ConfigAction::Show => {
                println!("{}", serde_json::to_string_pretty(&cfg)?);
            }
            ConfigAction::Set { key, value } => {
                display::print_warning(&format!(
                    "Setting '{key}' = '{value}' is not yet implemented"
                ));
            }
            ConfigAction::Reset => {
                config::AppConfig::default().save()?;
                display::print_success("Configuration reset to defaults.");
            }
        },

        // ── TUI ──────────────────────────────────────────────────────────────
        Command::Tui => {
            tui::run().await?;
        }
    }

    Ok(())
}
