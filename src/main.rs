#![recursion_limit = "1024"]

use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
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
    name: String,
}

struct ProjectListItem {
    index: usize,
    name: String,
    tasks: i32,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Command to run
    command: Option<String>,
    /// Optional subcommand to work on
    subcommand: Option<String>,

    /// Display only projects without tasks
    #[clap(short, long)]
    short: bool,

    /// Do not switch context during operation
    #[clap(short, long)]
    nosetcontext: bool,
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

    if let Some(command) = args.command.as_deref() {
        match command {
            "init" => init_projects(),
            "list" => list_projects(args),
            "add" => insert_project(args),
            "reset" => reset_projects(),
            _ => parse_subcommand(args),
        }
    } else {
        list_projects(args)
    }
}

fn parse_subcommand(args: Cli) {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match subcommand {
            "done" => remove_project(args),
            _ => println!("Subcommand {} not found", subcommand),
        }
    } else {
        println!("Please provide a valid subcommand");
    }
}

fn init_config() {
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

fn reset_projects() {
    let empty_vec = vec![];
    write_project_list(&empty_vec).unwrap();
}

fn init_projects() -> () {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    let output = Command::new(cfg.task_path)
        .arg("_unique")
        .arg("project")
        .output()
        .expect("Command failed- check task binary");

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .for_each(|x| add_project_item(x.to_string()).unwrap());
}

// Add/Remove projects
fn insert_project(args: Cli) -> () {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match add_project_item(subcommand.to_string()) {
            Ok(_p) => println!("Successfully processed project"),
            Err(e) => println!("Failed to add project {:?}", e),
        }
    } else {
        println!("No project specified - Please provide a project name or run gtd --help for more details")
    }
}

fn remove_project(args: Cli) -> () {
    if let Some(command) = args.command.as_deref() {
        match remove_project_item(command.to_string()) {
            Ok(p) => println!("Successfully removed project {:?}", p),
            Err(e) => println!("Failed to remove project {:?}", e),
        }
    } else {
        println!("No project specified - Please provide a project name or run gtd --help for more details")
    }
}

fn add_project_item(project: String) -> io::Result<()> {
    let mut projects = get_projects_list();
    if check_duplicates(&project) {
        projects.push(Project { name: project });
        write_project_list(&projects)?;
    }
    Ok(())
}

fn remove_project_item(project_id: String) -> io::Result<String> {
    let mut projects = get_projects_list();
    let project = projects.remove(project_id.parse::<usize>().unwrap());
    write_project_list(&projects)?;
    Ok(project.name)
}

fn get_projects_list() -> Vec<Project> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    let file = File::open(cfg.storage_path)
        .expect("Project storage file not found - Check your config location");
    return serde_json::from_reader(file).expect("Error reading file");
}

fn write_project_list(projects: &Vec<Project>) -> io::Result<()> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    serde_json::to_writer(&File::create(cfg.storage_path)?, &projects)?;
    Ok(())
}

// Context
fn get_context() -> String {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    let output = Command::new(cfg.task_path)
        .arg("_get")
        .arg("rc.context")
        .output()
        .unwrap();
    let context: String = String::from_utf8(output.stdout).unwrap().replace("\n", "");
    return context;
}

fn check_context(context: String) {
    if !context.trim().is_empty() {
        let text = format!("!!!WARNING: Context set to {}", context);
        println!("{}", text.red())
    }
}

fn set_context(context: String) {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    // let text = format!("Setting context to {}", context.clone());
    Command::new(cfg.task_path)
        .arg("context")
        .arg(context.to_string())
        .status()
        .expect("Failed to set context");
    // println!("{}", output);
}

// Project listing
fn list_projects(args: Cli) -> () {
    let context = get_context();
    if args.nosetcontext {
        check_context(context.clone());
    } else {
        set_context("none".to_string());
    }
    let projects = get_projects_list();
    let mut output = vec![];
    for (index, project) in projects.iter().enumerate() {
        let count = project_count(project).unwrap();
        output.push(ProjectListItem {
            index,
            name: project.name.clone(),
            tasks: count,
        });
    }
    output.sort_by(|a, b| a.tasks.cmp(&b.tasks));
    for item in output.iter() {
        let text = format!(
            "{} | {} - Has {} tasks remaining",
            item.index, item.name, item.tasks
        );
        if item.tasks == 0 {
            println!("{}", text.yellow());
        } else if !args.short {
            println!("{}", text.green());
        }
    }
    if !args.nosetcontext {
        set_context(context);
    }
}

fn check_duplicates(project_name: &str) -> bool {
    let projects = get_projects_list();
    for project in projects.iter() {
        if project_name == project.name {
            println!("{} already in project list - skipping", project_name);
            return false;
        }
    }
    return true;
}

fn project_count(project: &Project) -> io::Result<i32> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    let mut text = "pro:".to_string();
    text = text + &project.name;
    let output = Command::new(cfg.task_path)
        .arg(text)
        .arg("\\(status:waiting or status:pending\\)'")
        .arg("count")
        .output()?;
    let value: String = String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .collect();
    Ok(value.parse::<i32>().unwrap())
}
