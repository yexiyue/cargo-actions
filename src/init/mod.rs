use anyhow::anyhow;
use clap::Args;
use std::{env::current_dir, ffi::OsStr};
use tempfile::tempdir;
use walkdir;
mod use_template_id;

use crate::{
    // favorites::config::FavoriteConfig,
    git::GitUrl,
    info,
    init::use_template_id::use_template_id,
    path_configs::{PathConfigs, SelectPathConfig, WritePath},
    success,
    warn,
    Run,
};

#[derive(Debug, Args)]
pub struct InitArgs {
    ///[URL]: The github name or url of the action
    ///[ID]: The id of the action
    #[arg(default_value = "yexiyue/cargo-actions", name = "URL|ID")]
    url: Option<String>,

    /// The subpath of the action
    #[arg(short, long)]
    subpath: Option<String>,

    /// use favorites
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    favorite: bool,

    /// use the template id of the action
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    id: bool,
}

impl Run for InitArgs {
    // todo 添加本地favorite 支持
    fn run(&mut self) -> anyhow::Result<()> {
        if self.favorite == false {
            if !self.id {
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
                        cargo_actions.push(
                            entry
                                .path()
                                .parent()
                                .ok_or(anyhow!("cargo-action.json parent not found"))?
                                .to_path_buf(),
                        );
                    }
                }

                let path_configs = PathConfigs::from(&cargo_actions)?;

                if path_configs.len() == 0 {
                    return Err(anyhow!("No cargo action found in {}", &url.to_string()));
                } else {
                    info!("🎉 Actions downloaded successfully");
                }
                if path_configs.len() == 1 {
                    let config = &path_configs[0];
                    info!("⚙️ Action title: {}", config.config.name);
                    if config.config.description.is_some() {
                        info!(
                            "⚙️ Action description: {}",
                            config.config.description.as_ref().unwrap()
                        );
                    }
                    let default_write_path = current_dir()?
                        .join(".github/workflows")
                        .join(format!("{}.yaml", config.config.name))
                        .to_string_lossy()
                        .to_string();
                    let write_path = WritePath::input_and_confirm(default_write_path)?;
                    config.write_template(write_path)?;

                    success!("write success");

                    if config.config.success_message.is_some() {
                        info!("{}", config.config.success_message.as_ref().unwrap());
                    }
                } else {
                    let select_config = SelectPathConfig::asker()
                        .action_config(&path_configs, 0)
                        .finish();
                    select_config.write_template()?;
                }
            } else {
                use_template_id(self.url.as_ref().unwrap())?;
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}
