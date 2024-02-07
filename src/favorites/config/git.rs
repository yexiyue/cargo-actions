use serde::{Deserialize, Serialize};

use super::FavoriteMeta;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FavoriteGit {
    pub url: String,
    pub path: Option<String>,
    pub subpath: String,
    #[serde(flatten)]
    pub meta: FavoriteMeta,
}
