use anyhow::Result;
use std::process::Command;

use crate::config::{Action, Config};

/// Validate and update config by resolving bundle IDs for App actions
pub fn validate_and_update_config(config: &mut Config) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    for binding in &mut config.bindings {
        for action in &mut binding.actions {
            if let Action::App { target, bundle_id } = action {
                // Skip if bundle ID already exists and is not empty
                if bundle_id.as_ref().map_or(false, |id| !id.is_empty()) {
                    continue;
                }

                // Try to resolve bundle ID from app name
                match try_resolve_bundle_id(target) {
                    Ok(resolved_id) => {
                        *bundle_id = Some(resolved_id.clone());
                        warnings.push(format!(
                            "Updated '{}' with bundle ID: {}",
                            target, resolved_id
                        ));
                    }
                    Err(_) => {
                        warnings.push(format!(
                            "Warning: Could not resolve bundle ID for '{}'. App may not launch correctly.", 
                            target
                        ));
                    }
                }
            }
        }
    }

    Ok(warnings)
}

/// Try to resolve bundle ID from app name using osascript
fn try_resolve_bundle_id(app_name: &str) -> Result<String> {
    let output = Command::new("osascript")
        .args(&["-e", &format!("id of application \"{}\"", app_name)])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Could not resolve bundle ID"));
    }

    let bundle_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if bundle_id.is_empty() {
        Err(anyhow::anyhow!("Empty bundle ID returned"))
    } else {
        Ok(bundle_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_bundle_id() {
        // Try to resolve Safari (should be installed on macOS)
        match try_resolve_bundle_id("Safari") {
            Ok(bundle_id) => {
                assert!(bundle_id.contains("safari"));
                println!("Safari bundle ID: {}", bundle_id);
            }
            Err(e) => {
                println!("Could not resolve Safari: {}", e);
            }
        }
    }
}
