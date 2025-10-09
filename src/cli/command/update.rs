use std::io::{self, Write};
use crate::cli::traits::runnable::Runnable;
use clap::Parser;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use crate::cli::command::Reportable;
use crate::config::{load_config, Config};
use crate::utils;

#[derive(Parser)]
pub struct UpdateArgs {
    #[arg(long, short, help = "Filter branches to update")]
    filter: Option<String>,
    #[arg(long, short, help = "Stays on current branch after update (pun intended)")]
    stay: bool,
    #[arg(long, short, help = "Override branch to update")]
    branch: Option<String>,
}


struct UpdateResult {
    repository_name: String,
    target_branch: String,
    status: UpdateStatus,
}

enum UpdateStatus {
    Success,
    PendingChanges,
    UnableToFetch,
    UnableToCheckout,
    UnableToPull,
}

impl Runnable for UpdateArgs {
    fn run(&self) {
        let config = load_config().unwrap_or_else(|| {
            println!("Config not loaded, using default values");
            Config {
                projects: Default::default(),
            }
        });

        let repos = utils::collect_repos(&self.filter);

        if repos.is_empty() {
            println!("No repositories found");
            return;
        }

        let confirmation = {
            let names = repos.iter().map(|r| r.name()).collect::<Vec<_>>();
            println!("Are you sure you want to update {} repositories: {names:?}", names.len());
            ask_confirmation()
        };

        if let Confirmation::No = confirmation {
            println!("Update cancelled");
            return
        }

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut handles = vec![];

        // todo - add confirmation for repos
        for repository in repos {
            let repository_name = repository.name();
            let current_branch = repository.get_current_branch();
            let target_branch = self.branch.clone()
                .unwrap_or(config.get_target_branch(&repository_name));
            let stay = self.stay;

            // todo - add tracing (simple printing gets messy with async)
            let handle = runtime.spawn( async move {
                let status =
                    if repository.has_pending_changes().expect("Unable to check repository status") {
                        UpdateStatus::PendingChanges
                    } else if repository.fetch().is_err() {
                        UpdateStatus::UnableToFetch
                    } else if repository.checkout(&target_branch).is_err() {
                        UpdateStatus::UnableToCheckout
                    } else if repository.pull().is_err() {
                        UpdateStatus::UnableToPull
                    } else {
                        UpdateStatus::Success
                    };

                if stay {
                    repository.checkout(&current_branch).expect("Unable to checkout current branch");
                }

                UpdateResult {
                    repository_name,
                    target_branch,
                    status,
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let project_status = runtime.block_on(handle).unwrap();

            project_status.display()
                .expect(&format!("Failed to print update result for {}", project_status.repository_name));
        }
    }
}

enum Confirmation { Yes, No }

fn ask_confirmation() -> Confirmation {
    use std::io::{stdin,stdout,Write};

    for _ in 0..3 {
        let mut s=String::new();
        print!("Confirm (yes/no): ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        let confirmation = match s.to_ascii_lowercase().as_str() {
            "y" | "yes" => Some(Confirmation::Yes),
            "n" | "no" => Some(Confirmation::No),
            _ => None
        };

        if let Some(confirmation) = confirmation {
            return confirmation;
        }
    }

    panic!("Just say yes or no")
}

impl Reportable for UpdateResult {
    fn display(&self) -> io::Result<()> {
        let (status_message, color) = match self.status {
            UpdateStatus::Success => ("updated successfully".to_string(), Color::Green),
            UpdateStatus::PendingChanges => ("pending changes".to_string(), Color::Yellow),
            UpdateStatus::UnableToFetch => ("unable to fetch".to_string(), Color::Red),
            UpdateStatus::UnableToCheckout => (format!("unable to checkout '{}'", self.target_branch), Color::Red),
            UpdateStatus::UnableToPull => ("pending changes".to_string(), Color::Red),
        };

        let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = buffer_writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut buffer, "{:<35}", self.repository_name)?;
        write!(&mut buffer, "{:<10}", self.target_branch)?;
        write!(&mut buffer, "| ")?;

        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(&mut buffer, "{}\n", status_message)?;

        buffer_writer.print(&buffer)
    }
}
