use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

pub fn draw_bindings(frame: &mut Frame, app: &App, area: Rect) {
    let saved_count = app.config.bindings.len();

    // Create items for saved bindings
    let mut items: Vec<ListItem> = app
        .config
        .bindings
        .iter()
        .enumerate()
        .map(|(i, binding)| {
            let key = format!("{:<12}", binding.display_key(&app.config.settings.anchor_key));
            let actions = format!("{:<40}", truncate(&binding.actions_summary(), 38));
            let desc = truncate(&binding.description, 30);

            let is_selected = i == app.selected_binding;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(key, style),
                Span::styled(actions, style.fg(Color::Cyan)),
                Span::styled(desc, style.fg(Color::DarkGray)),
            ]);

            ListItem::new(line)
        })
        .collect();

    // Create items for dynamic bindings (darker color)
    let dynamic_items: Vec<ListItem> = app
        .dynamic_bindings
        .iter()
        .enumerate()
        .map(|(i, binding)| {
            let key = format!("{:<12}", binding.display_key(&app.config.settings.anchor_key));
            let actions = format!("{:<40}", truncate(&binding.actions_summary(), 38));
            let desc = truncate(&binding.description, 30);

            let global_index = saved_count + i;
            let is_selected = global_index == app.selected_binding;

            let base_color = Color::DarkGray;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(base_color)
            };

            let line = Line::from(vec![
                Span::styled(key, style),
                Span::styled(actions, style),
                Span::styled(desc, style),
            ]);

            ListItem::new(line)
        })
        .collect();

    // Combine both lists
    items.extend(dynamic_items);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bindings ")
                .title_bottom(
                    Line::from(
                        " Key          Actions                                  Description ",
                    )
                    .left_aligned(),
                ),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.selected_binding));

    frame.render_stateful_widget(list, area, &mut state);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
