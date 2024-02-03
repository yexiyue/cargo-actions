use cargo_actions::{error, CargoAction, Run};
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let indicatif_layer = tracing_indicatif::IndicatifLayer::new();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(indicatif_layer.get_stdout_writer())
                .without_time()
                .with_target(false)
                .with_level(false),
        )
        .with(indicatif_layer)
        .init();
    let cargo_action = CargoAction::parse();

    match cargo_action.run() {
        Ok(_) => {}
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
