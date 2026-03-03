use crate::cli::Args;
use crate::cli::traits::runnable::Runnable;
use clap::Parser;
use log::LevelFilter;

mod cli;
mod config;
mod utils;

fn main() {
    configure_logger();
    let args = Args::parse();
    args.run();
}

fn configure_logger() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}

// todo - add strict clippy CI
// todo - get rid of all unwrap() calls
