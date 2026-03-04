use crate::cli::traits::runnable::Runnable;
use crate::config::{get_config_path, load_config, save_config};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub(crate) enum ConfigCommand {
    #[command(about = "Update configuration")]
    Set { key: String, value: String },
    #[command(about = "List configurations")]
    List,
}

impl Runnable for ConfigCommand {
    fn run(&self) {
        load_config().unwrap_or_default();
        match self {
            ConfigCommand::Set { key, value } => {
                let mut config = load_config().unwrap_or_default();
                let key_parts = key.split(".").collect::<Vec<&str>>();
                let mut changed = false;

                match key_parts.first() {
                    Some(&"projects") => {
                        let project_name = &key_parts[1];

                        match key_parts.get(2) {
                            Some(&"default-branch") => {
                                config.set_target_branch(
                                    project_name.to_string(),
                                    String::from(value),
                                );
                                changed = true;
                            }
                            Some(&"group") => {
                                config.set_group(project_name.to_string(), String::from(value));
                                changed = true;
                            }
                            _ => println!("invalid key '{key}'"),
                        };
                    }
                    _ => println!("invalid key '{key}'"),
                }

                if changed {
                    save_config(&config);
                }
            }
            ConfigCommand::List => {
                println!("Configuration path: {}", get_config_path().display());

                match load_config() {
                    None => println!("Configuration file not found"),
                    Some(config) => println!("{config:#?}"),
                }
            }
        }
    }
}
