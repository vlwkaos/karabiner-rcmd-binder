use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredApp {
    pub name: String,           // Display name (e.g., "KakaoWork")
    pub bundle_id: String,      // Full bundle ID (e.g., "com.kakaoenterprise.macos.kakaowork")
    pub last_component: String, // Last part of bundle ID (e.g., "kakaowork")
    #[serde(default = "default_timestamp")]
    pub last_seen: i64,         // Unix timestamp (seconds since epoch)
}

fn default_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

impl DiscoveredApp {
    fn new(name: String, bundle_id: String) -> Self {
        let last_component = extract_last_component(&bundle_id);
        let last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        Self {
            name,
            bundle_id,
            last_component,
            last_seen,
        }
    }
}

/// Extract last component from bundle ID
/// e.g., "com.kakaoenterprise.macos.kakaowork" -> "kakaowork"
fn extract_last_component(bundle_id: &str) -> String {
    bundle_id.split('.').last().unwrap_or(bundle_id).to_string()
}

/// Extract parent domain from bundle ID for disambiguation
/// e.g., "com.kakaoenterprise.macos.kakaowork" -> "macos.kakaowork"
pub fn extract_parent_component(bundle_id: &str) -> String {
    let parts: Vec<&str> = bundle_id.split('.').collect();
    if parts.len() >= 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        bundle_id.to_string()
    }
}

/// Discover running GUI applications via osascript with bundle IDs
pub fn discover_running_apps() -> Vec<DiscoveredApp> {
    let script = r#"
        tell application "System Events"
            set appList to {}
            repeat with proc in (every process whose background only is false)
                try
                    set appName to name of proc
                    set appBundle to bundle identifier of proc
                    set end of appList to appName & "|" & appBundle
                end try
            end repeat
            return appList
        end tell
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout
                .split(',')
                .filter_map(|entry| {
                    let entry = entry.trim();
                    if let Some((name, bundle_id)) = entry.split_once('|') {
                        let name = name.trim().to_string();
                        let bundle_id = bundle_id.trim().to_string();
                        if !name.is_empty() && !bundle_id.is_empty() {
                            return Some(DiscoveredApp::new(name, bundle_id));
                        }
                    }
                    None
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Discover installed applications by scanning common directories with bundle IDs
/// Optimized: Limits to 150 apps and skips /System/Applications for performance
pub fn discover_installed_apps() -> Vec<DiscoveredApp> {
    let mut apps = Vec::new();
    let paths = [
        "/Applications",
        &format!("{}/Applications", std::env::var("HOME").unwrap_or_default()),
        // Skip /System/Applications - it's slow and users rarely need those apps
    ];

    for path in &paths {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                // Limit total apps for performance
                if apps.len() >= 150 {
                    break;
                }

                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".app") {
                        let app_path = entry.path();
                        let plist_path = app_path.join("Contents/Info.plist");

                        // Try to read bundle ID from Info.plist
                        if let Ok(bundle_id) = read_bundle_id_from_plist(&plist_path) {
                            let app_name = name.trim_end_matches(".app").to_string();
                            apps.push(DiscoveredApp::new(app_name, bundle_id));
                        }
                    }
                }
            }
        }
    }

    apps
}

/// Read CFBundleIdentifier from Info.plist using plutil
fn read_bundle_id_from_plist(plist_path: &std::path::Path) -> Result<String, std::io::Error> {
    let output = Command::new("plutil")
        .args(&["-extract", "CFBundleIdentifier", "raw", "-o", "-"])
        .arg(plist_path)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to extract bundle ID",
        ));
    }

    let bundle_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if bundle_id.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Empty bundle ID",
        ))
    } else {
        Ok(bundle_id)
    }
}

/// Discover all apps: running first, then installed (deduplicated by bundle ID)
pub fn discover_all_apps() -> Vec<DiscoveredApp> {
    let running = discover_running_apps();
    let installed = discover_installed_apps();

    let mut seen_bundles: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    // Add running apps first (prefer running over installed)
    for app in running {
        if seen_bundles.insert(app.bundle_id.clone()) {
            result.push(app);
        }
    }

    // Add installed apps (skip if already running)
    for app in installed {
        if seen_bundles.insert(app.bundle_id.clone()) {
            result.push(app);
        }
    }

    // Sort by display name (case-insensitive)
    result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_running_apps() {
        let apps = discover_running_apps();
        // Should return something on macOS, empty vec on error
        println!("Running apps: {} found", apps.len());
        for app in apps.iter().take(5) {
            println!("  {} -> {}", app.name, app.bundle_id);
        }
    }

    #[test]
    fn test_discover_installed_apps() {
        let apps = discover_installed_apps();
        assert!(!apps.is_empty(), "Should find at least some apps");
        println!("Found {} installed apps", apps.len());
        for app in apps.iter().take(5) {
            println!("  {} -> {}", app.name, app.bundle_id);
        }
    }

    #[test]
    fn test_discover_all_apps() {
        let apps = discover_all_apps();
        assert!(!apps.is_empty(), "Should find apps");
        // Check no duplicates by bundle ID
        let mut bundle_seen = HashSet::new();
        for app in &apps {
            assert!(
                bundle_seen.insert(app.bundle_id.clone()),
                "Duplicate bundle ID: {} ({})",
                app.bundle_id,
                app.name
            );
        }
        println!("Found {} unique apps (by bundle ID)", apps.len());
    }

    #[test]
    fn test_extract_last_component() {
        assert_eq!(extract_last_component("com.apple.Safari"), "Safari");
        assert_eq!(
            extract_last_component("com.kakaoenterprise.macos.kakaowork"),
            "kakaowork"
        );
        assert_eq!(extract_last_component("singleword"), "singleword");
    }

    #[test]
    fn test_extract_parent_component() {
        assert_eq!(extract_parent_component("com.apple.Safari"), "apple.Safari");
        assert_eq!(
            extract_parent_component("com.kakaoenterprise.macos.kakaowork"),
            "macos.kakaowork"
        );
        assert_eq!(extract_parent_component("singleword"), "singleword");
    }
}
