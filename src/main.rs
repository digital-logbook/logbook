use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

use log::info;

#[derive(Parser)]
#[command(version, about)]
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

fn create_entry(force: &bool) {
    if *force {
        info!("Deleting existing entry to allow creating a new one.");
    }
    info!("Creating new entry.");
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    match &cli.command {
        Some(Commands::Create { force }) => create_entry(force),
        None => {}
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
