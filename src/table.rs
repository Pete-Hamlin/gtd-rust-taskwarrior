use comfy_table::presets::NOTHING;
use comfy_table::{Attribute, Cell, Color, Table};
use std::error::Error;

use crate::{config::GtdConfig, parser::Task, project::ProjectListItem};

pub fn display_project_list(cfg: &GtdConfig, tasks: &[Task], projects: &[ProjectListItem]) {
    let mut table = Table::new();
    table.load_preset(NOTHING);
    table.set_header(vec![
        Cell::new("ID").add_attribute(Attribute::Underlined),
        Cell::new("Name").add_attribute(Attribute::Underlined),
        Cell::new("Tasks").add_attribute(Attribute::Underlined),
    ]);
    let mut output = generate_list(tasks, projects);

    output.sort_by(|a, b| a.tasks.cmp(&b.tasks));
    for (index, item) in output.into_iter().enumerate() {
        if !cfg.short || item.tasks == 0 {
            if index % 2 == 0 {
                table.add_row(vec![
                    Cell::new(item.index.to_string()).bg(Color::Black),
                    Cell::new(item.name.to_string()).bg(Color::Black),
                    Cell::new(item.tasks.to_string()).bg(Color::Black),
                ]);
            } else {
                table.add_row(vec![
                    item.index.to_string(),
                    item.name.to_string(),
                    item.tasks.to_string(),
                ]);
            }
        }
    }
    println!("{table}");
}

fn generate_list(tasks: &[Task], projects: &[ProjectListItem]) -> Vec<ProjectListItem> {
    let result: Vec<ProjectListItem> = projects
        .iter()
        .enumerate()
        .map(|(index, project)| {
            let count = project_count(tasks, &project.name).unwrap();
            return ProjectListItem {
                index,
                name: project.name.clone(),
                tasks: count,
            };
        })
        .collect();
    return result;
}

fn project_count(tasks: &[Task], project_title: &str) -> Result<i32, Box<dyn Error>> {
    let count = tasks
        .into_iter()
        .filter(|t| t.project.clone().expect("Error reading project on task") == project_title)
        .count();
    Ok(count as i32)
}
