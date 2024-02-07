use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

pub use self::{git::FavoriteGit, local::FavoriteLocal};
mod git;
mod local;

/// favorite
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Favorite {
    Git(FavoriteGit),
    Local(FavoriteLocal),
}

impl Favorite {
    pub fn get_id(&self) -> &str {
        match self {
            Self::Git(git) => &git.meta.id,
            Self::Local(local) => &local.meta.id,
        }
    }
}

/// favorite meta
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FavoriteMeta {
    pub id: String,
    pub origin: String,
    pub description: String,
    pub author: Option<String>,
}

impl FavoriteMeta {
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_string();
        self
    }
    pub fn set_origin(&mut self, origin: &str) -> &mut Self {
        self.origin = origin.to_string();
        self
    }
    pub fn set_describe(&mut self, describe: &str) -> &mut Self {
        self.description = describe.to_string();
        self
    }
}

/// favorite config
#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteConfig {
    favorites: Vec<Favorite>,
}

impl Deref for FavoriteConfig {
    type Target = Vec<Favorite>;

    fn deref(&self) -> &Self::Target {
        &self.favorites
    }
}
impl DerefMut for FavoriteConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.favorites
    }
}

impl FavoriteConfig {
    /// read favorite config
    pub fn read_favorite_config() -> Result<Self> {
        let home = std::env::var("HOME")?;
        let config_path = PathBuf::new()
            .join(home)
            .join(".cargo-actions/favorite.json");

        match fs::metadata(&config_path) {
            Ok(_) => {
                let file = fs::File::open(config_path)?;
                let config: Self = serde_json::from_reader(file)?;
                Ok(config)
            }
            Err(_) => {
                let config = FavoriteConfig { favorites: vec![] };
                Ok(config)
            }
        }
    }

    /// write favorite config
    pub fn write_favorite_config(&self) -> Result<()> {
        let home = std::env::var("HOME")?;
        let config_path = PathBuf::new()
            .join(home)
            .join(".cargo-actions/favorite.json");
        fs_extra::dir::create_all(config_path.parent().unwrap(), false)?;
        let file = fs::File::create(config_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn has_id(&self, name: &str) -> bool {
        self.iter().any(|favorite| favorite.get_id() == name)
    }

    pub fn add_favorite(&mut self, favorite: Favorite) -> Result<String> {
        let id = favorite.get_id().to_string();
        if self.has_id(favorite.get_id()) {
            anyhow::bail!("The favorite {} already exists", &id);
        }
        self.push(favorite);
        Ok(id)
    }

    pub fn remove_favorite(&mut self, name: &str) -> Result<()> {
        let index = self
            .iter()
            .position(|favorite| favorite.get_id() == name)
            .ok_or_else(|| anyhow::format_err!("favorite not found"))?;
        self.remove(index);
        Ok(())
    }

    pub fn get_ids(&self) -> Vec<&str> {
        self.iter().map(|favorite| favorite.get_id()).collect()
    }
}

impl Drop for FavoriteConfig {
    fn drop(&mut self) {
        self.write_favorite_config().unwrap();
    }
}
