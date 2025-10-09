use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub projects: HashMap<String, ProjectConfig>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub default_branch: Option<String>,
}

// todo add documentation/guide
pub fn load_config() -> Option<Config> {
    let path = BaseDirectories::with_prefix("sammy").find_config_file(Path::new("config.yaml"))?;

    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        serde_yaml::from_str(&content).ok()
    } else {
        None
    }
}

impl Config {
    pub fn get_target_branch(&self, project_name: &String) -> String {
        self
            .projects
            .get(project_name)
            .and_then(|c| c.default_branch.clone())
            .unwrap_or("develop".to_string())
    }
}
