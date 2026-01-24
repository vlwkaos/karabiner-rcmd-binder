use std::collections::HashSet;
use std::process::Command;

/// Discover running GUI applications via osascript
pub fn discover_running_apps() -> Vec<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of every process whose background only is false")
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => Vec::new(),
    }
}

/// Discover installed applications by scanning common directories
pub fn discover_installed_apps() -> Vec<String> {
    let mut apps = Vec::new();
    let paths = [
        "/Applications",
        "/System/Applications",
        &format!("{}/Applications", std::env::var("HOME").unwrap_or_default()),
    ];
    
    for path in &paths {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".app") {
                        let app_name = name.trim_end_matches(".app").to_string();
                        apps.push(app_name);
                    }
                }
            }
        }
    }
    
    apps
}

/// Discover all apps: running first, then installed (deduplicated, case-insensitive)
pub fn discover_all_apps() -> Vec<String> {
    let running = discover_running_apps();
    let installed = discover_installed_apps();
    
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    // Add running apps first
    for app in running {
        let lower = app.to_lowercase();
        if seen.insert(lower) {
            result.push(app);
        }
    }
    
    // Add installed apps (skip if already in running)
    for app in installed {
        let lower = app.to_lowercase();
        if seen.insert(lower) {
            result.push(app);
        }
    }
    
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_running_apps() {
        let apps = discover_running_apps();
        // Should return something on macOS, empty vec on error
        println!("Running apps: {:?}", apps);
    }

    #[test]
    fn test_discover_installed_apps() {
        let apps = discover_installed_apps();
        assert!(!apps.is_empty(), "Should find at least some apps");
        println!("Found {} installed apps", apps.len());
    }

    #[test]
    fn test_discover_all_apps() {
        let apps = discover_all_apps();
        assert!(!apps.is_empty(), "Should find apps");
        // Check no duplicates (case-insensitive)
        let mut lower_seen = HashSet::new();
        for app in &apps {
            assert!(lower_seen.insert(app.to_lowercase()), 
                    "Duplicate app (case-insensitive): {}", app);
        }
    }
}
