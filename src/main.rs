mod app;
mod app_discovery;
mod config;
mod karabiner;
mod keycodes;
mod scripts;
mod ui;
mod validation;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use app::{ActionEditorField, App, EditorField, InputMode, Tab};
use app_discovery::DiscoveredApp;
use config::{load_config, save_config, Browser, UrlMatchType};
use karabiner::apply_to_karabiner;
use scripts::install_scripts;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load config and create app
    let config = load_config()?;
    let mut app = App::new(config);

    // Create channel for app discovery
    let (tx, rx) = mpsc::channel();

    // Load cached apps immediately and start background discovery
    app.discovered_apps = app.config.cached_apps.clone();
    app.start_app_discovery();
    spawn_app_discovery(tx.clone());

    // Run the main loop
    let res = run_app(&mut terminal, &mut app, rx, tx);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    rx: Receiver<Vec<DiscoveredApp>>,
    tx: Sender<Vec<DiscoveredApp>>,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Check for app discovery results (non-blocking)
        if let Ok(apps) = rx.try_recv() {
            app.finish_app_discovery(apps);
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Clear status on any keypress
                app.clear_status();

                match &app.input_mode {
                    InputMode::Normal => handle_normal_mode(app, key.code, key.modifiers, &tx)?,
                    InputMode::Editing => handle_editing_mode(app, key.code, key.modifiers)?,
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_normal_mode(
    app: &mut App,
    key: KeyCode,
    _modifiers: KeyModifiers,
    tx: &Sender<Vec<DiscoveredApp>>,
) -> Result<()> {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Tab => {
            app.next_tab();
        }
        KeyCode::BackTab => {
            app.prev_tab();
        }
        KeyCode::Char('s') => {
            // Save to karabiner
            save_to_karabiner(app)?;
        }
        _ => match app.tab {
            Tab::Bindings => handle_bindings_normal(app, key, tx)?,
            Tab::Settings => handle_settings_normal(app, key)?,
        },
    }
    Ok(())
}

fn handle_bindings_normal(
    app: &mut App,
    key: KeyCode,
    _tx: &Sender<Vec<DiscoveredApp>>,
) -> Result<()> {
    match key {
        KeyCode::Char('j') | KeyCode::Down => {
            app.next_binding();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.prev_binding();
        }
        KeyCode::Char('a') => {
            app.start_new_binding();
        }
        KeyCode::Char('e') | KeyCode::Enter => {
            app.start_edit_binding();
        }
        KeyCode::Char('d') => {
            app.delete_binding();
        }
        _ => {}
    }
    Ok(())
}

fn handle_settings_normal(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Char(',') | KeyCode::Char('<') => {
            app.prev_browser();
        }
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('.') | KeyCode::Char('>') => {
            app.next_browser();
        }
        _ => {}
    }
    Ok(())
}

