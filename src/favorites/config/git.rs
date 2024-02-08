use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::builder::OsStr;
use fs_extra::file::CopyOptions;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;

use crate::{
    config::Config,
    git::GitUrl,
    info,
    path_configs::{PathConfig, PathConfigs},
};

use super::FavoriteMeta;
static TEMP: Lazy<tempfile::TempDir> = Lazy::new(|| tempfile::tempdir().unwrap());

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoriteGit {
    pub url: String,
    pub path: Option<String>,
    pub subpath: Option<String>,
    #[serde(flatten)]
    pub meta: FavoriteMeta,
}

impl FavoriteGit {
    pub fn new(url: &str, subpath: Option<String>) -> Self {
        Self {
            path: None,
            subpath,
            url: url.to_string(),
            meta: FavoriteMeta::default(),
        }
    }

    pub fn find_actions(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut path = TEMP.path().to_path_buf();
        let git_url: &GitUrl = &self.url.as_str().into();
        info!("clone git repo...");
        git_url.clone(&path)?;

        if self.subpath.is_some() {
            path = path.join(self.subpath.as_ref().unwrap());
        }

        let metadata = std::fs::metadata(&path)?;
        let mut cargo_actions: Vec<PathBuf> = vec![];
        if metadata.is_dir() {
            let path_str = &path.to_string_lossy().to_string();
            for entry in glob::glob(&format!("{}/**/cargo-action.json", path_str))? {
                let entry = entry?;
                cargo_actions.push(entry);
            }
        } else {
            if path.file_name() != Some(&OsStr::from("cargo-action.json")) {
                bail!("not found actions");
            }
            cargo_actions.push(path);
        }
        if cargo_actions.is_empty() {
            bail!("not found actions");
        }

        Ok(cargo_actions)
    }

    fn gen_favorite(
        &self,
        copy: bool,
        PathConfig(path, config): &PathConfig<'_>,
    ) -> anyhow::Result<Self> {
        let home = std::env::var("HOME")?;
        let cargo_actions_home = PathBuf::new().join(home).join(".cargo-actions");

        let mut favorite = self.clone();

        let mut origin_path = self.url.clone();

        let temp_dir = TEMP.path().as_os_str().to_string_lossy().to_string();
        let relative_path = path
            .as_os_str()
            .to_string_lossy()
            .to_string()
            .replace(&temp_dir, "")
            .replacen("/", "", 1);
        origin_path = format!("{}/{}", self.url, relative_path);
        favorite.subpath = Some(relative_path);

        // 根据源路径生成md5 哈希值作为id
        let digest = md5::compute(&origin_path);
        let md5 = format!("{:?}", digest);
        favorite.meta.set_id(&md5);

        // 如果是复制，就复制到cargo-actions目录下
        if copy {
            let parent = path.parent().unwrap().to_path_buf();
            let yaml_path = parent.join(&config.path);

            let new_path = cargo_actions_home
                .join(&md5)
                .join(yaml_path.file_name().unwrap());
            // 拷贝模版文件
            let options = CopyOptions::new().overwrite(true).skip_exist(false);
            fs_extra::dir::create_all(new_path.parent().unwrap(), false)?;
            fs_extra::file::copy(&yaml_path, &new_path, &options)?;
            // 更改配置文件的模版路径
            let mut new_config = config.clone();
            new_config.to_mut().path = yaml_path.file_name().unwrap().to_string_lossy().to_string();
            // 写入配置文件
            let config_path = cargo_actions_home.join(&md5).join("cargo-action.json");
            let file = fs::File::create(&config_path)?;
            serde_json::to_writer_pretty(&file, &new_config)?;

            favorite.path = Some(config_path.to_string_lossy().to_string());
        }

        favorite.meta.author = config.author.clone();
        let git_url = GitUrl::from(&self.url);
        favorite.meta.set_origin(&git_url.to_string());
        favorite.meta.set_describe(&config.description);
        Ok(favorite)
    }

    pub fn select(&self, copy: bool) -> anyhow::Result<Vec<Self>> {
        let actions = self.find_actions()?;
        let path_configs = PathConfigs::from(&actions)?;
        let mut res: Vec<Self> = vec![];
        if path_configs.len() == 1 {
            let favorite = self.gen_favorite(copy, &path_configs[0])?;
            res.push(favorite);
        } else {
            let selected: Vec<&PathConfig<'_>> =
                path_configs.multiselect("Select the action to add to the favorite")?;
            for item in selected {
                let favorite = self.gen_favorite(copy, item)?;
                res.push(favorite);
            }
        }
        Ok(res)
    }

    pub fn write(&self) -> anyhow::Result<()> {
        if self.path.is_some() {
            info!("🚀 Copy action from local");
            let path = PathBuf::from(self.path.as_ref().unwrap());
            let config = Config::from(&path)?;
            config.write(&path.parent().as_ref().unwrap().to_path_buf())?;
        } else {
            info!("🚀 Downloading action from github...");
            let temp = tempdir()?;
            let git_url = GitUrl::from(&self.url);
            git_url.clone(&temp.path())?;

            let path = temp.path().join(&self.subpath.as_ref().unwrap());   
            let config = Config::from(&path)?;
            config.write(&path.parent().as_ref().unwrap().to_path_buf())?;
        }
        Ok(())
    }
}
