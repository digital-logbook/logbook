use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

use crate::Controller;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<WarnLevel>,
}

pub struct CliController {
    cli: Cli,
}

impl Controller for CliController {
    fn process_events(&self) {
        self.cli = Cli::parse();
    }
}
