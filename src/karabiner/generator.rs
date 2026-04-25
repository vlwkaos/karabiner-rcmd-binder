use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::fs;

use crate::config::{Action, AnchorKey, Binding, Browser, CenterMouseMode, Config};
use crate::karabiner::backup::{create_backup, karabiner_config_path};

const RULE_PREFIX: &str = "[rcmdb]";
const VAR_PREFIX: &str = "rcmdb_";
// ^ Runtime path: $HOME expands when karabiner executes the shell command, not at save time.
const SCRIPTS_RUNTIME_DIR: &str = "$HOME/.config/karabiner-rcmd-binder/scripts";

/// Generate Karabiner complex_modifications rules from our config
pub fn generate_rules(config: &Config) -> Vec<Value> {
    config
        .bindings
        .iter()
        .map(|b| generate_binding_rule(b, &config.settings.anchor_key, &config.settings.default_browser, config.settings.center_mouse))
        .collect()
}

/// Generate a single rule for a binding
fn generate_binding_rule(binding: &Binding, anchor_key: &AnchorKey, default_browser: &Browser, center_mouse: CenterMouseMode) -> Value {
    let description = if binding.description.is_empty() {
        format!("{} {}+{}", RULE_PREFIX, anchor_key.display_prefix(), binding.key)
    } else {
        format!(
            "{} {}+{}: {}",
            RULE_PREFIX, anchor_key.display_prefix(), binding.key, binding.description
        )
    };

    let manipulators = if binding.actions.len() <= 1 {
        // Single action - no cycling needed
        generate_single_action_manipulators(binding, anchor_key, default_browser, center_mouse)
    } else {
        // Multiple actions - cycling
        generate_cycling_manipulators(binding, anchor_key, default_browser, center_mouse)
    };

    json!({
        "description": description,
        "manipulators": manipulators
    })
}

/// Generate manipulators for a single action (no cycling)
fn generate_single_action_manipulators(
    binding: &Binding,
    anchor_key: &AnchorKey,
    default_browser: &Browser,
    center_mouse: CenterMouseMode,
) -> Vec<Value> {
    let from = json!({
        "key_code": binding.key,
        "modifiers": {
            "mandatory": [anchor_key.as_karabiner_modifier()],
            "optional": ["caps_lock"]
        }
    });

    let to = if binding.actions.is_empty() {
        vec![]
    } else {
        vec![action_to_karabiner(&binding.actions[0], default_browser, center_mouse)]
    };

    vec![json!({
        "type": "basic",
        "from": from,
        "to": to
    })]
}

/// Generate manipulators for cycling through multiple actions
fn generate_cycling_manipulators(
    binding: &Binding,
    anchor_key: &AnchorKey,
    default_browser: &Browser,
    center_mouse: CenterMouseMode,
) -> Vec<Value> {
    let var_name = format!("{}{}_cycle", VAR_PREFIX, binding.key);
    let num_actions = binding.actions.len();

    let from = json!({
        "key_code": binding.key,
        "modifiers": {
            "mandatory": [anchor_key.as_karabiner_modifier()],
            "optional": ["caps_lock"]
        }
    });

    binding
        .actions
        .iter()
        .enumerate()
        .map(|(i, action)| {
            let next_value = (i + 1) % num_actions;
            let action_to = action_to_karabiner(action, default_browser, center_mouse);

            json!({
                "type": "basic",
                "from": from.clone(),
                "to": [
                    action_to,
                    {
                        "set_variable": {
                            "name": var_name,
                            "value": next_value
                        }
                    }
                ],
                "conditions": [{
                    "type": "variable_if",
                    "name": var_name,
                    "value": i
                }]
            })
        })
        .collect()
}

/// Convert our Action to Karabiner's to event
fn action_to_karabiner(action: &Action, default_browser: &Browser, center_mouse: CenterMouseMode) -> Value {
    match action {
        Action::App { target, bundle_id } => {
            let launch_cmd = match bundle_id {
                Some(id) if !id.is_empty() => match center_mouse {
                    CenterMouseMode::Off => format!("open -b {}", id),
                    // $HOME expands at shell runtime — not tied to the save-time user path
                    mode => format!(
                        "open -b {} && \"{}/center-mouse.sh\" '{}' '{}'",
                        id, SCRIPTS_RUNTIME_DIR, id, mode.as_str()
                    ),
                },
                _ => format!("open -a '{}'", target), // Fallback: no bundle ID, skip center_mouse
            };
            json!({
                "shell_command": launch_cmd
            })
        }
        Action::Url {
            target,
            match_type,
            browser,
        } => {
            let browser = browser.as_ref().unwrap_or(default_browser);
            json!({
                "shell_command": format!(
                    "\"{}/url-focus.sh\" '{}' '{}' '{}'",
                    SCRIPTS_RUNTIME_DIR,
                    target,
                    match_type.as_str(),
                    browser.as_str()
                )
            })
        }
        Action::Shell { command } => {
            json!({
                "shell_command": command
            })
        }
    }
}

