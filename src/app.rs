use crate::config::{Action, Binding, Browser, Config, UrlMatchType};

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
    pub fn all() -> &'static [ActionType] {
        &[ActionType::App, ActionType::Url, ActionType::Shell]
    }

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
            match_type: UrlMatchType::Domain,
            browser: None,
            field: ActionEditorField::Type,
        }
    }

    pub fn from_action(action: &Action) -> Self {
        match action {
            Action::App { target } => Self {
                action_type: ActionType::App,
                target: target.clone(),
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
                match_type: match_type.clone(),
                browser: browser.clone(),
                field: ActionEditorField::Type,
            },
            Action::Shell { command } => Self {
                action_type: ActionType::Shell,
                target: command.clone(),
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
            self.actions.swap(self.selected_action, self.selected_action - 1);
            self.selected_action -= 1;
        }
    }

    pub fn move_action_down(&mut self) {
        if self.selected_action < self.actions.len().saturating_sub(1) {
            self.actions.swap(self.selected_action, self.selected_action + 1);
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
    pub autocomplete_suggestions: Vec<String>,
    pub autocomplete_selected: usize,
    pub show_autocomplete: bool,
    
    // App discovery state
    pub discovered_apps: Vec<String>,
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
        self.autocomplete_suggestions = autocomplete(partial)
            .into_iter()
            .map(|s| s.to_string())
            .take(10)
            .collect();
        self.autocomplete_selected = 0;
        self.show_autocomplete = !self.autocomplete_suggestions.is_empty() && !partial.is_empty();
    }

    pub fn update_app_autocomplete(&mut self, partial: &str) {
        if partial.is_empty() {
            self.autocomplete_suggestions = self.discovered_apps.clone();
        } else {
            let lower = partial.to_lowercase();
            self.autocomplete_suggestions = self.discovered_apps
                .iter()
                .filter(|app| app.to_lowercase().contains(&lower))
                .cloned()
                .take(10)
                .collect();
        }
        self.autocomplete_selected = 0;
        self.show_autocomplete = !self.autocomplete_suggestions.is_empty();
    }
    
    pub fn start_app_discovery(&mut self) {
        self.apps_loading = true;
        self.discovered_apps.clear();
    }
    
    pub fn finish_app_discovery(&mut self, apps: Vec<String>) {
        self.discovered_apps = apps;
        self.apps_loading = false;
    }

    pub fn select_autocomplete(&mut self) -> Option<String> {
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

    /// Returns true if current context is text input mode (not command/navigation mode)
    pub fn is_input_mode(&self) -> bool {
        if let Some(editor) = &self.binding_editor {
            if let Some(action_editor) = &editor.action_editor {
                // In action editor: only Target field is INPUT mode
                action_editor.field == ActionEditorField::Target
            } else {
                // In binding editor: Key and Description fields are INPUT mode
                matches!(editor.field, EditorField::Key | EditorField::Description)
            }
        } else {
            // Not editing, always NAV mode
            false
        }
    }

    /// Returns mode label for display
    pub fn mode_label(&self) -> &'static str {
        if self.is_input_mode() {
            "INPUT"
        } else {
            "NAV"
        }
    }
}
