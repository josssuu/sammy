pub mod repository;

use std::fs::{read_dir, DirEntry};
use std::path::Path;

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

pub fn collect_repos(filter: &Option<String>) -> Vec<DirEntry> {
    // todo - implement utils::repository::Repository
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