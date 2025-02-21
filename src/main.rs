#![recursion_limit = "1024"]
use std::error::Error;

mod config;
mod parser;
mod project;
mod table;

use config::{get_config, Cli, GtdConfig};
use parser::{get_task_list, Task};
use project::{generate_project_list, get_projects, Project};
use table::display_project_list;

use clap::Parser;
use std::fs::{remove_file, File};
use std::io;

fn main() {
    let args = Cli::parse();
    let cfg = get_config(&args);
    let tasks = get_task_list(&cfg).expect("Failed to get task list");
    let mut projects = get_projects(&cfg).expect("Failed to retrieve project list");

    if let Some(command) = args.command.as_deref() {
        match command {
            "init" => init_projects(&tasks),
            "list" => list_projects(&cfg, &tasks, &projects),
            "add" => add_project(&args, &mut projects),
            "reset" => reset_projects(&cfg),
            _ => parse_subcommand(&args, &tasks, &mut projects),
        }
    } else {
        list_projects(&cfg, &tasks, &projects)
    }
}

fn parse_subcommand(args: &Cli, tasks: &[Task], projects: &mut Vec<Project>) {
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
        show_project(&projects, &tasks, id);
    }
}

fn init_projects(tasks: &[Task]) -> () {
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

fn list_projects(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) {
    let project_list = generate_project_list(tasks, projects);
    display_project_list(cfg, tasks, &project_list);
}

fn reset_projects(cfg: &GtdConfig) {
    remove_file(&cfg.storage_path).expect("Error removing config file.")
}

fn show_project(projects: &[Project], tasks: &[Task], project_id: usize) {
    let project = &projects[project_id];
    // let tasks = tasks
    //     .into_iter()
    //     .filter(|t| t.project == project.name)
    //     .collect();
}

fn update_project_entry(task: &Task, proj_name: &str, projects: &mut Vec<Project>) -> () {
    println!("{:?}", proj_name);
}

// Add/Remove projects
fn add_project(args: &Cli, projects: &mut Vec<Project>) -> () {
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

fn write_project_list(projects: &mut Vec<Project>) -> io::Result<()> {
    let cfg: GtdConfig = confy::load("gtd-rust", None).expect("Failed to load config");
    serde_json::to_writer(&File::create(cfg.storage_path)?, &projects)?;
    Ok(())
}

// Project listing
//fn list_projects(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) -> () {
//    let mut output = vec![];
//    for (index, project) in projects.iter().enumerate() {
//        let count = project_count(tasks, &project.name).unwrap();
//        output.push(ProjectListItem {
//            index,
//            name: project.name.clone(),
//            tasks: count,
//        });
//    }
//    output.sort_by(|a, b| a.tasks.cmp(&b.tasks));
//    for item in output.iter() {
//        let text = format!(
//            "{} | {} - Has {} tasks remaining",
//            item.index, item.name, item.tasks
//        );
//        if item.tasks == 0 {
//            println!("{}", text.yellow());
//        } else if !cfg.short {
//            println!("{}", text.green());
//        }
//    }
//}
