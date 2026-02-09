use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Browser {
    Firefox,
    Chrome,
    Safari,
    Arc,
    Edge,
}

impl Default for Browser {
    fn default() -> Self {
        Browser::Firefox
    }
}

impl Browser {
    pub fn all() -> &'static [Browser] {
        &[
            Browser::Firefox,
            Browser::Chrome,
            Browser::Safari,
            Browser::Arc,
            Browser::Edge,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Browser::Firefox => "firefox",
            Browser::Chrome => "chrome",
            Browser::Safari => "safari",
            Browser::Arc => "arc",
            Browser::Edge => "edge",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Browser::Firefox => "Firefox",
            Browser::Chrome => "Chrome",
            Browser::Safari => "Safari",
            Browser::Arc => "Arc",
            Browser::Edge => "Edge",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AnchorKey {
    #[default]
    RightCommand,
    RightOption,
}

impl AnchorKey {
    pub fn all() -> &'static [AnchorKey] {
        &[AnchorKey::RightCommand, AnchorKey::RightOption]
    }

    pub fn as_karabiner_modifier(&self) -> &'static str {
        match self {
            AnchorKey::RightCommand => "right_command",
            AnchorKey::RightOption => "right_option",
        }
    }

    pub fn display_prefix(&self) -> &'static str {
        match self {
            AnchorKey::RightCommand => "rcmd",
            AnchorKey::RightOption => "ropt",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AnchorKey::RightCommand => "Right Command",
            AnchorKey::RightOption => "Right Option",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UrlMatchType {
    Exact,
    Domain,
    Path,
    Glob,
}

impl Default for UrlMatchType {
    fn default() -> Self {
        UrlMatchType::Domain
    }
}

impl UrlMatchType {
    pub fn all() -> &'static [UrlMatchType] {
        &[
            UrlMatchType::Exact,
            UrlMatchType::Domain,
            UrlMatchType::Path,
            UrlMatchType::Glob,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UrlMatchType::Exact => "exact",
            UrlMatchType::Domain => "domain",
            UrlMatchType::Path => "path",
            UrlMatchType::Glob => "glob",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    App {
        target: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bundle_id: Option<String>,
    },
    Url {
        target: String,
        #[serde(default)]
        match_type: UrlMatchType,
        #[serde(skip_serializing_if = "Option::is_none")]
        browser: Option<Browser>,
    },
    Shell {
        command: String,
    },
}

impl Action {
    pub fn display_summary(&self) -> String {
        match self {
            Action::App { target, bundle_id } => {
                if bundle_id.is_some() {
                    format!("{} ✓", target) // Checkmark shows bundle ID present
                } else {
                    target.clone()
                }
            }
            Action::Url {
                target, match_type, ..
            } => {
                format!("{} ({})", target, match_type.as_str())
            }
            Action::Shell { command } => {
                let truncated = if command.len() > 30 {
                    format!("{}...", &command[..27])
                } else {
                    command.clone()
                };
                format!("$ {}", truncated)
            }
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Action::App { .. } => "App",
            Action::Url { .. } => "URL",
            Action::Shell { .. } => "Shell",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub key: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub actions: Vec<Action>,
}

impl Binding {
    pub fn actions_summary(&self) -> String {
        if self.actions.is_empty() {
            return "(no actions)".to_string();
        }
        if self.actions.len() == 1 {
            return self.actions[0].display_summary();
        }
        // Cycling: show as A -> B -> C
        self.actions
            .iter()
            .map(|a| a.display_summary())
            .collect::<Vec<_>>()
            .join(" -> ")
    }

    pub fn display_key(&self, anchor_key: &AnchorKey) -> String {
        format!("{}+{}", anchor_key.display_prefix(), self.key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub anchor_key: AnchorKey,
    #[serde(default)]
    pub default_browser: Browser,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            anchor_key: AnchorKey::default(),
            default_browser: Browser::Firefox,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub bindings: Vec<Binding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cached_apps: Vec<crate::app_discovery::DiscoveredApp>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            bindings: Vec::new(),
            cached_apps: Vec::new(),
        }
    }
}
