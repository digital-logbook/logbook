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

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    match &cli.command {
        Some(Commands::Create { force }) => {
            if *force {
                info!("Force creating a new entry.");
            } else {
                info!("Creating a new entry.");
            }
        }
        None => {}
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
