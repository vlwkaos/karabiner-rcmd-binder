use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use super::bindings::draw_bindings;
use super::editor::draw_editor;
use super::settings::draw_settings;
use crate::app::{App, InputMode, Tab};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Status/Help
        ])
        .split(frame.area());

    draw_tabs(frame, app, chunks[0]);
    draw_content(frame, app, chunks[1]);
    draw_status_bar(frame, app, chunks[2]);
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let binding_count = app.config.bindings.len();
    let titles = vec![
        format!("Bindings ({})", binding_count),
        "Settings".to_string(),
    ];
    let selected = match app.tab {
        Tab::Bindings => 0,
        Tab::Settings => 1,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " karabiner-rcmd-binder v{} ",
            env!("CARGO_PKG_VERSION")
        )))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    // If we're editing, show the editor popup
    if app.binding_editor.is_some() {
        // Draw the background content dimmed
        match app.tab {
            Tab::Bindings => draw_bindings(frame, app, area),
            Tab::Settings => draw_settings(frame, app, area),
        }
        // Draw editor overlay
        draw_editor(frame, app, area);
    } else {
        match app.tab {
            Tab::Bindings => draw_bindings(frame, app, area),
            Tab::Settings => draw_settings(frame, app, area),
        }
    }
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let (msg, style) = if let Some(ref status) = app.status_message {
        (status.clone(), Style::default().fg(Color::Yellow))
    } else {
        let help = match (&app.input_mode, &app.binding_editor) {
            (InputMode::Normal, None) => match app.tab {
                Tab::Bindings => "(a)dd (e)dit (d)elete (j/k)nav (Tab)switch (s)ave (q)uit",
                Tab::Settings => "(←→)change (Tab)switch (s)ave (q)uit",
            },
            (InputMode::Editing, Some(editor)) => {
                if let Some(action_editor) = &editor.action_editor {
                    // Action Editor
                    if action_editor.edit_mode {
                        // EDIT mode: typing in Target field
                        if action_editor.action_type == crate::app::ActionType::App {
                            "[EDIT] Type app name | (↑↓)autocomplete (Enter)finish (Tab)next (Esc)exit"
                        } else {
                            "[EDIT] Type target | (Enter)finish (Tab)next (Esc)exit"
                        }
                    } else {
                        // NAV mode
                        match action_editor.field {
                            crate::app::ActionEditorField::Target => {
                                "[NAV] (Enter)edit (s)ave (Tab)next (Esc)cancel"
                            }
                            _ => {
                                "[NAV] (←→)change (s)ave (Tab)next (Esc)cancel"
                            }
                        }
                    }
                } else {
                    // Binding Editor
                    if editor.edit_mode {
                        // EDIT mode: typing in Key/Description
                        match editor.field {
                            crate::app::EditorField::Key => {
                                "[EDIT] Type key | (↑↓)autocomplete (Enter)finish (Tab)next (Esc)exit"
                            }
                            crate::app::EditorField::Description => {
                                "[EDIT] Type description | (Enter)finish (Tab)next (Esc)exit"
                            }
                            _ => "[NAV]",
                        }
                    } else {
                        // NAV mode
                        match editor.field {
                            crate::app::EditorField::Key | crate::app::EditorField::Description => {
                                "[NAV] (Enter)edit (s)ave (Tab)next (Esc)cancel"
                            }
                            crate::app::EditorField::Actions => {
                                "[NAV] (a)dd (e)dit (d)elete (j/k)nav (J/K)move (s)ave (Tab)next (Esc)cancel"
                            }
                        }
                    }
                }
            }
            _ => "",
        };
        (help.to_string(), Style::default().fg(Color::Gray))
    };

    let paragraph = Paragraph::new(Line::from(vec![Span::styled(msg, style)]))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}
