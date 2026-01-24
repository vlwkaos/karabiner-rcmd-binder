use crate::app_discovery::DiscoveredApp;
use crate::config::{Action, Binding, Browser, Config, UrlMatchType};

#[derive(Debug, Clone)]
pub struct AutocompleteSuggestion {
    pub display: String, // What user sees: "KakaoWork" or "Terminal (apple.terminal)"
    pub value: String,   // App name to store: "KakaoWork"
    pub bundle_id: String, // Bundle ID to store: "com.kakaoenterprise.macos.kakaowork"
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Bindings,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorField {
    Key,
    Description,
    Actions,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionEditorField {
    Type,
    Target,
    MatchType,
    Browser,
}

#[derive(Debug, Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
pub struct ActionEditor {
    pub action_type: ActionType,
    pub target: String,
    pub bundle_id: Option<String>, // For App actions
    pub match_type: UrlMatchType,
    pub browser: Option<Browser>,
    pub field: ActionEditorField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    App,
    Url,
    Shell,
}

impl ActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::App => "App",
            ActionType::Url => "URL",
            ActionType::Shell => "Shell",
        }
    }

    pub fn next(&self) -> ActionType {
        match self {
            ActionType::App => ActionType::Url,
            ActionType::Url => ActionType::Shell,
            ActionType::Shell => ActionType::App,
        }
    }

    pub fn prev(&self) -> ActionType {
        match self {
            ActionType::App => ActionType::Shell,
            ActionType::Url => ActionType::App,
            ActionType::Shell => ActionType::Url,
        }
    }
}

impl ActionEditor {
    pub fn new() -> Self {
        Self {
            action_type: ActionType::App,
            target: String::new(),
            bundle_id: None,
            match_type: UrlMatchType::Domain,
            browser: None,
            field: ActionEditorField::Type,
        }
    }

    pub fn from_action(action: &Action) -> Self {
        match action {
            Action::App { target, bundle_id } => Self {
                action_type: ActionType::App,
                target: target.clone(),
                bundle_id: bundle_id.clone(),
                match_type: UrlMatchType::Domain,
                browser: None,
                field: ActionEditorField::Type,
            },
            Action::Url {
                target,
                match_type,
                browser,
            } => Self {
                action_type: ActionType::Url,
                target: target.clone(),
                bundle_id: None,
                match_type: match_type.clone(),
                browser: browser.clone(),
                field: ActionEditorField::Type,
            },
            Action::Shell { command } => Self {
                action_type: ActionType::Shell,
                target: command.clone(),
                bundle_id: None,
                match_type: UrlMatchType::Domain,
                browser: None,
                field: ActionEditorField::Type,
            },
        }
    }

    pub fn to_action(&self) -> Action {
        match self.action_type {
            ActionType::App => Action::App {
                target: self.target.clone(),
                bundle_id: self.bundle_id.clone(),
            },
            ActionType::Url => Action::Url {
                target: self.target.clone(),
                match_type: self.match_type.clone(),
                browser: self.browser.clone(),
            },
            ActionType::Shell => Action::Shell {
                command: self.target.clone(),
            },
        }
    }

    pub fn next_field(&mut self) {
        self.field = match (&self.action_type, &self.field) {
            (_, ActionEditorField::Type) => ActionEditorField::Target,
            (ActionType::Url, ActionEditorField::Target) => ActionEditorField::MatchType,
            (ActionType::Url, ActionEditorField::MatchType) => ActionEditorField::Browser,
            (ActionType::Url, ActionEditorField::Browser) => ActionEditorField::Type,
            (_, ActionEditorField::Target) => ActionEditorField::Type,
            _ => ActionEditorField::Type,
        };
    }

    pub fn prev_field(&mut self) {
        self.field = match (&self.action_type, &self.field) {
            (_, ActionEditorField::Type) => {
                if self.action_type == ActionType::Url {
                    ActionEditorField::Browser
                } else {
                    ActionEditorField::Target
                }
            }
            (_, ActionEditorField::Target) => ActionEditorField::Type,
            (_, ActionEditorField::MatchType) => ActionEditorField::Target,
            (_, ActionEditorField::Browser) => ActionEditorField::MatchType,
        };
    }
}

#[derive(Debug, Clone)]
pub struct BindingEditor {
    pub key: String,
    pub description: String,
    pub actions: Vec<Action>,
    pub selected_action: usize,
    pub field: EditorField,
    pub action_editor: Option<ActionEditor>,
    pub editing_action_index: Option<usize>,
}

impl BindingEditor {
    pub fn new() -> Self {
        Self {
            key: String::new(),
            description: String::new(),
            actions: Vec::new(),
            selected_action: 0,
            field: EditorField::Key,
            action_editor: None,
            editing_action_index: None,
        }
    }

