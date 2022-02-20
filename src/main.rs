#![recursion_limit = "1024"]

use serde::{Deserialize, Serialize};
use std::process::Command;

// Config setup
#[derive(Serialize, Deserialize)]
struct GtdConfig {
    initialized: bool,
    task_path: String,
}

impl ::std::default::Default for GtdConfig {
    fn default() -> Self {
        Self {
            initialized: false,
            task_path: "task".into(),
        }
    }
}

fn main() {
    let cfg: GtdConfig = init_config();
    println!("task_path: {}", cfg.task_path);
    let output = Command::new("task")
        .arg("_unique")
        .arg("project")
        .output()
        .expect("Command failed");

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| println!("{}", x));
}

fn init_config() -> GtdConfig {
    let cfg: GtdConfig = confy::load("gtd-rust").expect("Failed to load config");
    if !cfg.initialized {
        println!("Attempting to find task in $PATH...");
        // Check if `task` in current path
        Command::new("which")
            .arg("task")
            .status()
            .expect("Failed to find task binary - please set manually");
        let new_cfg = GtdConfig {
            task_path: "task".into(),
            initialized: true,
        };
        confy::store("gtd-rust", new_cfg).expect("Failed to load new config");
        return confy::load("gtd-rust").expect("Failed to load config");
    } else {
        return cfg;
    }
}
