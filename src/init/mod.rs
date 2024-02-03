use anyhow::{anyhow, Context};
use dialoguer::theme;
use std::{ffi::OsStr, fs::File};
use tempfile::tempdir;
use walkdir;

use crate::{config::Config, git::GitUrl, info};

pub fn init(name: Option<String>) -> anyhow::Result<()> {
    info!("🚀 Downloading actions from github...");

    let git_name = name.unwrap();
    let url: GitUrl = git_name.as_str().into();
    let dir = tempdir()?;
    match url.clone(dir.path()) {
        Ok(_) => {
            info!("🎉 Actions downloaded successfully");
        }
        Err(e) => {
            return Err(anyhow!("Failed to download actions from github: {}", e));
        }
    }

    // 获取所有cargo-action.json文件
    let entries = walkdir::WalkDir::new(dir.path());
    let mut cargo_actions = vec![];
    for entry in entries.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == OsStr::new("cargo-action.json") {
            cargo_actions.push(entry.path().to_path_buf());
        }
    }
    let mut configs: Vec<Config> = vec![];

    // 解析所有cargo-action.json文件
    for path in &cargo_actions {
        let file = File::open(path).with_context(|| format!("Failed to open {:?}", path))?;
        let config = serde_json::from_reader(file)?;
        configs.push(config);
    }

    let result = cargo_actions.iter().zip(configs.iter()).collect::<Vec<_>>();

    if configs.len() == 0 {
        return Err(anyhow!("No cargo action found in {}", &url.to_string()));
    }

    let choice = dialoguer::Select::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&configs)
        .default(0)
        .interact()?;
    let (action_path, config) = result[choice];
    let wirte_path = action_path.clone();
    config.write(&wirte_path.parent().unwrap().to_path_buf())?;
    Ok(())
}
