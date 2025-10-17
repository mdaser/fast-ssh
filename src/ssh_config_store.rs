use crate::database::FileDatabase;
use anyhow::{format_err, Result};
use ssh_cfg::{SshConfig, SshConfigParser, SshHostConfig};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::PathBuf;

trait ConfigComments {
    fn get_comments(
        &self,
    ) -> (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    );
}

impl ConfigComments for SshConfig {
    fn get_comments(
        &self,
    ) -> (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ) {
        let mut comments = HashMap::new();
        let mut tags = HashMap::new();
        let mut tabs = HashMap::new();

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let config_path = PathBuf::from(home).join(".ssh").join("config");

        if let Ok(contents) = read_to_string(config_path) {
            let mut current_comment = Vec::new();
            let mut current_tags = Vec::new();
            let mut current_group: Option<String> = None;

            for line in contents.lines() {
                let trimmed = line.trim();

                if trimmed.starts_with('#') {
                    let comment_text = trimmed[1..].trim().to_string();
                    if comment_text.starts_with("tags:") {
                        let tags_line = comment_text["tags:".len()..].trim();
                        current_tags = tags_line.split(',').map(|s| s.trim().to_string()).collect();
                    } else if comment_text.starts_with("tab:") {
                        let tab_line = comment_text["tab:".len()..].trim();
                        current_group = Some(tab_line.trim().to_string());
                    } else {
                        current_comment.push(comment_text);
                    }
                } else if trimmed.starts_with("Host ") {
                    let host = trimmed["Host ".len()..].trim().to_string();
                    if !current_comment.is_empty() {
                        comments.insert(host.clone(), current_comment.join("\n"));
                        current_comment.clear();
                    }
                    if !current_tags.is_empty() {
                        tags.insert(host.clone(), current_tags.join(", "));
                        current_tags.clear();
                    }
                    if !current_group.is_none() {
                        tabs.insert(host, current_group.clone().unwrap());
                        current_group = None;
                    }
                } else if trimmed.is_empty() {
                    current_comment.clear();
                    current_tags.clear();
                    current_group = None;
                }
            }
        }

        (comments, tags, tabs)
    }
}

#[derive(Debug)]
pub struct SshGroupItem {
    pub name: String,
    pub full_name: String,
    pub connection_count: i64,
    pub last_used: i64,
    pub host_config: SshHostConfig,
    pub comment: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug)]
pub struct SshGroup {
    pub name: String,
    pub items: Vec<SshGroupItem>,
}

#[derive(Debug)]
pub struct SshConfigStore {
    pub config: SshConfig,
    pub groups: Vec<SshGroup>,
}

impl SshConfigStore {
    pub async fn new(db: &FileDatabase) -> Result<SshConfigStore> {
        let ssh_config = SshConfigParser::parse_home().await?;

        let (comments, tags, tabs) = ssh_config.get_comments();

        let mut scs = SshConfigStore {
            config: ssh_config,
            groups: Vec::new(),
        };

        scs.create_ssh_groups(db, &comments, &tags, &tabs);

        if scs.groups.is_empty() {
            return Err(format_err!("Your configuration file contains no entries (or only wildcards) ! Please add at least one."));
        }

        Ok(scs)
    }

    fn create_ssh_groups(
        &mut self,
        db: &FileDatabase,
        comments: &std::collections::HashMap<String, String>,
        tags: &std::collections::HashMap<String, String>,
        tabs: &std::collections::HashMap<String, String>,
    ) {
        let mut groups: Vec<SshGroup> = vec![SshGroup {
            name: "Others".to_string(),
            items: Vec::new(),
        }];

        self.config.iter().for_each(|(host_short, value)| {
            let host_entry = db.get_host_values(host_short).unwrap();

            if host_short.contains('*') {
                return;
            }

            if tabs.contains_key(host_short) {
                let tab_name = tabs
                    .get(host_short)
                    .cloned()
                    .unwrap_or_else(|| "Others".to_string());
                let group = groups.iter_mut().find(|g| g.name == tab_name);

                let group_item = SshGroupItem {
                    name: host_short.to_string(),
                    connection_count: host_entry.connection_count,
                    last_used: host_entry.last_used_date,
                    full_name: host_short.to_string(),
                    host_config: value.clone(),
                    comment: comments.get(host_short).cloned(),
                    tags: tags.get(host_short).cloned(),
                };

                if group.is_none() {
                    groups.push(SshGroup {
                        name: tab_name.to_string(),
                        items: vec![group_item],
                    });

                    return;
                }

                let group = &mut group.unwrap().items;
                group.push(group_item);

                return;
            }

            groups[0].items.push(SshGroupItem {
                full_name: host_short.to_string(),
                connection_count: host_entry.connection_count,
                last_used: host_entry.last_used_date,
                name: host_short.to_string(),
                host_config: value.clone(),
                comment: comments.get(host_short).cloned(),
                tags: tags.get(host_short).cloned(),
            });
        });

        groups.reverse();
        self.groups = groups.into_iter().filter(|g| !g.items.is_empty()).collect();
    }
}