fn handle_editing_mode(app: &mut App, key: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
    let editor = match app.binding_editor.as_mut() {
        Some(e) => e,
        None => return Ok(()),
    };

    // Check if we're editing an action
    if let Some(ref mut action_editor) = editor.action_editor {
        match key {
            KeyCode::Esc => {
                app.show_autocomplete = false;
                editor.cancel_action_edit();
            }
            KeyCode::Char('s') if action_editor.field != ActionEditorField::Target => {
                // Save action with 's' (but NOT when typing in Target field)
                app.show_autocomplete = false;
                editor.finish_action_edit();
            }
            KeyCode::Enter => {
                // Enter selects autocomplete if shown, otherwise does nothing
                if app.show_autocomplete && action_editor.field == ActionEditorField::Target {
                    if let Some(suggestion) =
                        app.autocomplete_suggestions.get(app.autocomplete_selected)
                    {
                        action_editor.target = suggestion.value.clone();
                        // Store bundle ID for App actions
                        if action_editor.action_type == crate::app::ActionType::App {
                            action_editor.bundle_id = Some(suggestion.bundle_id.clone());
                        }
                    }
                    app.show_autocomplete = false;
                }
            }
            KeyCode::Tab => {
                app.show_autocomplete = false;
                action_editor.next_field();
            }
            KeyCode::BackTab => {
                app.show_autocomplete = false;
                action_editor.prev_field();
            }
            KeyCode::Left => {
                // Arrow keys work in all fields for navigation
                match action_editor.field {
                    ActionEditorField::Type => {
                        action_editor.action_type = action_editor.action_type.prev();
                    }
                    ActionEditorField::MatchType => {
                        let types = UrlMatchType::all();
                        let idx = types
                            .iter()
                            .position(|t| t == &action_editor.match_type)
                            .unwrap_or(0);
                        let prev_idx = idx.checked_sub(1).unwrap_or(types.len() - 1);
                        action_editor.match_type = types[prev_idx].clone();
                    }
                    ActionEditorField::Browser => {
                        let browsers = Browser::all();
                        match &action_editor.browser {
                            None => {
                                action_editor.browser = Some(browsers.last().unwrap().clone());
                            }
                            Some(b) => {
                                let idx = browsers.iter().position(|x| x == b).unwrap_or(0);
                                if idx == 0 {
                                    action_editor.browser = None;
                                } else {
                                    action_editor.browser = Some(browsers[idx - 1].clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Char('<') | KeyCode::Char(',')
                if action_editor.field != ActionEditorField::Target =>
            {
                // Character shortcuts only work when NOT in text input
                match action_editor.field {
                    ActionEditorField::Type => {
                        action_editor.action_type = action_editor.action_type.prev();
                    }
                    ActionEditorField::MatchType => {
                        let types = UrlMatchType::all();
                        let idx = types
                            .iter()
                            .position(|t| t == &action_editor.match_type)
                            .unwrap_or(0);
                        let prev_idx = idx.checked_sub(1).unwrap_or(types.len() - 1);
                        action_editor.match_type = types[prev_idx].clone();
                    }
                    ActionEditorField::Browser => {
                        let browsers = Browser::all();
                        match &action_editor.browser {
                            None => {
                                action_editor.browser = Some(browsers.last().unwrap().clone());
                            }
                            Some(b) => {
                                let idx = browsers.iter().position(|x| x == b).unwrap_or(0);
                                if idx == 0 {
                                    action_editor.browser = None;
                                } else {
                                    action_editor.browser = Some(browsers[idx - 1].clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Right => {
                // Arrow keys work in all fields for navigation
                match action_editor.field {
                    ActionEditorField::Type => {
                        action_editor.action_type = action_editor.action_type.next();
                    }
                    ActionEditorField::MatchType => {
                        let types = UrlMatchType::all();
                        let idx = types
                            .iter()
                            .position(|t| t == &action_editor.match_type)
                            .unwrap_or(0);
                        let next_idx = (idx + 1) % types.len();
                        action_editor.match_type = types[next_idx].clone();
                    }
                    ActionEditorField::Browser => {
                        let browsers = Browser::all();
                        match &action_editor.browser {
                            None => {
                                action_editor.browser = Some(browsers[0].clone());
                            }
                            Some(b) => {
                                let idx = browsers.iter().position(|x| x == b).unwrap_or(0);
                                if idx == browsers.len() - 1 {
                                    action_editor.browser = None;
                                } else {
                                    action_editor.browser = Some(browsers[idx + 1].clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Char('>') | KeyCode::Char('.')
                if action_editor.field != ActionEditorField::Target =>
            {
                // Character shortcuts only work when NOT in text input
                match action_editor.field {
                    ActionEditorField::Type => {
                        action_editor.action_type = action_editor.action_type.next();
                    }
                    ActionEditorField::MatchType => {
                        let types = UrlMatchType::all();
                        let idx = types
                            .iter()
                            .position(|t| t == &action_editor.match_type)
                            .unwrap_or(0);
                        let next_idx = (idx + 1) % types.len();
                        action_editor.match_type = types[next_idx].clone();
                    }
                    ActionEditorField::Browser => {
                        let browsers = Browser::all();
                        match &action_editor.browser {
                            None => {
                                action_editor.browser = Some(browsers[0].clone());
                            }
                            Some(b) => {
                                let idx = browsers.iter().position(|x| x == b).unwrap_or(0);
                                if idx == browsers.len() - 1 {
                                    action_editor.browser = None;
                                } else {
                                    action_editor.browser = Some(browsers[idx + 1].clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Char(c) if action_editor.field == ActionEditorField::Target => {
                // When in Target field, all characters go into the text field
                action_editor.target.push(c);
                // Show autocomplete for App type
                if action_editor.action_type == crate::app::ActionType::App {
                    let target = action_editor.target.clone();
                    app.update_app_autocomplete(&target);
                }
            }
            KeyCode::Backspace => {
                if action_editor.field == ActionEditorField::Target {
                    action_editor.target.pop();
                    // Update autocomplete for App type
                    if action_editor.action_type == crate::app::ActionType::App {
                        let target = action_editor.target.clone();
                        app.update_app_autocomplete(&target);
                    }
                }
            }
            KeyCode::Down => {
                if app.show_autocomplete && action_editor.field == ActionEditorField::Target {
                    app.next_autocomplete();
                } else if action_editor.field == ActionEditorField::Target {
                    // Only navigate if not in autocomplete
                }
            }
            KeyCode::Up => {
                if app.show_autocomplete && action_editor.field == ActionEditorField::Target {
                    app.prev_autocomplete();
                }
            }
            _ => {}
        }
        return Ok(());
    }

    // Binding editor (not action editor)
    match key {
        KeyCode::Esc => {
            app.show_autocomplete = false;
            app.cancel_edit();
        }
        KeyCode::Char('s') if editor.field == EditorField::Actions => {
            // Save binding with 's' when in Actions field
            if !editor.key.is_empty() {
                app.show_autocomplete = false;
                app.save_binding();
            }
        }
        KeyCode::Enter => {
            // Enter selects autocomplete if shown
            if app.show_autocomplete {
                if let Some(suggestion) = app
                    .autocomplete_suggestions
                    .get(app.autocomplete_selected)
                    .cloned()
                {
                    if let Some(ed) = app.binding_editor.as_mut() {
                        if ed.field == EditorField::Key {
                            ed.key = suggestion.value;
                        }
                    }
                }
                app.show_autocomplete = false;
            } else if editor.field == EditorField::Actions && !editor.actions.is_empty() {
                // Edit selected action
                editor.start_editing_action();
            }
        }
        KeyCode::Tab => {
            app.show_autocomplete = false;
            editor.next_field();
        }
        KeyCode::BackTab => {
            app.show_autocomplete = false;
            editor.prev_field();
        }
        _ => match editor.field {
            EditorField::Key => handle_key_field(app, key)?,
            EditorField::Description => handle_description_field(app, key)?,
            EditorField::Actions => handle_actions_field(app, key)?,
        },
    }

    Ok(())
}

fn handle_key_field(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char(c) => {
            if let Some(editor) = app.binding_editor.as_mut() {
                // Allow typing to filter autocomplete
                editor.key.push(c);
                let key_clone = editor.key.clone();
                app.update_autocomplete(&key_clone);
            }
        }
        KeyCode::Backspace => {
            if let Some(editor) = app.binding_editor.as_mut() {
                editor.key.pop();
                let key_clone = editor.key.clone();
                app.update_autocomplete(&key_clone);
            }
        }
        KeyCode::Down => {
            if app.show_autocomplete {
                app.next_autocomplete();
            }
        }
        KeyCode::Up => {
            if app.show_autocomplete {
                app.prev_autocomplete();
            }
        }
        KeyCode::Right => {
            // Accept autocomplete
            if let Some(suggestion) = app.select_autocomplete() {
                if let Some(editor) = app.binding_editor.as_mut() {
                    editor.key = suggestion.value;
                }
                app.show_autocomplete = false;
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_description_field(app: &mut App, key: KeyCode) -> Result<()> {
    let editor = app.binding_editor.as_mut().unwrap();

    match key {
        KeyCode::Char(c) => {
            editor.description.push(c);
        }
        KeyCode::Backspace => {
            editor.description.pop();
        }
        _ => {}
    }

    Ok(())
}

fn handle_actions_field(app: &mut App, key: KeyCode) -> Result<()> {
    let editor = app.binding_editor.as_mut().unwrap();

    match key {
        KeyCode::Char('a') => {
            editor.start_adding_action();
        }
        KeyCode::Char('e') => {
            editor.start_editing_action();
        }
        KeyCode::Char('d') => {
            editor.delete_selected_action();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if editor.selected_action > 0 {
                editor.selected_action -= 1;
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            if editor.selected_action < editor.actions.len().saturating_sub(1) {
                editor.selected_action += 1;
            }
        }
        KeyCode::Char('K') => {
            editor.move_action_up();
        }
        KeyCode::Char('J') => {
            editor.move_action_down();
        }
        _ => {}
    }

    Ok(())
}

fn spawn_app_discovery(tx: Sender<Vec<DiscoveredApp>>) {
    thread::spawn(move || {
        let apps = app_discovery::discover_all_apps();
        let _ = tx.send(apps);
    });
}

fn save_to_karabiner(app: &mut App) -> Result<()> {
    // Validate and update config (resolve bundle IDs for apps without them)
    let warning_count = match validation::validate_and_update_config(&mut app.config) {
        Ok(warnings) => warnings.len(),
        Err(e) => {
            app.set_status(format!("Validation error: {}", e));
            return Err(e);
        }
    };

    // Save our config
    save_config(&app.config)?;

    // Install scripts
    let scripts_dir = install_scripts()?;

    // Apply to karabiner
    apply_to_karabiner(&app.config, &scripts_dir.to_string_lossy())?;

    // Show status with warning count if any
    if warning_count > 0 {
        app.set_status(format!("Saved ({} bundle IDs updated)", warning_count));
    } else {
        app.set_status("Saved to karabiner.json (backup created)");
    }
    Ok(())
}
