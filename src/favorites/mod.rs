use crate::Run;
use clap::{Args, Subcommand};
mod add;
mod remove;
use add::AddArgs;

use remove::RemoveArgs;
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
            config::FavoriteConfig::read_favorite_config()?.render_table();
        } else {
            self.subcommand.as_mut().unwrap().run()?;
        }
        Ok(())
    }
}
