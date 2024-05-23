mod init;
use clap::Parser;
use init::InitArgs;
use login::login;
use upload::UploadArgs;
mod client;
mod favorite;
pub mod git;
mod graphql;
mod login;
pub mod logs;
mod mine;
mod path_configs;
mod token;
mod upload;

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

/// Represents different actions that can be performed by the application.
#[derive(Debug, Parser)]
pub enum ActionsArgs {
    /// Initializes a new GitHub Actions workflow in your project.
    Init(InitArgs),

    /// Authenticates the user with GitHub.
    Login,

    /// Uploads a custom template to Cargo Actions for future use.
    Upload(UploadArgs),

    /// Initializes a GitHub Actions workflow based on a template saved by the user.
    Mine,

    /// Initializes a GitHub Actions workflow using a template from the user's favorites.
    Favorite,
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
            Self::Mine => mine::run(),
            Self::Favorite => favorite::run(),
        }
    }
}
