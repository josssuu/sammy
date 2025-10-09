pub mod repository;

use std::fs::read_dir;
use std::path::Path;
use crate::utils::repository::Repository;

pub fn collect_repos(filter: &Option<String>) -> Vec<Repository> {
    read_dir(Path::new("./"))
        .expect("Failed to read directory")
        .map(|entry| {
            let entry = entry.expect("Failed to read entry");
            Repository::new(entry.path())
        })
        .filter(|repo| {
            if let Some(filter) = filter {
                if !repo.name().contains(filter) {
                    return false
                }
            }

            repo.is_git_project()
        })
        .collect()
}