use cargo_actions::{CargoAction, Run};
use clap::Parser;

fn main() {
    let cargo_action = CargoAction::parse();
    cargo_action.run().unwrap();
}
