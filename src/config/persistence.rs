use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::model::Config;

const APP_NAME: &str = "karabiner-rcmd-binder";
const CONFIG_FILE: &str = "config.toml";

pub fn config_dir() -> Result<PathBuf> {
    // Use $HOME/.config on all platforms for consistency
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".config").join(APP_NAME))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE))
}

pub fn scripts_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("scripts"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config from {:?}", path))?;
    let config: Config = toml::from_str(&content).with_context(|| "Failed to parse config TOML")?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("Failed to create config dir {:?}", dir))?;

    let path = config_path()?;
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(&path, content).with_context(|| format!("Failed to write config to {:?}", path))?;
    Ok(())
}

pub fn ensure_scripts_dir() -> Result<PathBuf> {
    let dir = scripts_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("Failed to create scripts dir {:?}", dir))?;
    Ok(dir)
}
