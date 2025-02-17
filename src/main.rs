#![recursion_limit = "1024"]
use std::error::Error;

mod config;
mod parser;

use config::{init_config, Cli, GtdConfig};
use parser::{get_task_list, Task};

use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{remove_file, File};
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
    let mut projects = get_projects_list().expect("Failed to retrieve project list");

    if let Some(command) = args.command.as_deref() {
        match command {
            "init" => init_projects(&tasks),
            "list" => list_projects(&args, &cfg, &projects),
            "add" => insert_project(&args, &mut projects),
            "reset" => reset_projects(&cfg),
            _ => parse_subcommand(&args, &mut projects),
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

fn parse_subcommand(args: &Cli, projects: &mut Vec<Project>) {
    // If we have subcommands, command should be a project ID, which is an an integer
    let id: usize = args
        .command
        .clone()
        .expect("ID incorrect format, check gtd --help for correct syntax")
        .parse::<usize>()
        .unwrap();
    if id > projects.len() {
        println!("No project found with ID {:?}", id.to_string());
        return;
    }
    if let Some(subcommand) = args.subcommand.as_deref() {
        match subcommand {
            "done" => mark_item_as_done(id, projects),
            "delete" => delete_item(id, projects),
            _ => println!("Subcommand {} not found", subcommand),
        }
    } else {
        println!("Please provide a valid subcommand");
    }
}

fn reset_projects(cfg: &GtdConfig) {
    remove_file(&cfg.storage_path).expect("Error removing config file.")
}

fn init_projects(tasks: &Vec<Task>) -> () {
    let mut name_list: Vec<String> = vec![];
    let mut projects: Vec<Project> = vec![];
    tasks.into_iter().for_each(|task| {
        let project_name = task.project.clone().unwrap();
        if !name_list.contains(&project_name) {
            update_project_entry(task, &project_name, &mut projects);
            name_list.push(project_name);
        }
    });
    write_project_list(&mut projects);
}

fn update_project_entry(task: &Task, proj_name: &str, projects: &mut Vec<Project>) -> () {
    println!("{:?}", proj_name);
}

// Add/Remove projects
fn insert_project(args: &Cli, projects: &mut Vec<Project>) -> () {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match add_project_item(subcommand.to_string(), projects) {
            Ok(_p) => println!("Successfully processed project"),
            Err(e) => println!("Failed to add project {:?}", e),
        }
    } else {
        println!("No task specified - please see gtd --help on running this command")
    }
}

fn mark_item_as_done(proj_id: usize, projects: &mut Vec<Project>) -> () {
    match remove_project_item(proj_id, projects) {
        Ok(p) => println!("Successfully removed project {:?}", p),
        Err(e) => println!("Failed to remove project {:?}", e),
    }
}

fn delete_item(proj_id: usize, projects: &mut Vec<Project>) -> () {
    match remove_project_item(proj_id, projects) {
        Ok(p) => println!("Successfully removed project {:?}", p),
        Err(e) => println!("Failed to remove project {:?}", e),
    }
}
fn add_project_item(project: String, projects: &mut Vec<Project>) -> io::Result<()> {
    projects.push(Project { name: project });
    write_project_list(projects)?;
    Ok(())
}

fn remove_project_item(
    project_id: usize,
    projects: &mut Vec<Project>,
) -> Result<String, Box<dyn Error>> {
    let project = projects.remove(project_id);
    write_project_list(projects)?;
    Ok(project.name)
}

fn get_projects_list() -> Result<Vec<Project>, Box<dyn Error>> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    let file = File::open(cfg.storage_path)
        .expect("Project storage file not found - Check your config location");
    let projects: Vec<Project> = match serde_json::from_reader(file) {
        Ok(projects) => projects,
        Err(_) => vec![],
    };
    Ok(projects)
}

fn write_project_list(projects: &mut Vec<Project>) -> io::Result<()> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    serde_json::to_writer(&File::create(cfg.storage_path)?, &projects)?;
    Ok(())
}

// Project listing
fn list_projects(_: &Cli, cfg: &GtdConfig, projects: &Vec<Project>) -> () {
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
