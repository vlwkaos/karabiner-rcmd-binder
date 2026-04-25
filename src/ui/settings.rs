use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, SettingsField};
use crate::config::CenterMouseMode;

pub fn draw_settings(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Settings ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Anchor key
            Constraint::Length(3), // Default browser
            Constraint::Length(3), // Center mouse
            Constraint::Min(0),    // Padding
        ])
        .split(inner);

    // Anchor key
    let anchor_style = if app.settings_field == SettingsField::AnchorKey {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let anchor_block = Block::default()
        .borders(Borders::ALL)
        .title(" Anchor Key (</> to change) ")
        .border_style(anchor_style);

    let anchor_text = app.config.settings.anchor_key.display_name();
    let anchor_para = Paragraph::new(anchor_text).block(anchor_block);
    frame.render_widget(anchor_para, chunks[0]);

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
    frame.render_widget(browser_para, chunks[1]);

    // Center mouse toggle
    let cm_style = if app.settings_field == SettingsField::CenterMouse {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let cm_value = app.config.settings.center_mouse.display_name();
    let cm_value_style = match app.config.settings.center_mouse {
        CenterMouseMode::Off => Style::default().fg(Color::DarkGray),
        CenterMouseMode::Always => Style::default().fg(Color::Green),
        CenterMouseMode::MultiMonitorOnly => Style::default().fg(Color::Cyan),
    };
    let cm_block = Block::default()
        .borders(Borders::ALL)
        .title(" Center Mouse on App Focus (space) ")
        .border_style(cm_style);
    let cm_para = Paragraph::new(Line::from(vec![
        Span::raw(" "),
        Span::styled(cm_value, cm_value_style),
    ]))
    .block(cm_block);
    frame.render_widget(cm_para, chunks[2]);
}
