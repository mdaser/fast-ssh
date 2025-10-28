//
// Copyright (C) 2025 by Martin Daser
//

use anyhow::{format_err, Context, Result};
use std::fs;
use tui::widgets::TableState;

use crate::{
    config::{resolve_config, Config},
    database::FileDatabase,
    searcher::Searcher,
    ssh_config_store::{SshConfigStore, SshGroup, SshGroupItem},
};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub enum ConfigDisplayMode {
    Global,
    Selected,
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Normal,
    Searching,
    SpawnSsh,
    Ping(String),
    Quit,
}

pub struct App {
    state: AppState,
    state_info: String,
    pub searcher: Searcher,
    pub selected_group: usize,
    pub host_state: TableState,
    pub scs: SshConfigStore,
    pub config_display_mode: ConfigDisplayMode,
    pub config_paragraph_offset: u16,
    pub db: FileDatabase,
    show_help: bool,
    _config: Config,
}

impl App {
    pub async fn new() -> Result<App> {
        let db = App::create_or_get_db_file()?;
        let scs = SshConfigStore::new(&db).await?;
        let config = resolve_config();

        Ok(App {
            state: AppState::Normal,
            state_info: String::from("Welcome to FastSSH!"),
            selected_group: 0,
            config_paragraph_offset: 0,
            scs,
            host_state: TableState::default(),
            config_display_mode: ConfigDisplayMode::Selected,
            db,
            searcher: Searcher::new(),
            show_help: false,
            _config: config,
        })
    }

    pub fn create_or_get_db_file() -> Result<FileDatabase> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| format_err!("Could not get config directory"))?;

        let conf_path = config_dir.join("FastSSH");
        let db_path = conf_path.join("db.ron");

        fs::create_dir_all(&conf_path)
            .with_context(|| format_err!("Could not create the config directory"))?;

        FileDatabase::new(db_path.to_str().unwrap())
    }

    pub fn get_selected_group(&self) -> &SshGroup {
        &self.scs.groups[self.selected_group]
    }

    pub fn get_selected_item(&self) -> Option<&SshGroupItem> {
        if let Some(host_state) = self.host_state.selected() {
            let items_len = self.get_items_based_on_mode().len();
            if host_state < items_len {
                Some(self.get_items_based_on_mode()[host_state])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_all_items(&self) -> Vec<&SshGroupItem> {
        self.scs
            .groups
            .iter()
            .flat_map(|group| &group.items)
            .collect::<Vec<&SshGroupItem>>()
    }

    pub fn get_items_based_on_mode(&self) -> Vec<&SshGroupItem> {
        let items: Vec<&SshGroupItem> = match self.state {
            AppState::Searching => self.searcher.get_filtered_items(self),
            _ => self
                .get_selected_group()
                .items
                .iter()
                .collect::<Vec<&SshGroupItem>>(),
        };

        items
    }

    pub fn change_selected_group(&mut self, rot_right: bool) {
        let actual_idx = self.selected_group;
        let items_len = self.scs.groups.len();

        self.selected_group = match rot_right {
            true => (actual_idx + 1) % items_len,
            false => (actual_idx + items_len - 1) % items_len,
        };
    }

    pub fn change_selected_item(&mut self, rot_right: bool) {
        let items_len = self.get_items_based_on_mode().len();

        if items_len == 0 {
            return;
        }

        let i = match self.host_state.selected() {
            Some(i) => {
                if rot_right {
                    (i + 1) % items_len
                } else {
                    (i + items_len - 1) % items_len
                }
            }
            None => 0,
        };
        self.host_state.select(Some(i));
    }

    pub fn scroll_config_paragraph(&mut self, offset: i64) {
        self.config_paragraph_offset = (self.config_paragraph_offset as i64 + offset).max(0) as u16;
    }

    pub fn toggle_config_display_mode(&mut self) {
        self.config_display_mode = match self.config_display_mode {
            ConfigDisplayMode::Global => ConfigDisplayMode::Selected,
            ConfigDisplayMode::Selected => ConfigDisplayMode::Global,
        };
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn set_state_info(&mut self, state_info: String) {
        self.state_info = state_info;
    }

    pub fn state_info(&self) -> &str {
        &self.state_info
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn app_name(&self) -> &str {
        "FastSSH"
    }

    pub fn app_version(&self) -> &str {
        VERSION.unwrap_or("unknown")
    }
}
