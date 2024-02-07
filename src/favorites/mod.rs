use crate::{warn, Run};
use clap::{Args, Subcommand};
mod add;
mod remove;
use add::AddArgs;
use prettytable::row;
use remove::RemoveArgs;

use self::config::Favorite;
mod config;

#[derive(Debug, Subcommand)]
pub enum FavoriteCommand {
    /// add a favorite
    Add(AddArgs),
    /// remove a favorite
    Remove(RemoveArgs),
}

#[derive(Debug, Args)]
pub struct FavoriteArgs {
    /// list favorites
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    list: bool,

    #[command(subcommand)]
    subcommand: Option<FavoriteCommand>,
}

impl Run for FavoriteCommand {
    fn run(&mut self) -> anyhow::Result<()> {
        match self {
            FavoriteCommand::Add(add) => add.run(),
            FavoriteCommand::Remove(remove) => remove.run(),
        }
    }
}

impl Run for FavoriteArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        if self.list {
            // todo 使用表格打印
            let favorite_configs = config::FavoriteConfig::read_favorite_config()?;
            if favorite_configs.len() == 0 {
                warn!("There is no favorite, please add one");
            } else {
                let mut table = prettytable::Table::new();
            
                table.set_titles(row![FYc=>"ID", "Author","Origin", "Description"]);
                for item in favorite_configs.iter() {
                    match item {
                        Favorite::Git(git) => {
                            let mut origin = git.meta.origin.clone();
                            if origin.len() > 40 {
                                origin.insert_str(40, "\n");
                            };

                            let mut description = git.meta.description.clone();
                            if description.len() > 40 {
                                description.insert_str(40, "\n");
                            };

                            table.add_row(row![
                                Fcc->&git.meta.id,
                                Fmb->&git.meta.author.as_ref().unwrap_or(&"--".to_string()),
                                Fbl->&origin,
                                &description
                            ]);
                        }
                        Favorite::Local(local) => {
                            let mut origin = local.meta.origin.clone();
                            if origin.len() > 40 {
                                origin.insert_str(40, "\n");
                            };

                            let mut description = local.meta.description.clone();
                            if description.len() > 40 {
                                description.insert_str(40, "\n");
                            };

                            table.add_row(row![
                                Fcc->&local.meta.id,
                                Fmc->&local.meta.author.as_ref().unwrap_or(&"--".to_string()),
                                Fbl->&origin,
                                &description
                            ]);
                        }
                    }
                }
                table.printstd();
            }
        } else {
            self.subcommand.as_mut().unwrap().run()?;
        }
        Ok(())
    }
}
