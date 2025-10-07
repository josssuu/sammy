use crate::cli::check::CheckArgs;
use crate::cli::traits::runnable::Runnable;
use crate::cli::update::UpdateArgs;
use crate::cli::woof::WoofArgs;
use clap::Parser;

mod check;
pub mod traits;
mod update;
mod woof;

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

    #[command(about = "Update repositories")]
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
