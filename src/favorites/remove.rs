use clap::Args;
use dialoguer::theme::ColorfulTheme;

use crate::{Asker, Run};

use super::config::FavoriteConfig;

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// name of the favorite
    name: Option<String>,
}

impl Asker for RemoveArgs {
    fn ask(&mut self) -> anyhow::Result<()> {
        let favorites = FavoriteConfig::read_favorite_config()?;
        // todo 重构，提供更详细的信息进行删除
        if self.name.is_none() {
            let options = &favorites.get_ids();
            let index = dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select the favorite you want to remove")
                .items(options)
                .interact()?;
            self.name = Some(options[index].to_string());
        }
        Ok(())
    }
}

impl Run for RemoveArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        self.ask()?;
        let mut favorites = FavoriteConfig::read_favorite_config()?;
        Ok(favorites.remove_favorite(self.name.as_ref().unwrap())?)
    }
}
