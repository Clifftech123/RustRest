use super::app::{App, FocusArea};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title bar
            Constraint::Length(3), // method + URL
            Constraint::Length(8), // headers
            Constraint::Min(8),    // body | response
            Constraint::Length(1), // status bar
        ])
        .split(f.size());

    render_title(f, root[0]);
    render_request_line(f, root[1], app);
    render_headers(f, root[2], app);
    render_body_response(f, root[3], app);
    render_status_bar(f, root[4], app);
}

fn focused(is_focused: bool) -> Style {
    if is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn render_title(f: &mut Frame, area: ratatui::layout::Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("RustRest ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("i", Style::default().fg(Color::Yellow)),
        Span::raw(":edit  "),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(":focus  "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(":send  "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(":quit"),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, area);
}

fn render_request_line(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(area);

    let method_widget = Paragraph::new(app.method.to_string())
        .block(Block::default().title("Method").borders(Borders::ALL))
        .style(focused(app.focus == FocusArea::MethodSelector));
    f.render_widget(method_widget, chunks[0]);

    let url_widget = Paragraph::new(app.url.as_str())
        .block(Block::default().title("URL").borders(Borders::ALL))
        .style(focused(app.focus == FocusArea::UrlInput));
    f.render_widget(url_widget, chunks[1]);
}

fn render_headers(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let widget = Paragraph::new(app.headers_raw.as_str())
        .block(
            Block::default()
                .title("Headers  (Key: Value, one per line)")
                .borders(Borders::ALL),
        )
        .style(focused(app.focus == FocusArea::HeadersInput))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, area);
}

fn render_body_response(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let body = Paragraph::new(app.body_raw.as_str())
        .block(Block::default().title("Body (JSON)").borders(Borders::ALL))
        .style(focused(app.focus == FocusArea::BodyInput))
        .wrap(Wrap { trim: false });
    f.render_widget(body, chunks[0]);

    let resp_title = match (app.is_loading, app.status_code) {
        (true, _)    => "Response  [sending…]".to_string(),
        (_, Some(s)) => format!("Response  [{s}]"),
        _            => "Response".to_string(),
    };
    let response = Paragraph::new(app.response_text.as_str())
        .block(Block::default().title(resp_title).borders(Borders::ALL))
        .style(focused(app.focus == FocusArea::ResponseView))
        .wrap(Wrap { trim: false });
    f.render_widget(response, chunks[1]);
}

fn render_status_bar(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let span = if let Some(err) = &app.error_message {
        Span::styled(err.as_str(), Style::default().fg(Color::Red))
    } else if let Some(ms) = app.elapsed_ms {
        Span::styled(format!("OK — {ms}ms"), Style::default().fg(Color::Green))
    } else {
        Span::styled("Ready", Style::default().fg(Color::DarkGray))
    };
    f.render_widget(Paragraph::new(Line::from(span)), area);
}
