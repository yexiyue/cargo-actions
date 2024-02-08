use anyhow::anyhow;
use clap::Args;
use dialoguer::theme::ColorfulTheme;
use std::ffi::OsStr;
use tempfile::tempdir;
use walkdir;

use crate::{
    favorites::config::FavoriteConfig,
    git::GitUrl,
    info,
    path_configs::{PathConfig, PathConfigs},
    Run,
};

#[derive(Debug, Args)]
pub struct InitArgs {
    /// The github name or url of the action
    #[arg(default_value = "yexiyue/cargo-actions")]
    url: Option<String>,

    /// The subpath of the action
    #[arg(short, long)]
    subpath: Option<String>,
    
    /// use favorites
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    favorite: bool,
}

impl Run for InitArgs {
    // todo 添加本地favorite 支持
    fn run(&mut self) -> anyhow::Result<()> {
        if self.favorite == false {
            info!("🚀 Downloading actions from github...");

            let git_name = self.url.as_ref().unwrap();
            let url: GitUrl = git_name.as_str().into();
            let dir = tempdir()?;
            match url.clone(dir.path()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(anyhow!("Please check if the Git user / repository exists. \n Failed to download actions from github: {}", e));
                }
            }
            let d_path = match &self.subpath {
                Some(subpath) => dir.path().join(subpath),
                None => dir.path().to_path_buf(),
            };
            // 获取所有cargo-action.json文件
            let entries = walkdir::WalkDir::new(d_path);
            let mut cargo_actions = vec![];
            for entry in entries.into_iter().filter_map(|e| e.ok()) {
                if entry.file_name() == OsStr::new("cargo-action.json") {
                    cargo_actions.push(entry.path().to_path_buf());
                }
            }
            let path_configs = PathConfigs::from(&cargo_actions)?;

            if path_configs.len() == 0 {
                return Err(anyhow!("No cargo action found in {}", &url.to_string()));
            } else {
                info!("🎉 Actions downloaded successfully");
            }
            if path_configs.len() == 1 {
                let PathConfig(action_path, config) = &path_configs[0];
                info!("⚙️ Action title: {}", config.description);
                config.write(&action_path.parent().unwrap().to_path_buf())?;
                Ok(())
            } else {
                let PathConfig(action_path, config) = path_configs.select("Select an action")?;
                config.write(&action_path.parent().unwrap().to_path_buf())?;
                Ok(())
            }
        } else {
            let favorites = FavoriteConfig::read_favorite_config()?;
            favorites.render_table();
            let options = &favorites.get_ids();
            let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select the favorite you want to remove")
                .items(options)
                .interact()?;
            let favorite = favorites.get(index).unwrap();
            favorite.write()?;
            Ok(())
        }
    }
}
