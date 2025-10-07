use crate::cli::traits::runnable::Runnable;
use clap::Parser;

#[derive(Parser)]
pub struct UpdateArgs {}

impl Runnable for UpdateArgs {
    fn run(&self) {
        unimplemented!();
    }
}
