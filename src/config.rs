use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

// Config setup
#[derive(Serialize, Deserialize)]
pub struct GtdConfig {
    pub initialized: bool,
    pub storage_path: String,
    pub task_path: String,
}

impl ::std::default::Default for GtdConfig {
    fn default() -> Self {
        Self {
            initialized: false,
            task_path: "task".into(),
            storage_path: "./projects.json".into(),
        }
    }
}

pub fn init_config() {
    // Allows for running tasks on initial loading of config
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    if !cfg.initialized {
        println!("Attempting to find task in $PATH...");
        // Check if `task` in current path
        Command::new("which")
            .arg("task")
            .status()
            .expect("Failed to find task binary - please set manually");
        let storage_path = env::var("HOME").unwrap() + "/.task/projects.data";
        let new_cfg = GtdConfig {
            task_path: "task".into(),
            storage_path,
            initialized: true,
        };
        confy::store("gtd-rust", None, new_cfg).expect("Failed to load new config");
    }
}
