use comfy_table::Table;
use std::error::Error;

use crate::{config::GtdConfig, parser::Task, Project};

struct ProjectListItem {
    index: usize,
    name: String,
    tasks: i32,
}

pub fn project_list(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) {
    let mut table = Table::new();
    let mut output = vec![];
    for (index, project) in projects.iter().enumerate() {
        let count = project_count(tasks, &project.name).unwrap();
        output.push(ProjectListItem {
            index,
            name: project.name.clone(),
            tasks: count,
        });
    }
    output.sort_by(|a, b| a.tasks.cmp(&b.tasks));
    for item in output.iter() {
        if !cfg.short || item.tasks == 0 {
            table.add_row(vec![
                item.index.to_string(),
                item.name.to_string(),
                item.tasks.to_string(),
            ]);
        } else if !cfg.short {
        }
    }
    table.set_header(vec!["ID", "Name", "Tasks Remaining"]);
    println!("{table}");
}

fn project_count(tasks: &[Task], project_title: &str) -> Result<i32, Box<dyn Error>> {
    let count = tasks
        .into_iter()
        .filter(|t| t.project.clone().expect("Error reading project on task") == project_title)
        .count();
    Ok(count as i32)
}
