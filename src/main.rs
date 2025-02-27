use log::error;
use rosenpass::{cli::Cli, sodium::sodium_init};
use std::process::exit;

/// Catches errors, prints them through the logger, then exits
pub fn main() {
    env_logger::init();
    match sodium_init().and_then(|()| Cli::run()) {
        Ok(_) => {}
        Err(e) => {
            error!("{e}");
            exit(1);
        }
    }
}
