use crate::cli::Args;
use crate::cli::traits::runnable::Runnable;
use clap::Parser;

mod cli;
mod config;
mod utils;

fn main() {
    let args = Args::parse();
    args.run();
}

// todo - add strict clippy CI
// todo - get rid of all unwrap() calls
