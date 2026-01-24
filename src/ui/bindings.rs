use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

pub fn draw_bindings(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .config
        .bindings
        .iter()
        .enumerate()
        .map(|(i, binding)| {
            let key = format!("{:<12}", binding.display_key());
            let actions = format!("{:<40}", truncate(&binding.actions_summary(), 38));
            let desc = truncate(&binding.description, 30);

            let style = if i == app.selected_binding {
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

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bindings ")
                .title_bottom(Line::from(" Key          Actions                                  Description ").left_aligned()),
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
