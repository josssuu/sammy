pub mod repository;

use std::fs::read_dir;
use std::path::Path;
use crate::config::Config;
use crate::utils::repository::Repository;

pub fn collect_repos(config: &Config, filter: &Option<String>, group: &Option<String>) -> Vec<Repository> {
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

            if let Some(group) = group {
                return config.projects
                    .get(&repo.name())
                    .and_then(|project_config| project_config.group.as_ref())
                    .is_some_and(|project_group| project_group == group);
            }

            repo.is_git_project()
        })
        .collect()
}