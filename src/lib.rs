mod config;
mod init;
mod utils;

use clap::{Parser, Subcommand};

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(author,version,about,long_about = None)]
pub struct CargoAction {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Init a github actions workflow
    Init {
        #[arg(default_value = "yexiyue/cargo-actions")]
        name: Option<String>,
    },
}

impl Run for CargoAction {
    fn run(&self) -> anyhow::Result<()> {
        Ok(self.subcommand.run()?)
    }
}

impl Run for Commands {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Init { name } => init::init(name.clone()),
        }
    }
}
