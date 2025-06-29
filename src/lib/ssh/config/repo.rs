use std::{
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use toml::Table;

use crate::{ssh::git::Repo, ssh::vars::*};
use dirs::home_dir;

#[derive(Serialize, Deserialize)]
pub struct RepoConfig {
    pub name: String,
    pub public: bool,
    pub members: Vec<String>,
    pub failed_push_message: Option<String>,
    pub web_template: Option<String>,
    pub extra: Option<Table>,
}

fn _get_config_file_path() {
    if let Some(config_file) = dirs::home_dir() {
        let config_file_path = home_dir().expect("REASON").join(&REPO_CONFIG_FILE);
        println!("Full path to config file: {:?}", config_file_path);

        // You can then use `config_file_path` to read or write to the file.
        // For example, to check if it exists:
        if config_file_path.exists() {
            println!("Config file exists!");
        } else {
            println!("Config file does not exist.");
        }
    } else {
        eprintln!("Error: Could not determine home directory.");
    }
}

pub async fn load_repo_config(repo_path: &Path) -> anyhow::Result<RepoConfig> {
    let config_file_path = home_dir().expect("REASON").join(&REPO_CONFIG_FILE);
    println!("Full path to config file: {:?}", config_file_path);

    if config_file_path.exists() {
        println!("Config file exists!");
    } else {
        println!("Config file does not exist.");
    }

    let temp_dir = tempdir()?;
    let clone_dir = temp_dir.path().join(repo_path);
    Repo::clone(repo_path, &clone_dir).await?;

    let text =
        read_to_string(clone_dir.join(&config_file_path)).context("Couldn't read repo.toml")?;
    Ok(toml::from_str(&text)?)
}

pub async fn new_repo_config(repo_path: &Path, username: &str) -> anyhow::Result<()> {
    let config_name = PathBuf::from(REPO_CONFIG_FILE);

    let temp_dir = tempdir()?;
    let clone_dir = temp_dir.path().join(repo_path);
    let repo = Repo::clone(repo_path, &clone_dir).await?;

    let config = RepoConfig {
        name: repo_path.to_str().unwrap().to_string(),
        public: false,
        members: vec![username.to_string()],
        failed_push_message: None,
        extra: None,
        web_template: None,
    };

    let text = toml::to_string(&config)?;
    write(clone_dir.join(config_name), text).context("Could not write default repo config")?;
    repo.push_changes("chore: create repo config").await?;
    Ok(())
}
