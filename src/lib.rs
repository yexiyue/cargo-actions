mod init;
pub mod utils;
use clap::Parser;
use init::InitArgs;
use login::login;
pub mod git;
mod login;
pub mod logs;
mod path_configs;

pub trait Run {
    fn run(&mut self) -> anyhow::Result<()>;
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

    /// Login with github
    Login,
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
            Self::Login => login(),
        }
    }
}
