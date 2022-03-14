#![recursion_limit = "1024"]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::process::Command;

// Config setup
#[derive(Serialize, Deserialize)]
struct GtdConfig {
    initialized: bool,
    storage_path: String,
    task_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Project {
    id: usize,
    name: String,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Command to run
    command: String,
    // Optional subcommand to work on
    subcommand: Option<String>,
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

fn main() {
    init_config();
    let args = Cli::parse();

    match args.command.as_str() {
        "init" => init_projects(),
        "insert" => insert_project(args),
        "list" => list_projects(),
        _ => println!("Subcommand {} not found", args.command),
    }
}

fn init_config() {
    // Allows for running tasks on initial loading of config
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
            storage_path: "./projects.json".into(),
            initialized: true,
        };
        confy::store("gtd-rust", new_cfg).expect("Failed to load new config");
    }
}

fn init_projects() -> () {
    let cfg: GtdConfig = confy::load("gtd-rust").expect("Failed to load config");
    let output = Command::new(cfg.task_path)
        .arg("_unique")
        .arg("project")
        .output()
        .expect("Command failed- check task binary");

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| save_new_project(x.to_string()).unwrap());
}

fn insert_project(args: Cli) -> () {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match save_new_project(subcommand.to_string()) {
            Ok(p) => println!("Successfully added project {:?}", p),
            Err(e) => println!("Failed to add project {:?}", e),
        }
    } else {
        println!("No project specified - Please provide a project name or run gtd --help for more details")
    }
}

fn save_new_project(project: String) -> io::Result<()> {
    let cfg: GtdConfig = confy::load("gtd-rust").expect("Failed to load config");
    let mut projects = get_projects_list();
    projects.push(Project {
        id: 0,
        name: project,
    });
    serde_json::to_writer(&File::create(cfg.storage_path)?, &projects)?;
    sync_project_list(&mut projects)?;
    Ok(())
}

fn get_projects_list() -> Vec<Project> {
    let cfg: GtdConfig = confy::load("gtd-rust").expect("Failed to load config");
    let file = File::open(cfg.storage_path)
        .expect("Project storage file not found - Check your config location");
    return serde_json::from_reader(file).expect("Error reading file");
}

fn sync_project_list(projects: &mut Vec<Project>) -> io::Result<()> {
    let cfg: GtdConfig = confy::load("gtd-rust").expect("Failed to load config");
    // This follows the task approach where every item has id n, that udpates whenever a value is added/removed (i.e. n items will always have id 1-n)
    let mut id = 1;
    for mut project in projects.iter_mut() {
        project.id = id;
        println!("Project: {} - {}", project.id, project.name);
        id += 1;
    }
    serde_json::to_writer(&File::create(cfg.storage_path)?, &projects)?;
    Ok(())
}

fn list_projects() -> () {}
