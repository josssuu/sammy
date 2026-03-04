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

fn default_config_path() -> String {
    get_config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<unresolved>".to_string())
}

impl Runnable for ConfigCommand {
    fn run(&self) {
        match self {
            ConfigCommand::Set { key, value } => match load_config() {
                None => println!("Configuration file not found"),
                Some(mut config) => {
                    let key_parts = key.split(".").collect::<Vec<&str>>();

                    match key_parts.first() {
                        Some(&"projects") => {
                            let project_name = &key_parts[1];

                            match key_parts.get(2) {
                                Some(&"default-branch") => config.set_target_branch(
                                    project_name.to_string(),
                                    String::from(value),
                                ),
                                Some(&"group") => {
                                    config.set_group(project_name.to_string(), String::from(value))
                                }
                                _ => println!("invalid key '{key}'"),
                            };
                        }
                        _ => println!("invalid key '{key}'"),
                    }

                    save_config(&config);
                }
            },
            ConfigCommand::List => {
                println!("Configuration path: {}", default_config_path());

                match load_config() {
                    None => println!("Configuration file not found"),
                    Some(config) => println!("{config:#?}"),
                }
            }
        }
    }
}
