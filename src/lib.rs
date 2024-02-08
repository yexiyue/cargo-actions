mod config;
mod favorites;
mod init;
pub mod utils;
use clap::Parser;
use favorites::FavoriteArgs;
use init::InitArgs;
pub mod git;
pub mod logs;
mod path_configs;

pub trait Run {
    fn run(&mut self) -> anyhow::Result<()>;
}
pub trait Asker {
    fn ask(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(author, version, about, name = "cargo actions", bin_name = "cargo")]
pub enum CargoAction {
    #[command(subcommand, name = "actions", alias = "act")]
    Actions(ActionsArgs),
}

#[derive(Debug, Parser)]
pub enum ActionsArgs {
    /// Init a github actions workflow
    Init(InitArgs),

    /// Add a favorite command
    #[command(alias = "fav")]
    Favorite(FavoriteArgs),
}

impl Run for CargoAction {
    fn run(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Actions(action) => action.run(),
        }
    }
}

impl Run for ActionsArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Init(init) => init.run(),
            Self::Favorite(args) => args.run(),
        }
    }
}
