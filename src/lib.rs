mod init;
mod utils;
mod config;
use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use config::Config;

use crate::config::Runner;


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
    Init,
}

impl Run for CargoAction {
    fn run(&self) -> anyhow::Result<()> {
        Ok(self.subcommand.run()?)
    }
}

impl Run for Commands {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Init => {
                let config_file=fs::File::open("actions/test/config.json")?;
                let config:Config=serde_json::from_reader(config_file)?;
                config.write(Some("actions/test".parse()?))?;
                Ok(())
            }
        }
    }
}
