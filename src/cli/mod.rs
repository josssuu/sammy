use traits::runnable::Runnable;
use clap::{command, Parser};
use command::check::CheckArgs;
use crate::cli::command::update::UpdateArgs;
use crate::cli::command::woof::WoofArgs;

pub mod traits;
mod command;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    cmd: MainCommand,
}

#[derive(Parser)]
enum MainCommand {
    #[command(about = "Check repositories' states")]
    Check(CheckArgs),

    #[command(about = "Update repositories. Does not allow updating if changes are present.\n  1. Fetch\n  2. Checkout\n  3. Pull")]
    Update(UpdateArgs),

    #[command(about = "Talk to Sammy")]
    Woof(WoofArgs),
}

impl Runnable for Args {
    fn run(&self) {
        match &self.cmd {
            MainCommand::Check(args) => args.run(),
            MainCommand::Update(args) => args.run(),
            MainCommand::Woof(args) => args.run(),
        }
    }
}
