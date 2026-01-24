use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

const BACKUP_PREFIX: &str = "karabiner.json.switchkey-backup-";
const MAX_BACKUPS: usize = 3;

pub fn karabiner_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".config/karabiner/karabiner.json"))
}

/// Create a backup of karabiner.json with timestamp
/// Keeps only the last MAX_BACKUPS backups
pub fn create_backup() -> Result<Option<PathBuf>> {
    let config_path = karabiner_config_path()?;
    if !config_path.exists() {
        return Ok(None);
    }

    let config_dir = config_path
        .parent()
        .context("Could not determine karabiner config directory")?;

    // Generate backup filename with timestamp
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let backup_name = format!("{}{}", BACKUP_PREFIX, timestamp);
    let backup_path = config_dir.join(&backup_name);

    // Copy current config to backup
    fs::copy(&config_path, &backup_path)
        .with_context(|| format!("Failed to create backup at {:?}", backup_path))?;

    // Clean up old backups
    cleanup_old_backups(config_dir)?;

    Ok(Some(backup_path))
}

/// Remove old backups, keeping only MAX_BACKUPS most recent
fn cleanup_old_backups(config_dir: &Path) -> Result<()> {
    let mut backups: Vec<PathBuf> = fs::read_dir(config_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(BACKUP_PREFIX))
                .unwrap_or(false)
        })
        .collect();

    // Sort by name (timestamp ensures chronological order)
    backups.sort();

    // Remove oldest backups if we have more than MAX_BACKUPS
    while backups.len() > MAX_BACKUPS {
        if let Some(oldest) = backups.first() {
            fs::remove_file(oldest)
                .with_context(|| format!("Failed to remove old backup {:?}", oldest))?;
            backups.remove(0);
        }
    }

    Ok(())
}

/// List existing backups
pub fn list_backups() -> Result<Vec<PathBuf>> {
    let config_path = karabiner_config_path()?;
    let config_dir = config_path
        .parent()
        .context("Could not determine karabiner config directory")?;

    if !config_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups: Vec<PathBuf> = fs::read_dir(config_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(BACKUP_PREFIX))
                .unwrap_or(false)
        })
        .collect();

    backups.sort();
    backups.reverse(); // Most recent first
    Ok(backups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karabiner_path() {
        let path = karabiner_config_path().unwrap();
        assert!(path.ends_with(".config/karabiner/karabiner.json"));
    }
}
