use clap::Parser;
use log::{debug, error, info, trace, warn};

#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    env_logger::init();
    let _ = Cli::parse();

    trace!("So is there someone?");
    debug!("Test! 1, 2, 3, Test!");
    info!("Hello World!");
    warn!("What was that?!?");
    error!("Aaaaaaaarrgrgghhhhhh!");
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
