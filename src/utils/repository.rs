use std::path::PathBuf;
use std::process::Command;

pub struct Repository {
    path: PathBuf,
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
