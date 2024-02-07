use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::builder::OsStr;

use fs_extra::file::CopyOptions;

use serde::{Deserialize, Serialize};

use super::FavoriteMeta;
use crate::path_configs::{PathConfig, PathConfigs};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoriteLocal {
    pub path: String,
    #[serde(flatten)]
    pub meta: FavoriteMeta,
}

impl FavoriteLocal {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.into(),
            meta: FavoriteMeta::default(),
        }
    }

    pub fn find_actions(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut path = PathBuf::new();
        if self.path.starts_with("/") {
            path = path.join(&self.path);
        } else {
            path = std::env::current_dir()?.join(&self.path);
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
        // 获取路径转为string
        let path_str = path.to_string_lossy().to_string();
        let mut favorite = self.clone();

        // 根据源路径生成md5 哈希值作为id
        let digest = md5::compute(&path_str);
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

            favorite.path = config_path.to_string_lossy().to_string();
        } else {
            favorite.path = path_str.clone();
        }
        
        favorite.meta.author = config.author.clone();
        favorite.meta.set_origin(&path_str);
        favorite.meta.set_describe(&config.description);
        Ok(favorite)
    }

    pub fn select(&self, copy: bool) -> anyhow::Result<Vec<FavoriteLocal>> {
        let actions = self.find_actions()?;
        let path_configs = PathConfigs::from(&actions)?;
        let mut res: Vec<FavoriteLocal> = vec![];
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
}
