use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    trace!("So is there someone?");
    debug!("Test! 1, 2, 3, Test!");
    info!("Hello World!");
    warn!("What was that?!?");
    error!("Aaaaaaaarrgrgghhhhhh!");
}
