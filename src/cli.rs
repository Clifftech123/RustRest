use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name    = "rr",
    bin_name = "rr",
    about   = "RustRest — a modern HTTP client for the terminal",
    version,
    propagate_version = true
)]
pub struct Cli {
    /// Response output format.
    #[arg(short, long, global = true, default_value = "pretty", value_name = "FORMAT")]
    pub format: OutputFormat,

    /// Print request and response headers.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except the response body.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable TLS certificate verification.
    #[arg(long, global = true)]
    pub insecure: bool,

    /// Request timeout in seconds.
    #[arg(long, global = true, default_value = "30", value_name = "SECS")]
    pub timeout: u64,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Colorized, pretty-printed output.
    Pretty,
    /// Raw JSON (no colour).
    Json,
    /// Plain text.
    Plain,
}

#[derive(Subcommand)]
pub enum Command {
    /// Send a GET request.
    Get {
        /// Target URL.
        url: String,
        /// Request headers, e.g. `"Authorization: Bearer token"`.
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
        /// Query parameters, e.g. `page=1`.
        #[arg(short, long, value_name = "KEY=VALUE")]
        query: Vec<String>,
    },

    /// Send a POST request.
    Post {
        url: String,
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
        /// JSON body or path prefixed with `@`, e.g. `@body.json`.
        #[arg(short, long, value_name = "JSON|@FILE")]
        body: Option<String>,
        /// Form fields, e.g. `name=Alice`.
        #[arg(short, long, value_name = "KEY=VALUE")]
        form: Vec<String>,
    },

    /// Send a PUT request.
    Put {
        url: String,
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
        #[arg(short, long, value_name = "JSON|@FILE")]
        body: Option<String>,
    },

    /// Send a PATCH request.
    Patch {
        url: String,
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
        #[arg(short, long, value_name = "JSON|@FILE")]
        body: Option<String>,
    },

    /// Send a DELETE request.
    Delete {
        url: String,
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
    },

    /// Send a HEAD request.
    Head {
        url: String,
        #[arg(short = 'H', long = "header", value_name = "HEADER")]
        headers: Vec<String>,
    },

    /// Manage saved request collections.
    Collection {
        #[command(subcommand)]
        action: CollectionAction,
    },

    /// Browse request history.
    History {
        /// Number of entries to show.
        #[arg(short, long, default_value = "20")]
        limit: usize,
        /// Wipe history.
        #[arg(long)]
        clear: bool,
    },

    /// View or modify app configuration.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Launch the interactive TUI (Phase 3).
    Tui,
}

#[derive(Subcommand)]
pub enum CollectionAction {
    /// List all saved collections.
    List,
    /// Show requests inside a collection.
    Show { name: String },
    /// Execute a saved request.
    Run { collection: String, request: String },
    /// Delete a collection.
    Delete { name: String },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Print current configuration.
    Show,
    /// Set a configuration key.
    Set { key: String, value: String },
    /// Reset all settings to defaults.
    Reset,
}
