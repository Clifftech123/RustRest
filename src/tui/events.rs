use super::app::{App, AppMode, FocusArea};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

pub enum Action {
    Quit,
    SendRequest,
}

/// Poll for a keyboard event (non-blocking with 16 ms timeout ≈ 60 fps).
pub fn handle_events(app: &mut App) -> Result<Option<Action>> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            return match app.mode {
                AppMode::Normal => handle_normal(app, key),
                AppMode::Insert => handle_insert(app, key),
            };
        }
    }
    Ok(None)
}

fn handle_normal(app: &mut App, key: event::KeyEvent) -> Result<Option<Action>> {
    match key.code {
        KeyCode::Char('q') => return Ok(Some(Action::Quit)),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return Ok(Some(Action::Quit));
        }
        KeyCode::Tab                                                 => app.next_focus(),
        KeyCode::Enter                                               => return Ok(Some(Action::SendRequest)),
        KeyCode::Char('m') if app.focus == FocusArea::MethodSelector => app.next_method(),
        KeyCode::Char('i')                                           => app.mode = AppMode::Insert,
        _ => {}
    }
    Ok(None)
}

fn handle_insert(app: &mut App, key: event::KeyEvent) -> Result<Option<Action>> {
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Normal,

        KeyCode::Char(c) => match app.focus {
            FocusArea::UrlInput     => app.url.push(c),
            FocusArea::HeadersInput => app.headers_raw.push(c),
            FocusArea::BodyInput    => app.body_raw.push(c),
            _                       => {}
        },

        KeyCode::Backspace => match app.focus {
            FocusArea::UrlInput     => { app.url.pop(); }
            FocusArea::HeadersInput => { app.headers_raw.pop(); }
            FocusArea::BodyInput    => { app.body_raw.pop(); }
            _                       => {}
        },

        KeyCode::Enter => match app.focus {
            FocusArea::HeadersInput => app.headers_raw.push('\n'),
            FocusArea::BodyInput    => app.body_raw.push('\n'),
            _                       => return Ok(Some(Action::SendRequest)),
        },

        _ => {}
    }
    Ok(None)
}
