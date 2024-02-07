use clap::Args;
use dialoguer::theme::ColorfulTheme;

use crate::{error, info, success, Asker, Run};

use super::config::{Favorite, FavoriteConfig, FavoriteLocal};

#[derive(Debug, Args)]
pub struct AddArgs {
    /// git url of the favorite
    url: Option<String>,
    /// Use local path
    #[arg(long, action=clap::ArgAction::SetTrue)]
    local: bool,
    /// Copy the favorite to the local
    #[arg(short, long,action=clap::ArgAction::SetTrue)]
    copy: bool,
}

impl Asker for AddArgs {
    fn ask(&mut self) -> anyhow::Result<()> {
        let home = std::env::var("HOME")?;
        if self.local == false {
            self.local = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .default(false)
                .with_prompt("Use local path?")
                .interact()?;
        }
        if self.url.is_none() {
            let prompt = match self.local {
                true => "Enter the local path",
                false => "Enter the git url",
            };
            self.url = Some(
                dialoguer::Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .interact()?,
            );
        }

        if self.copy == false {
            self.copy = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .default(false)
                .with_prompt(format!("Copy the favorite to the {home}/.cargo-actions?"))
                .interact()?;
        }
        Ok(())
    }
}

impl Run for AddArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        self.ask()?;
        self.add_favorites()
    }
}

impl AddArgs {
    fn add_favorites(&mut self) -> anyhow::Result<()> {
        let mut favorites = FavoriteConfig::read_favorite_config()?;

        if self.local {
            let local = FavoriteLocal::new(self.url.as_ref().unwrap());
            let fav = local.select(self.copy)?;
            for i in fav {
                match favorites.add_favorite(Favorite::Local(i)) {
                    Ok(name) => {
                        success!("add favorite success, id: {}", name);
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
            info!("\nplease run `cargo actions favorite --list` to see the result");
            Ok(())
        } else {
            // todo 复制到本地并替换path
            Ok(())
        }
    }
}
