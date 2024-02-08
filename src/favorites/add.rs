use clap::Args;
use dialoguer::theme::ColorfulTheme;

use crate::{error, favorites::config::FavoriteGit, info, success, Asker, Run};

use super::config::{Favorite, FavoriteConfig, FavoriteLocal};

#[derive(Debug, Args)]
pub struct AddArgs {
    /// git url of the favorite
    url: Option<String>,
    /// subpath of the favorite
    #[arg(short, long)]
    subpath: Option<String>,
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
        if self.subpath.is_none() && self.local == false {
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
        if self.local == false && self.subpath.is_none() {
            self.subpath = Some(
                dialoguer::Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the subpath")
                    .allow_empty(true)
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
            let git = FavoriteGit::new(self.url.as_ref().unwrap(), self.subpath.clone());
            let fav = git.select(self.copy)?;
            for i in fav {
                match favorites.add_favorite(Favorite::Git(i)) {
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
        }
    }
}
