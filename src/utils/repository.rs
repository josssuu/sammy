use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Repository {
    pub path: PathBuf,
}

// todo - save logs to file
impl Repository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn name(&self) -> String {
        self
            .path
            .file_name()
            .expect("Path does not have a file name")
            .to_str()
            .expect("Path does not contain valid UTF-8")
            .to_string()
    }

    pub fn is_git_project(&self) -> bool {
        if !self.path.is_dir() {
            return false
        }

        read_dir(self.path.clone())
            .expect("Failed to read directory")
            .any(|entry| {
                let entry = entry.expect("No such entry");
                let git = Path::new(".git");
                entry.file_name().eq(git)
            })
    }

    pub fn get_current_branch(&self) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
        stdout
    }

    pub fn fetch(&self) -> Result<(), ()> {
        let output = Command::new("git")
            .args(["fetch"])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        if output.status.success() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn checkout(&self, branch: &String) -> Result<(), ()> {
        let output = Command::new("git")
            .args(["checkout", branch])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        if output.status.success() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pull(&self) -> Result<(), ()> {
        let output = Command::new("git")
            .args(["pull"])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        if output.status.success() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_remote_head(&self, branch: &str) -> Option<String> {
        let output = Command::new("git")
            .args(["ls-remote", "--heads", "origin", branch])
            .current_dir(self.path.clone())
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

    pub fn get_local_head(&self, branch: &str) -> Option<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--verify", branch])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
            Some(stdout)
        } else {
            None
        }
    }

    pub fn is_local_ahead(&self, branch: &str) -> Option<bool> {
        let output = Command::new("git")
            .args(["rev-list", "--count", "--left-only", format!("{}...origin/{}", branch, branch).as_str()])
            .current_dir(self.path.clone())
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

    pub fn has_pending_changes(&self) -> Option<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(self.path.clone())
            .output()
            .expect("Failed to execute git command");

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout").trim().to_string();
            Some(!stdout.is_empty())
        } else {
            None
        }
    }
}
