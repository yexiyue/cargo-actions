use cargo_actions::{error, CargoAction, Run};
use clap::Parser;

fn main() {
    let mut cargo_action = CargoAction::parse();

    match cargo_action.run() {
        Ok(_) => {}
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
