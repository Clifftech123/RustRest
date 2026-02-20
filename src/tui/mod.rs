pub mod app;
pub mod events;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Entry point for the interactive TUI mode (Phase 3).
pub async fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend  = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let mut app  = app::App::new();

    let result = run_loop(&mut term, &mut app).await;

    // Always restore the terminal, even on error.
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;

    result
}

async fn run_loop<B: ratatui::backend::Backend>(
    term: &mut Terminal<B>,
    app: &mut app::App,
) -> Result<()> {
    loop {
        term.draw(|f| ui::draw(f, app))?;

        if let Some(action) = events::handle_events(app)? {
            match action {
                events::Action::Quit => break,
                events::Action::SendRequest => app.send_request().await?,
            }
        }
    }
    Ok(())
}
