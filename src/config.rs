//
// Copyright (C) 2025, 2026 by Martin Daser
//

use crate::Theme;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub theme: Theme,
}

pub fn resolve_config() -> Config {
    let mut config = Config {
        theme: Theme::default(),
    };

    if let Ok(parsed_config) = parse_user_config() {
        config.theme = parsed_config.theme;
    }

    config
}

fn parse_user_config() -> Result<Config> {
    if let Some(config_dir) = dirs::config_dir() {
        let conf_path = config_dir.join("FastSSH");
        let config_file = conf_path.join("config.yaml");

        fs::create_dir_all(&conf_path)?;

        if !config_file.exists() {
            fs::write(&config_file, DEFAULT_CONFIG)?;
        }

        let config_file_text = fs::read_to_string(&config_file)?;

        let conf = match serde_yaml::from_str::<Option<Config>>(&config_file_text) {
            Ok(Some(conf)) => conf,
            Ok(None) => {
                println!("Error parsing config file");
                std::process::exit(1);
            }
            Err(e) => {
                println!(
                    "Error parsing config file, make sure format is valid :\n  {}",
                    e
                );
                std::process::exit(1);
            }
        };

        return Ok(conf);
    }

    println!("Error while getting config directory");
    std::process::exit(1);
}

const DEFAULT_CONFIG: &str = "
# This is the default configuration for FastSSH.

theme:
    border: \"#ff0000\"
    primary_fg: \"#000080\"
    primary_bg: \"#d0d0d0\"
    secondary_fg: \"#0000ff\"
    secondary_bg: \"#c0c0c0\"
    title_fg: \"#000070\"
    title_bg: \"#d0d0d0\"
    select_fg: \"#0000ff\"
    select_active: \"#000000\"
    select_bg: \"#c0c0c0\"
";
