#![recursion_limit = "1024"]
use std::error::Error;

mod config;
mod parser;
mod project;
mod table;

use config::{get_config, Cli, GtdConfig};
use parser::{get_task_list, Task};
use project::{generate_project_list, get_projects, write_project_list, Project};
use table::display_project_list;

use clap::Parser;
use std::fs::remove_file;
use std::io;

fn main() {
    let args = Cli::parse();
    let cfg = get_config(&args);
    let tasks = get_task_list(&cfg).expect("Failed to get task list");
    let mut projects = get_projects(&cfg).expect("Failed to retrieve project list");

    if let Some(command) = args.command.as_deref() {
        match command {
            "init" => init_projects(&cfg, &tasks),
            "list" => list_projects(&cfg, &tasks, &projects),
            "count" => count_projects(&cfg, &tasks, &projects),
            // "add" => add_project(&args, &mut projects),
            "reset" => reset_projects(&cfg),
            _ => parse_subcommand(&cfg, &args, &tasks, &mut projects),
        }
    } else {
        list_projects(&cfg, &tasks, &projects)
    }
}

fn parse_subcommand(_cfg: &GtdConfig, args: &Cli, tasks: &[Task], projects: &mut Vec<Project>) {
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
            "done" => mark_project_done(id, projects),
            "delete" => delete_item(id, projects),
            "show" => show_project(id, projects, tasks),
            _ => println!("Subcommand {} not found", subcommand),
        }
    } else {
        show_project(id, projects, tasks);
    }
}

fn init_projects(cfg: &GtdConfig, tasks: &[Task]) -> () {
    let mut name_list: Vec<String> = vec![];
    tasks.into_iter().for_each(|task| {
        let project_name = task.project.clone().unwrap();
        if !name_list.contains(&project_name) {
            name_list.push(project_name);
        }
    });
    let projects: Vec<Project> = name_list.into_iter().map(|name| Project { name }).collect();
    write_project_list(cfg, &projects);
}

fn list_projects(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) {
    let project_list = generate_project_list(tasks, projects);
    display_project_list(cfg, tasks, &project_list);
}

fn count_projects(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) {
    if !cfg.short {
        let count = projects.into_iter().count();
        println!("{:?}", count)
    } else {
        let count = generate_project_list(tasks, projects)
            .into_iter()
            .filter(|p| p.tasks == 0)
            .count();
        println!("{:?}", count)
    }
}

fn reset_projects(cfg: &GtdConfig) {
    remove_file(&cfg.storage_path).expect("Error removing config file.")
}

fn show_project(project_id: usize, projects: &[Project], _tasks: &[Task]) {
    let _project = &projects[project_id];
    // let tasks = tasks
    //     .into_iter()
    //     .filter(|t| t.project == project.name)
    //     .collect();
}

// Add/Remove projects
fn add_project(args: &Cli, projects: &mut Vec<Project>) -> () {
    if let Some(subcommand) = args.subcommand.as_deref() {
        match add_project_item(subcommand.to_string(), projects) {
            Ok(_p) => println!("Successfully processed project"),
            Err(e) => println!("Failed to add project {:?}", e),
        }
    } else {
        println!("No task specified - run `proj --help` for guidance on running this command")
    }
}

fn mark_project_done(proj_id: usize, projects: &mut Vec<Project>) -> () {
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
    Ok(())
}

fn remove_project_item(
    project_id: usize,
    projects: &mut Vec<Project>,
) -> Result<String, Box<dyn Error>> {
    let project = projects.remove(project_id);
    Ok(project.name)
}
