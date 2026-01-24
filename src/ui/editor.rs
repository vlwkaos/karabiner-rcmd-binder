use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::{ActionEditorField, ActionType, App, EditorField};

pub fn draw_editor(frame: &mut Frame, app: &App, area: Rect) {
    let editor = match &app.binding_editor {
        Some(e) => e,
        None => return,
    };

    // Center the popup
    let popup_area = centered_rect(70, 80, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Check if we're editing an action
    if let Some(action_editor) = &editor.action_editor {
        draw_action_editor(frame, app, action_editor, popup_area);
        return;
    }

    // Determine mode based on current field
    let is_input = matches!(editor.field, EditorField::Key | EditorField::Description);
    let mode_label = if is_input { "INPUT" } else { "NAV" };
    let border_color = if is_input { Color::Green } else { Color::Cyan };
    
    let base_title = if app.editing_binding_index.is_some() {
        "Edit Binding"
    } else {
        "New Binding"
    };
    let title = format!(" [{}] {} ", mode_label, base_title);
    
    let bottom_hint = if is_input {
        " (Tab)next | (Esc)cancel "
    } else {
        " (s)SAVE | (Esc)cancel "
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_bottom(Line::from(bottom_hint).right_aligned())
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Key
            Constraint::Length(3), // Description
            Constraint::Min(5),    // Actions list
        ])
        .split(inner);

    // Key field
    let key_style = if editor.field == EditorField::Key {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let key_block = Block::default()
        .borders(Borders::ALL)
        .title(" Key (rcmd+) ")
        .border_style(key_style);
    let key_text = Paragraph::new(editor.key.as_str()).block(key_block);
    frame.render_widget(key_text, chunks[0]);

    // Description field
    let desc_style = if editor.field == EditorField::Description {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let desc_block = Block::default()
        .borders(Borders::ALL)
        .title(" Description ")
        .border_style(desc_style);
    let desc_text = Paragraph::new(editor.description.as_str()).block(desc_block);
    frame.render_widget(desc_text, chunks[1]);

    // Actions list
    let actions_style = if editor.field == EditorField::Actions {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let action_items: Vec<ListItem> = editor
        .actions
        .iter()
        .enumerate()
        .map(|(i, action)| {
            let prefix = if i == editor.selected_action && editor.field == EditorField::Actions {
                "> "
            } else {
                "  "
            };
            let type_label = format!("[{}] ", action.type_name());
            let summary = action.display_summary();

            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(type_label, Style::default().fg(Color::Magenta)),
                Span::raw(summary),
            ]))
        })
        .collect();

    let actions_block = Block::default()
        .borders(Borders::ALL)
        .title(" Actions (cycle order) ")
        .title_bottom(Line::from(" (a)dd (e)dit (d)elete (k)up (j)down ").right_aligned())
        .border_style(actions_style);

    if action_items.is_empty() {
        let empty = Paragraph::new("  No actions. Press 'a' to add.")
            .style(Style::default().fg(Color::DarkGray))
            .block(actions_block);
        frame.render_widget(empty, chunks[2]);
    } else {
        let actions_list = List::new(action_items).block(actions_block);
        let mut state = ListState::default();
        state.select(Some(editor.selected_action));
        frame.render_stateful_widget(actions_list, chunks[2], &mut state);
    }

    // Render autocomplete LAST so it appears on top of all other widgets
    if app.show_autocomplete && editor.field == EditorField::Key {
        let suggestions_height = (app.autocomplete_suggestions.len() as u16 + 2).min(8);
        let autocomplete_area = Rect {
            x: chunks[0].x + 1,
            y: chunks[0].y + chunks[0].height,
            width: chunks[0].width.saturating_sub(2).min(35),
            height: suggestions_height,
        };
        // Only draw if within screen bounds
        if autocomplete_area.y + autocomplete_area.height <= area.height {
            draw_autocomplete(frame, app, autocomplete_area);
        }
    }
}

fn draw_action_editor(
    frame: &mut Frame,
    app: &App,
    action_editor: &crate::app::ActionEditor,
    area: Rect,
) {
    // Determine mode based on current field
    let is_input = action_editor.field == ActionEditorField::Target;
    let mode_label = if is_input { "INPUT" } else { "NAV" };
    let border_color = if is_input { Color::Green } else { Color::Cyan };
    
    let title = format!(" [{}] Edit Action ", mode_label);
    
    let bottom_hint = if is_input {
        " (Tab)next | (Esc)cancel | (Enter)select "
    } else {
        " (s)SAVE | (←→)change | (Tab)next | (Esc)cancel "
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_bottom(Line::from(bottom_hint).right_aligned())
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Different layout based on action type
    let num_fields = match action_editor.action_type {
        ActionType::Url => 4,
        _ => 2,
    };

    let mut constraints = vec![Constraint::Length(3); num_fields];
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    // Type field (always first)
    let type_style = field_style(action_editor.field == ActionEditorField::Type);
    let type_block = Block::default()
        .borders(Borders::ALL)
        .title(" Type (</> to change) ")
        .border_style(type_style);
    let type_text = Paragraph::new(action_editor.action_type.as_str()).block(type_block);
    frame.render_widget(type_text, chunks[0]);

    // Target field
    let target_style = field_style(action_editor.field == ActionEditorField::Target);
    let target_title = match action_editor.action_type {
        ActionType::App if app.apps_loading => " App Name (Loading...) ",
        ActionType::App => " App Name ",
        ActionType::Url => " URL ",
        ActionType::Shell => " Shell Command ",
    };
    let target_block = Block::default()
        .borders(Borders::ALL)
        .title(target_title)
        .border_style(target_style);
    let target_text = Paragraph::new(action_editor.target.as_str()).block(target_block);
    frame.render_widget(target_text, chunks[1]);

    // URL-specific fields
    if action_editor.action_type == ActionType::Url {
        // Match type
        let match_style = field_style(action_editor.field == ActionEditorField::MatchType);
        let match_block = Block::default()
            .borders(Borders::ALL)
            .title(" Match Type (</> to change) ")
            .border_style(match_style);
        let match_text = Paragraph::new(action_editor.match_type.as_str()).block(match_block);
        frame.render_widget(match_text, chunks[2]);

        // Browser override
        let browser_style = field_style(action_editor.field == ActionEditorField::Browser);
        let browser_block = Block::default()
            .borders(Borders::ALL)
            .title(" Browser (</> to change, empty=default) ")
            .border_style(browser_style);
        let browser_text = action_editor
            .browser
            .as_ref()
            .map(|b| b.display_name())
            .unwrap_or("(use default)");
        let browser_para = Paragraph::new(browser_text).block(browser_block);
        frame.render_widget(browser_para, chunks[3]);
    }

    // Render autocomplete LAST so it appears on top of all other widgets
    if app.show_autocomplete && action_editor.field == ActionEditorField::Target {
        let suggestions_height = (app.autocomplete_suggestions.len() as u16 + 2).min(8);
        let autocomplete_area = Rect {
            x: chunks[1].x + 1,
            y: chunks[1].y + chunks[1].height,
            width: chunks[1].width.saturating_sub(2).min(45),
            height: suggestions_height,
        };
        draw_autocomplete(frame, app, autocomplete_area);
    }
}

fn draw_autocomplete(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);

    let items: Vec<ListItem> = app
        .autocomplete_suggestions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == app.autocomplete_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(s.as_str(), style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Keys "));

    frame.render_widget(list, area);
}

fn field_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
