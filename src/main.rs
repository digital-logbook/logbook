use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

use log::{debug, error, info, trace, warn};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<WarnLevel>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new entry
    Create {
        /// Overwrite an already created (but not yet finished) entry
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

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
