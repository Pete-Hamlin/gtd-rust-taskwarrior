use crate::config::GtdConfig;
use crate::parser::Task;
use serde::{Deserialize, Serialize};
use std::error::Error;

use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    // tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListItem {
    pub index: usize,
    pub name: String,
    pub tasks: i32,
}

pub fn get_projects(cfg: &GtdConfig) -> Result<Vec<Project>, Box<dyn Error>> {
    let file = File::open(&cfg.storage_path)
        .expect("Project storage file not found - Check your config location");
    let projects: Vec<Project> = match serde_json::from_reader(file) {
        Ok(projects) => projects,
        Err(_) => vec![],
    };
    Ok(projects)
}

/// Converts the imported JSON `Project` struct to a `ProjectListItem` (the data we wish to display).
/// Currently attaches the following data:
/// - Current pending task count
///
/// * `tasks`: Parsed task list JSON
/// * `projects`: Parsed project list JSON
pub fn generate_project_list(tasks: &[Task], projects: &[Project]) -> Vec<ProjectListItem> {
    let result: Vec<ProjectListItem> = projects
        .iter()
        .enumerate()
        .map(|(index, project)| {
            let count = task_count(tasks, &project.name).unwrap();
            return ProjectListItem {
                index,
                name: project.name.clone(),
                tasks: count,
            };
        })
        .collect();
    return result;
}

fn task_count(tasks: &[Task], project_title: &str) -> Result<i32, Box<dyn Error>> {
    let count = tasks
        .into_iter()
        .filter(|t| t.project.clone().expect("Error reading project on task") == project_title)
        .count();
    Ok(count as i32)
}

pub fn write_project_list(cfg: &GtdConfig, projects: &[Project]) -> Result<(), Box<dyn Error>> {
    serde_json::to_writer(&File::create(&cfg.storage_path)?, &projects)?;
    Ok(())
}
