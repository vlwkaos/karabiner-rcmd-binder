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
        // Cycle through the app's windows on repeat presses (first press launches/focuses).
        // Only honored when this is the binding's sole action and bundle_id is present.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        cycle_windows: bool,
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
            Action::App { target, bundle_id, cycle_windows } => {
                let bundle_mark = if bundle_id.is_some() { " ✓" } else { "" }; // Checkmark shows bundle ID present
                let cycle_mark = if *cycle_windows { " ↻win" } else { "" }; // Window cycling enabled
                format!("{}{}{}", target, bundle_mark, cycle_mark)
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CenterMouseMode {
    #[default]
    Off,
    Always,
    MultiMonitorOnly,
}

impl CenterMouseMode {
    pub fn is_off(&self) -> bool {
        matches!(self, CenterMouseMode::Off)
    }

    pub fn cycle(&self) -> Self {
        match self {
            CenterMouseMode::Off => CenterMouseMode::Always,
            CenterMouseMode::Always => CenterMouseMode::MultiMonitorOnly,
            CenterMouseMode::MultiMonitorOnly => CenterMouseMode::Off,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CenterMouseMode::Off => "OFF",
            CenterMouseMode::Always => "ON",
            CenterMouseMode::MultiMonitorOnly => "MULTI ONLY",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CenterMouseMode::Off => "off",
            CenterMouseMode::Always => "always",
            CenterMouseMode::MultiMonitorOnly => "multi_monitor_only",
        }
    }
}

impl serde::Serialize for CenterMouseMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for CenterMouseMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CenterMouseMode;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "bool or string (off|always|multi_monitor_only)")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(if v { CenterMouseMode::Always } else { CenterMouseMode::Off })
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "off" => Ok(CenterMouseMode::Off),
                    "always" => Ok(CenterMouseMode::Always),
                    "multi_monitor_only" => Ok(CenterMouseMode::MultiMonitorOnly),
                    _ => Err(serde::de::Error::unknown_variant(
                        v,
                        &["off", "always", "multi_monitor_only"],
                    )),
                }
            }
        }
        d.deserialize_any(Visitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub anchor_key: AnchorKey,
    #[serde(default)]
    pub default_browser: Browser,
    #[serde(default, skip_serializing_if = "CenterMouseMode::is_off")]
    pub center_mouse: CenterMouseMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            anchor_key: AnchorKey::default(),
            default_browser: Browser::Firefox,
            center_mouse: CenterMouseMode::Off,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_mouse_mode_serde_roundtrip() {
        let modes = [
            (CenterMouseMode::Off, "\"off\""),
            (CenterMouseMode::Always, "\"always\""),
            (CenterMouseMode::MultiMonitorOnly, "\"multi_monitor_only\""),
        ];
        for (mode, expected_json) in &modes {
            let serialized = serde_json::to_string(mode).unwrap();
            assert_eq!(&serialized, expected_json);
            let deserialized: CenterMouseMode = serde_json::from_str(&serialized).unwrap();
            assert_eq!(&deserialized, mode);
        }
    }

    #[test]
    fn test_center_mouse_mode_deserialize_legacy_bool() {
        let from_true: CenterMouseMode = serde_json::from_str("true").unwrap();
        assert_eq!(from_true, CenterMouseMode::Always);

        let from_false: CenterMouseMode = serde_json::from_str("false").unwrap();
        assert_eq!(from_false, CenterMouseMode::Off);
    }

    #[test]
    fn test_settings_skips_off_on_serialize() {
        let settings = Settings::default();
        let toml = toml::to_string(&settings).unwrap();
        assert!(!toml.contains("center_mouse"), "Off should be omitted from serialized output");
    }

    #[test]
    fn test_settings_deserialize_legacy_center_mouse_true() {
        let toml = "center_mouse = true\n";
        let settings: Settings = toml::from_str(toml).unwrap();
        assert_eq!(settings.center_mouse, CenterMouseMode::Always);
    }

    // Action has no PartialEq, so match on fields rather than asserting whole-enum equality.

    #[test]
    fn test_app_cycle_windows_defaults_false_when_missing() {
        let json = r#"{"type":"app","target":"Terminal","bundle_id":"com.apple.Terminal"}"#;
        let action: Action = serde_json::from_str(json).unwrap();
        match action {
            Action::App { cycle_windows, .. } => assert!(!cycle_windows, "missing field must default false"),
            _ => panic!("expected Action::App"),
        }
    }

    // Deserialize of cycle_windows:true is covered by test_app_cycle_windows_true_round_trips.

    #[test]
    fn test_app_cycle_windows_false_is_omitted_on_serialize() {
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
            cycle_windows: false,
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(!json.contains("cycle_windows"), "false must be omitted");
    }

    #[test]
    fn test_app_cycle_windows_true_is_included_on_serialize() {
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
            cycle_windows: true,
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"cycle_windows\":true"), "true must be present");
    }

    #[test]
    fn test_app_cycle_windows_true_round_trips() {
        // Serialize-then-deserialize must preserve cycle_windows=true.
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
            cycle_windows: true,
        };
        let json = serde_json::to_string(&action).unwrap();
        let back: Action = serde_json::from_str(&json).unwrap();
        match back {
            Action::App { target, bundle_id, cycle_windows } => {
                assert_eq!(target, "Terminal");
                assert_eq!(bundle_id.as_deref(), Some("com.apple.Terminal"));
                assert!(cycle_windows);
            }
            _ => panic!("expected Action::App"),
        }
    }

    #[test]
    fn test_display_summary_appends_win_glyph_when_cycling() {
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
            cycle_windows: true,
        };
        let summary = action.display_summary();
        assert!(summary.ends_with("↻win"), "cycling app summary must end with win glyph, got {summary:?}");
    }

    #[test]
    fn test_display_summary_no_win_glyph_when_not_cycling() {
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
            cycle_windows: false,
        };
        let summary = action.display_summary();
        assert!(!summary.contains("↻win"), "non-cycling summary must omit win glyph");
    }

    #[test]
    fn test_display_summary_no_win_glyph_without_bundle_id_even_if_cycling() {
        // Structurally valid but semantically wrong: cycle_windows=true but no bundle_id.
        // Model has no cross-field guard, so glyph still shows; this pins the current contract
        // so a future guard (dropping the glyph) becomes a deliberate, visible change.
        let action = Action::App {
            target: "My App".to_string(),
            bundle_id: None,
            cycle_windows: true,
        };
        let summary = action.display_summary();
        assert!(!summary.contains("✓"), "no bundle id means no checkmark");
        assert!(summary.ends_with("↻win"), "cycle_windows glyph is not gated on bundle_id in the model");
    }
}