    pub fn from_binding(binding: &Binding) -> Self {
        Self {
            key: binding.key.clone(),
            description: binding.description.clone(),
            actions: binding.actions.clone(),
            selected_action: 0,
            field: EditorField::Key,
            action_editor: None,
            editing_action_index: None,
        }
    }

    pub fn to_binding(&self) -> Binding {
        Binding {
            key: self.key.clone(),
            description: self.description.clone(),
            actions: self.actions.clone(),
        }
    }

    pub fn next_field(&mut self) {
        self.field = match self.field {
            EditorField::Key => EditorField::Description,
            EditorField::Description => EditorField::Actions,
            EditorField::Actions => EditorField::Key,
        };
    }

    pub fn prev_field(&mut self) {
        self.field = match self.field {
            EditorField::Key => EditorField::Actions,
            EditorField::Description => EditorField::Key,
            EditorField::Actions => EditorField::Description,
        };
    }

    pub fn start_adding_action(&mut self) {
        self.action_editor = Some(ActionEditor::new());
        self.editing_action_index = None;
    }

    pub fn start_editing_action(&mut self) {
        if let Some(action) = self.actions.get(self.selected_action) {
            self.action_editor = Some(ActionEditor::from_action(action));
            self.editing_action_index = Some(self.selected_action);
        }
    }

    pub fn finish_action_edit(&mut self) {
        if let Some(editor) = self.action_editor.take() {
            let action = editor.to_action();
            if let Some(idx) = self.editing_action_index.take() {
                self.actions[idx] = action;
            } else {
                self.actions.push(action);
                self.selected_action = self.actions.len() - 1;
            }
        }
    }

    pub fn cancel_action_edit(&mut self) {
        self.action_editor = None;
        self.editing_action_index = None;
    }

    pub fn delete_selected_action(&mut self) {
        if !self.actions.is_empty() {
            self.actions.remove(self.selected_action);
            if self.selected_action >= self.actions.len() && !self.actions.is_empty() {
                self.selected_action = self.actions.len() - 1;
            }
        }
    }

    pub fn move_action_up(&mut self) {
        if self.selected_action > 0 {
            self.actions
                .swap(self.selected_action, self.selected_action - 1);
            self.selected_action -= 1;
        }
    }

