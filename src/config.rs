use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Command to run (init, list, add, reset or a project ID)
    pub command: Option<String>,
    /// Optional subcommand to work on
    pub subcommand: Option<String>,

    /// Display only projects without tasks
    #[clap(short, long)]
    pub short: bool,
}

// Config setup
#[derive(Serialize, Deserialize)]
pub struct GtdConfig {
    pub initialized: bool,
    pub storage_path: String,
    pub task_path: String,
    pub short: bool,
}

impl ::std::default::Default for GtdConfig {
    fn default() -> Self {
        Self {
            initialized: false,
            task_path: "task".into(),
            storage_path: "./projects.json".into(),
            short: true,
        }
    }
}

pub fn init_config(args: &Cli) -> GtdConfig {
    // Allows for running tasks on initial loading of config
    let cfg: GtdConfig = confy::load("projwarrior", None).expect("Failed to load config");
    if !cfg.initialized {
        println!("Attempting to find task in $PATH...");
        // Check if `task` in current path
        Command::new("which")
            .arg("task")
            .status()
            .expect("Failed to find task binary - please ensure the `task` command is available in your $PATH");
        let storage_path = env::var("HOME").unwrap() + "/.task/projects.data";
        let new_cfg = GtdConfig {
            task_path: "task".into(),
            storage_path,
            initialized: true,
            short: args.short,
        };
        confy::store("projwarrior", None, new_cfg).expect("Failed to load new config");
    }
    return cfg;
}
