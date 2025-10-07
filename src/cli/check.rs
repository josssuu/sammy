use crate::cli::traits::runnable::Runnable;
use clap::Parser;
use std::env::current_dir;
use std::fs::{read_dir, DirEntry};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(Parser)]
pub struct CheckArgs {
    #[arg(long, short, help = "[Experimental] Enable concurrent status check")]
    fast: bool,
    #[arg(long, short, help = "Show current branch", default_value_t = true)]
    show_current: bool,
    #[arg(long, short, help = "Filter repositories")]
    filter: Option<String>,
}

impl Runnable for CheckArgs {
    fn run(&self) {
        if self.fast {
            run_fast(self)
        } else {
            run_slow(self)
        }
    }
}

fn run_fast(args: &CheckArgs) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut handles = vec![];

    for repository in collect_repos(&args.filter) {
        let repository_name = repository.file_name().display().to_string();
        let current_branch_display = if args.show_current {
            Some(get_repository_current_branch(&repository.path()))
        } else {
            None
        };

        // todo - add tracing (simple printing gets messy with async)
        let handle = runtime.spawn( async move {
            let status = get_repository_status(&repository.path(), "develop");
            (repository_name, status, current_branch_display)
        });

        handles.push(handle);
    }


    for handle in handles {
        let (repository, status, current_branch) = runtime.block_on(handle).unwrap();

        print_branch_status(&repository, &status, current_branch)
            .expect("Failed to print branch status");
    }
}

fn run_slow(args: &CheckArgs) {
    for repository in collect_repos(&args.filter) {
        let repository_name = repository.file_name().display().to_string();

        let current_branch = if args.show_current {
            Some(get_repository_current_branch(&repository.path()))
        } else {
            None
        };

        // todo - add tracing (simple printing gets messy with async)
        let status = get_repository_status(&repository.path(), "develop");

        print_branch_status(&repository_name, &status, current_branch)
            .expect("Failed to print branch status");
    }
}


fn is_git_project(path: &Path) -> bool {
    if !path.is_dir() {
        return false
    }

    read_dir(path)
        .expect("Failed to read directory")
        .any(|entry| {
            let entry = entry.expect("No such entry");
            let git = Path::new(".git");
            entry.file_name().eq(git)
        })
}

fn get_remote_head(repository: &Path, branch: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["ls-remote", "--heads", "origin", branch])
        .current_dir(repository)
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
        let hash = stdout.split_once('\t')
            .map(|(h, _)| h.trim())
            .filter(|h| !h.is_empty())
            .map(|h| h.to_string());

        hash
    } else {
        None
    }
}

fn get_local_head(repository: &Path, branch: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", branch])
        .current_dir(repository)
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
        Some(stdout)
    } else {
        None
    }
}

fn is_local_ahead(repository: &Path, branch: &str) -> Option<bool> {
    let output = Command::new("git")
        .args(["rev-list", "--count", "--left-only", format!("{}...origin/{}", branch, branch).as_str()])
        .current_dir(repository)
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
        let diff_count = stdout.parse::<usize>().expect("Failed to parse stdout");
        Some(diff_count > 0)
    } else {
        None
    }
}

fn get_repository_status(path: &Path, branch: &str) -> BranchStatus {
    let repository_path = current_dir().expect("Unable to get current directory").join(path);

    let Some(remote_head) = get_remote_head(&repository_path, branch) else {
        return BranchStatus::RemoteNotFound
    };

    let Some(local_head) = get_local_head(&repository_path, branch) else {
        return BranchStatus::LocalNotFound
    };

    if local_head == remote_head {
        BranchStatus::UpToDate
    }
    else if is_local_ahead(&repository_path, branch).expect("Unable to check local branch status") {
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

fn print_branch_status(repository: &str, status: &BranchStatus, current_branch: Option<String>) -> io::Result<()> {
    let (status_message, color) = match status {
        BranchStatus::UpToDate => ("up to date", Color::Green),
        BranchStatus::UpdateAvailable => ("update available", Color::Yellow),
        BranchStatus::LocalAhead => ("local is ahead", Color::Magenta),
        BranchStatus::RemoteNotFound => ("remote 'develop' branch not found", Color::Red),
        BranchStatus::LocalNotFound => ("local 'develop' branch not found", Color::Red),
    };

    let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = buffer_writer.buffer();

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    write!(&mut buffer, "{:<35}", repository)?;

    if let Some(branch) = current_branch {
        write!(&mut buffer, "{:<10}", branch)?;
    }

    write!(&mut buffer, "| ")?;

    buffer.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut buffer, "{}\n", status_message)?;

    buffer_writer.print(&buffer)
}

fn get_repository_current_branch(repository: &Path) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repository)
        .output()
        .expect("Failed to execute git command");

    let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
    stdout
}

fn collect_repos(filter: &Option<String>) -> Vec<DirEntry> {
    read_dir(Path::new("./"))
        .expect("Failed to read directory")
        .map(|entry| entry.expect("Failed to read entry"))
        .filter(|entry| {
            if let Some(filter) = filter {
                let repository_name = entry.file_name().display().to_string();
                if !repository_name.contains(filter) {
                    return false
                }
            }

            is_git_project(&entry.path())
        })
        .collect()
}
