use crate::cli::traits::runnable::Runnable;
use clap::Parser;
use std::io::{self, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use crate::cli::command::Reportable;
use crate::config::{load_config, Config};
use crate::utils;
use crate::utils::repository::Repository;

#[derive(Parser)]
pub struct CheckArgs {
    #[arg(long, short, help = "Filter repositories")]
    filter: Option<String>,
}

impl Runnable for CheckArgs {
    fn run(&self) {
        let config = load_config().unwrap_or_else(|| {
            println!("Config not loaded, using default values");
            Config {
                projects: Default::default(),
            }
        });

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let mut handles = vec![];

        let repos = utils::collect_repos(&self.filter);

        if repos.is_empty() {
            println!("No repositories found");
            return;
        }

        for repository in repos {
            let repository_name = repository.name();
            let current_branch = repository.get_current_branch();
            let target_branch = config.get_target_branch(&repository_name);

            // todo - add tracing (simple printing gets messy with async)
            let handle = runtime.spawn( async move {
                let branch_status = get_repository_status(&repository, &target_branch);
                ProjectStatus {
                    name: repository_name,
                    status: branch_status,
                    current_branch,
                    target_branch,
                }
            });

            handles.push(handle);
        }


        for handle in handles {
            let project_status = runtime.block_on(handle).unwrap();

            project_status.display()
                .expect("Failed to print branch status");
        }
    }
}

struct ProjectStatus {
    name: String,
    status: BranchStatus,
    current_branch: String,
    target_branch: String,
}

fn get_repository_status(repo: &Repository, branch: &str) -> BranchStatus {
    let Some(remote_head) = repo.get_remote_head(branch) else {
        return BranchStatus::RemoteNotFound
    };

    let Some(local_head) = repo.get_local_head(branch) else {
        return BranchStatus::LocalNotFound
    };

    if local_head == remote_head {
        BranchStatus::UpToDate
    }
    else if repo.is_local_ahead(branch).expect("Unable to check local branch status") {
        BranchStatus::LocalAhead
    } else {
        BranchStatus::UpdateAvailable
    }
}

enum BranchStatus {
    UpToDate,
    UpdateAvailable,
    LocalAhead,
    RemoteNotFound,
    LocalNotFound,
}

impl Reportable for ProjectStatus {
    fn display(&self) -> io::Result<()> {
        let (status_message, color) = match self.status {
            BranchStatus::UpToDate => ("up to date".to_string(), Color::Green),
            BranchStatus::UpdateAvailable => ("update available".to_string(), Color::Yellow),
            BranchStatus::LocalAhead => ("local is ahead".to_string(), Color::Magenta),
            BranchStatus::RemoteNotFound => (format!("remote '{}' branch not found", self.target_branch), Color::Red),
            BranchStatus::LocalNotFound => (format!("local '{}' branch not found", self.target_branch), Color::Red),
        };

        let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = buffer_writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut buffer, "{:<35}", self.name)?;
        write!(&mut buffer, "{:<10}", self.current_branch)?;
        write!(&mut buffer, "| ")?;

        buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(&mut buffer, "{}\n", status_message)?;

        buffer_writer.print(&buffer)
    }
}

