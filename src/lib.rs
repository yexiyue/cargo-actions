mod config;
mod init;
pub mod utils;

use clap::Parser;

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(author, version, about, name = "cargo actions", bin_name = "cargo")]
pub enum CargoAction {
    #[command(subcommand, name = "actions")]
    Actions(Commands),
}

#[derive(Debug, Parser)]
pub enum Commands {
    /// Init a github actions workflow
    Init {
        #[arg(default_value = "yexiyue/cargo-actions")]
        name: Option<String>,
    },
}

impl Run for CargoAction {
    fn run(&self) -> anyhow::Result<()> {
        match &self {
            Self::Actions(action) => action.run(),
        }
    }
}

impl Run for Commands {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Init { name } => init::init(name.clone()),
        }
    }
}
