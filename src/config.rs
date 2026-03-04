use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub projects: HashMap<String, ProjectConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectConfig {
    pub default_branch: Option<String>,
    pub group: Option<String>,
}

pub(crate) fn get_config_path() -> Option<PathBuf> {
    BaseDirectories::with_prefix("sammy").find_config_file(Path::new("config.yaml"))
}

// todo add documentation/guide
pub fn load_config() -> Option<Config> {
    let path = get_config_path()?;

    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        serde_yaml::from_str(&content).ok()
    } else {
        None
    }
}

pub fn save_config(config: &Config) {
    let path = get_config_path().expect("unable to get config path");
    let content = serde_yaml::to_string(config).expect("Unable to serialize config");

    fs::write(path, content).expect("Unable to write config file");
}

impl Config {
    pub fn get_target_branch(&self, project_name: &str) -> String {
        self.projects
            .get(project_name)
            .and_then(|c| c.default_branch.clone())
            .unwrap_or("develop".to_string())
    }

    pub fn set_target_branch(&mut self, project_name: String, branch: String) {
        let project = self.projects.entry(project_name).or_default();
        project.default_branch = Some(branch);
    }

    pub fn set_group(&mut self, project_name: String, group: String) {
        let project = self.projects.entry(project_name).or_default();
        project.group = Some(group);
    }
}