/// Apply our rules to karabiner.json
/// This preserves existing rules and only replaces our [rcmdb] rules
pub fn apply_to_karabiner(config: &Config) -> Result<()> {
    let config_path = karabiner_config_path()?;

    // Load existing karabiner.json or create default
    let mut kara_config: Value = if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {:?}", config_path))?;
        serde_json::from_str(&content).with_context(|| "Failed to parse karabiner.json")?
    } else {
        json!({
            "global": {},
            "profiles": [{
                "name": "Default",
                "complex_modifications": {
                    "rules": []
                },
                "simple_modifications": []
            }]
        })
    };

    // Create backup before modifying
    create_backup()?;

    // Find the first profile (or create one)
    let profiles = kara_config
        .get_mut("profiles")
        .and_then(|p| p.as_array_mut())
        .context("karabiner.json missing profiles array")?;

    if profiles.is_empty() {
        profiles.push(json!({
            "name": "Default",
            "complex_modifications": {
                "rules": []
            },
            "simple_modifications": []
        }));
    }

    let profile = profiles.get_mut(0).context("No profile found")?;

    // Ensure complex_modifications exists
    if profile.get("complex_modifications").is_none() {
        profile
            .as_object_mut()
            .unwrap()
            .insert("complex_modifications".to_string(), json!({ "rules": [] }));
    }

    let complex_mods = profile
        .get_mut("complex_modifications")
        .and_then(|c| c.as_object_mut())
        .context("complex_modifications is not an object")?;

    // Ensure rules array exists
    if complex_mods.get("rules").is_none() {
        complex_mods.insert("rules".to_string(), json!([]));
    }

    let rules = complex_mods
        .get_mut("rules")
        .and_then(|r| r.as_array_mut())
        .context("rules is not an array")?;

    // Remove existing [rcmdb] rules
    rules.retain(|rule| {
        rule.get("description")
            .and_then(|d| d.as_str())
            .map(|d| !d.starts_with(RULE_PREFIX))
            .unwrap_or(true)
    });

    // Add our new rules
    let our_rules = generate_rules(config);
    for rule in our_rules {
        rules.push(rule);
    }

    // Write back to karabiner.json
    let output = serde_json::to_string_pretty(&kara_config)?;

    // Ensure parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&config_path, output)
        .with_context(|| format!("Failed to write {:?}", config_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Binding;

    #[test]
    fn test_single_action_rule() {
        let binding = Binding {
            key: "t".to_string(),
            description: "Terminal".to_string(),
            actions: vec![Action::App {
                target: "Terminal".to_string(),
                bundle_id: Some("com.apple.Terminal".to_string()),
            }],
        };

        let rule = generate_binding_rule(&binding, &AnchorKey::RightCommand, &Browser::Firefox, CenterMouseMode::Off);
        assert!(rule["description"].as_str().unwrap().contains("[rcmdb]"));
        assert_eq!(rule["manipulators"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_cycling_rule() {
        let binding = Binding {
            key: "t".to_string(),
            description: "Terminals".to_string(),
            actions: vec![
                Action::App {
                    target: "Terminal".to_string(),
                    bundle_id: Some("com.apple.Terminal".to_string()),
                },
                Action::App {
                    target: "iTerm".to_string(),
                    bundle_id: Some("com.googlecode.iterm2".to_string()),
                },
            ],
        };

        let rule = generate_binding_rule(&binding, &AnchorKey::RightCommand, &Browser::Firefox, CenterMouseMode::Off);
        let manipulators = rule["manipulators"].as_array().unwrap();
        assert_eq!(manipulators.len(), 2);

        // Check cycling variables
        assert!(manipulators[0]["to"][1]["set_variable"]["value"] == 1);
        assert!(manipulators[1]["to"][1]["set_variable"]["value"] == 0);
    }

    #[test]
    fn test_app_action_uses_runtime_home_path() {
        // The generated shell_command must embed the literal "$HOME" string so the shell
        // expands it at Karabiner runtime, not the absolute path baked at save time.
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
        };
        let cmd = action_to_karabiner(&action, &Browser::Firefox, CenterMouseMode::Always);
        let shell_cmd = cmd["shell_command"].as_str().unwrap();

        assert!(shell_cmd.contains("$HOME"), "must contain literal $HOME, not an absolute path");
        assert!(!shell_cmd.contains("/Users/"), "must not bake absolute user path at save time");
        assert!(!shell_cmd.contains("/home/"), "must not bake absolute user path at save time");
        // $HOME must be inside double-quotes so the shell expands it
        assert!(shell_cmd.contains("\"$HOME"), "must double-quote $HOME for shell expansion");
        assert!(shell_cmd.contains("'always'"), "always mode must pass mode arg to script");
    }

    #[test]
    fn test_multi_monitor_only_passes_mode_arg() {
        let action = Action::App {
            target: "Terminal".to_string(),
            bundle_id: Some("com.apple.Terminal".to_string()),
        };
        let cmd = action_to_karabiner(&action, &Browser::Firefox, CenterMouseMode::MultiMonitorOnly);
        let shell_cmd = cmd["shell_command"].as_str().unwrap();
        assert!(shell_cmd.contains("center-mouse.sh"), "must invoke center-mouse.sh");
        assert!(shell_cmd.contains("'multi_monitor_only'"), "must pass multi_monitor_only mode arg");
    }

    #[test]
    fn test_url_action_uses_runtime_home_path() {
        let action = Action::Url {
            target: "https://example.com".to_string(),
            match_type: crate::config::UrlMatchType::Domain,
            browser: None,
        };
        let cmd = action_to_karabiner(&action, &Browser::Firefox, CenterMouseMode::Off);
        let shell_cmd = cmd["shell_command"].as_str().unwrap();

        assert!(shell_cmd.contains("$HOME"), "must contain literal $HOME, not an absolute path");
        assert!(!shell_cmd.contains("/Users/"), "must not bake absolute user path at save time");
        assert!(shell_cmd.contains("\"$HOME"), "must double-quote $HOME for shell expansion");
    }
}
