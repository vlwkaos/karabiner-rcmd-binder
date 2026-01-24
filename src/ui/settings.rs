use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, SettingsField};

pub fn draw_settings(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Settings ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Default browser
            Constraint::Min(0),    // Padding
        ])
        .split(inner);

    // Default browser
    let browser_style = if app.settings_field == SettingsField::DefaultBrowser {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let browser_block = Block::default()
        .borders(Borders::ALL)
        .title(" Default Browser (</> to change) ")
        .border_style(browser_style);

    let browser_text = app.config.settings.default_browser.display_name();
    let browser_para = Paragraph::new(browser_text).block(browser_block);
    frame.render_widget(browser_para, chunks[0]);
}
