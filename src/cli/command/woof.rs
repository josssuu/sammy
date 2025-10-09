use crate::cli::traits::runnable::Runnable;
use clap::Parser;

#[derive(Parser)]
pub struct WoofArgs {}

impl Runnable for WoofArgs {
    fn run(&self) {
        println!("woof");
    }
}
