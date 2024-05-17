mod init;
pub mod utils;
use clap::Parser;
use init::InitArgs;
use login::login;
use mine::MineArgs;
use upload::UploadArgs;
pub mod git;
mod graphql;
mod login;
pub mod logs;
mod mine;
mod path_configs;
mod token;
mod upload;
mod client;

static CARGO_ACTIONS_URL: &str = "http://localhost:8000";
static CARGO_ACTIONS_FRONT_URL: &str = "http://localhost:5173";

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

    /// Upload a template to cargo actions
    Upload(UploadArgs),

    /// Init a github actions workflow from user's template
    Mine(MineArgs),
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
            Self::Upload(upload) => upload.run(),
            Self::Mine(mine) => mine.run(),
        }
    }
}
