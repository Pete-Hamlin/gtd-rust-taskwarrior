#![recursion_limit = "1024"]

mod config;
mod parser;

use config::{init_config, Cli, GtdConfig};
use parser::{get_task_list, Task};

use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::process::Command;

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

fn main() {
    let args = Cli::parse();
    let cfg = init_config(&args);
    let tasks = get_task_list().expect("Failed to get task list");

    if let Some(command) = args.command.as_deref() {
        match command {
            "init" => init_projects(&cfg),
            "list" => list_projects(&args, &cfg),
            "add" => insert_project(&args),
            "reset" => reset_projects(),
            "test" => reset_projects(),
            _ => parse_subcommand(&args),
        }
    } else {
        // list_projects(args)
        test_task_list(&tasks)
    }
}

fn test_task_list(tasks: &Vec<Task>) {
    tasks.into_iter().for_each(|task| {
        println!(
            "{:?}: {:?} - {:?}\n",
            task.id,
            task.description,
            task.project
                .clone()
                .expect("Something went wrong with project parsing!")
        )
    });
}

fn parse_subcommand(args: &Cli) {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match subcommand {
            "done" => remove_project(args),
            _ => println!("Subcommand {} not found", subcommand),
        }
    } else {
        println!("Please provide a valid subcommand");
    }
}

fn reset_projects() {
    let empty_vec = vec![];
    write_project_list(&empty_vec).unwrap();
}

fn init_projects(cfg: &GtdConfig) -> () {
    let output = Command::new(&cfg.task_path)
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
fn insert_project(args: &Cli) -> () {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match add_project_item(subcommand.to_string()) {
            Ok(_p) => println!("Successfully processed project"),
            Err(e) => println!("Failed to add project {:?}", e),
        }
    } else {
        println!("No project specified - Please provide a project name or run gtd --help for more details")
    }
}

fn remove_project(args: &Cli) -> () {
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

// Project listing
fn list_projects(_: &Cli, cfg: &GtdConfig) -> () {
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
        } else if !cfg.short {
            println!("{}", text.green());
        }
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
