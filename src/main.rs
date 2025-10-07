use crate::cli::Args;
use crate::cli::traits::runnable::Runnable;
use clap::Parser;

mod cli;

fn main() {
    let args = Args::parse();
    args.run();
}

// todo - add strict clippy CI
// todo - get rid of all unwrap() calls