    pub fn move_action_down(&mut self) {
        if self.selected_action < self.actions.len().saturating_sub(1) {
            self.actions
                .swap(self.selected_action, self.selected_action + 1);
            self.selected_action += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsField {
    DefaultBrowser,
}

pub struct App {
    pub config: Config,
    pub tab: Tab,
    pub input_mode: InputMode,
    pub should_quit: bool,
    pub status_message: Option<String>,

    // Bindings tab state
    pub selected_binding: usize,
    pub binding_editor: Option<BindingEditor>,
    pub editing_binding_index: Option<usize>,

    // Settings tab state
    pub settings_field: SettingsField,

    // Autocomplete state
    pub autocomplete_suggestions: Vec<AutocompleteSuggestion>,
    pub autocomplete_selected: usize,
    pub show_autocomplete: bool,

    // App discovery state
    pub discovered_apps: Vec<DiscoveredApp>,
    pub apps_loading: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tab: Tab::Bindings,
            input_mode: InputMode::Normal,
            should_quit: false,
            status_message: None,
            selected_binding: 0,
            binding_editor: None,
            editing_binding_index: None,
            settings_field: SettingsField::DefaultBrowser,
            autocomplete_suggestions: Vec::new(),
            autocomplete_selected: 0,
            show_autocomplete: false,
            discovered_apps: Vec::new(),
            apps_loading: false,
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Bindings => Tab::Settings,
            Tab::Settings => Tab::Bindings,
        };
    }

    pub fn prev_tab(&mut self) {
        self.next_tab(); // Only 2 tabs, same as next
    }

    // Bindings list navigation
    pub fn next_binding(&mut self) {
        if !self.config.bindings.is_empty() {
            self.selected_binding = (self.selected_binding + 1) % self.config.bindings.len();
        }
    }

    pub fn prev_binding(&mut self) {
        if !self.config.bindings.is_empty() {
            self.selected_binding = self
                .selected_binding
                .checked_sub(1)
                .unwrap_or(self.config.bindings.len() - 1);
        }
    }

    pub fn start_new_binding(&mut self) {
        self.binding_editor = Some(BindingEditor::new());
        self.editing_binding_index = None;
        self.input_mode = InputMode::Editing;
    }

    pub fn start_edit_binding(&mut self) {
        if let Some(binding) = self.config.bindings.get(self.selected_binding) {
            self.binding_editor = Some(BindingEditor::from_binding(binding));
            self.editing_binding_index = Some(self.selected_binding);
            self.input_mode = InputMode::Editing;
        }
    }

    pub fn delete_binding(&mut self) {
        if !self.config.bindings.is_empty() {
            self.config.bindings.remove(self.selected_binding);
            if self.selected_binding >= self.config.bindings.len()
                && !self.config.bindings.is_empty()
            {
                self.selected_binding = self.config.bindings.len() - 1;
            }
        }
    }

    pub fn save_binding(&mut self) {
        if let Some(editor) = self.binding_editor.take() {
            // Validate key before saving
            use crate::keycodes::is_valid_key;
            if !is_valid_key(&editor.key) {
                self.set_status(format!(
                    "Invalid key: '{}'. Use autocomplete for valid keys.",
                    editor.key
                ));
                self.binding_editor = Some(editor); // Put editor back
                return;
            }

            let binding = editor.to_binding();
            if let Some(idx) = self.editing_binding_index.take() {
                self.config.bindings[idx] = binding;
            } else {
                self.config.bindings.push(binding);
                self.selected_binding = self.config.bindings.len() - 1;
            }
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn cancel_edit(&mut self) {
        self.binding_editor = None;
        self.editing_binding_index = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn update_autocomplete(&mut self, partial: &str) {
        use crate::keycodes::autocomplete;
        // For key codes, we use simple string suggestions (no bundle ID needed)
        self.autocomplete_suggestions = autocomplete(partial)
            .into_iter()
            .map(|s| AutocompleteSuggestion {
                display: s.to_string(),
                value: s.to_string(),
                bundle_id: String::new(), // No bundle ID for key codes
            })
            .take(10)
            .collect();
        self.autocomplete_selected = 0;
        self.show_autocomplete = !self.autocomplete_suggestions.is_empty() && !partial.is_empty();
    }

    pub fn update_app_autocomplete(&mut self, partial: &str) {
        use crate::app_discovery::extract_parent_component;

        let filtered_apps: Vec<&DiscoveredApp> = if partial.is_empty() {
            self.discovered_apps.iter().collect()
        } else {
            let lower = partial.to_lowercase();
            self.discovered_apps
                .iter()
                .filter(|app| {
                    app.name.to_lowercase().contains(&lower)
                        || app.last_component.to_lowercase().contains(&lower)
                })
                .take(10)
                .collect()
        };

        // Check for name conflicts (case-insensitive)
        let has_conflict = |name: &str| -> bool {
            self.discovered_apps
                .iter()
                .filter(|app| app.name.eq_ignore_ascii_case(name))
                .count()
                > 1
        };

        self.autocomplete_suggestions = filtered_apps
            .into_iter()
            .map(|app| {
                let display = if has_conflict(&app.name) {
                    // Show parent component for disambiguation: "Terminal (apple.terminal)"
                    format!(
                        "{} ({})",
                        app.name,
                        extract_parent_component(&app.bundle_id)
                    )
                } else {
                    app.name.clone()
                };

                AutocompleteSuggestion {
                    display,
                    value: app.name.clone(),
                    bundle_id: app.bundle_id.clone(),
                }
            })
            .collect();

        self.autocomplete_selected = 0;
        self.show_autocomplete = !self.autocomplete_suggestions.is_empty();
    }

    pub fn start_app_discovery(&mut self) {
        self.apps_loading = true;
        self.discovered_apps.clear();
    }

    pub fn finish_app_discovery(&mut self, apps: Vec<DiscoveredApp>) {
        self.discovered_apps = apps;
        self.apps_loading = false;
    }

    pub fn select_autocomplete(&mut self) -> Option<AutocompleteSuggestion> {
        if self.show_autocomplete {
            self.autocomplete_suggestions
                .get(self.autocomplete_selected)
                .cloned()
        } else {
            None
        }
    }

    pub fn next_autocomplete(&mut self) {
        if !self.autocomplete_suggestions.is_empty() {
            self.autocomplete_selected =
                (self.autocomplete_selected + 1) % self.autocomplete_suggestions.len();
        }
    }

    pub fn prev_autocomplete(&mut self) {
        if !self.autocomplete_suggestions.is_empty() {
            self.autocomplete_selected = self
                .autocomplete_selected
                .checked_sub(1)
                .unwrap_or(self.autocomplete_suggestions.len() - 1);
        }
    }

    // Settings navigation
    pub fn next_browser(&mut self) {
        let browsers = Browser::all();
        let current_idx = browsers
            .iter()
            .position(|b| b == &self.config.settings.default_browser)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % browsers.len();
        self.config.settings.default_browser = browsers[next_idx].clone();
    }

    pub fn prev_browser(&mut self) {
        let browsers = Browser::all();
        let current_idx = browsers
            .iter()
            .position(|b| b == &self.config.settings.default_browser)
            .unwrap_or(0);
        let prev_idx = current_idx.checked_sub(1).unwrap_or(browsers.len() - 1);
        self.config.settings.default_browser = browsers[prev_idx].clone();
    }
}
