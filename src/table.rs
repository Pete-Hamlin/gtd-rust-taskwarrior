use comfy_table::presets::NOTHING;
use comfy_table::{Attribute, Cell, Color, Table};

use crate::project::{generate_project_list, Project};
use crate::{config::GtdConfig, parser::Task};

pub fn project_list_table(cfg: &GtdConfig, tasks: &[Task], projects: &[Project]) {
    let headers = vec!["ID", "Name", "Tasks"];
    let mut table = create_table(&headers);
    let mut output = generate_project_list(tasks, projects);

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

pub fn project_details_table(cfg: &GtdConfig, project: &Project, tasks: &[Task]) {
    let headers = vec!["Name", "Value"];
    let mut table = create_table(&headers);
    table.add_row(vec![Cell::new("Name"), Cell::new(&project.name)]);

    println!("{table}");
    task_list_table(cfg, tasks);
}

fn task_list_table(_cfg: &GtdConfig, tasks: &[Task]) {
    let headers = vec!["ID", "Entry", "Description", "Tags"];
    let mut table = create_table(&headers);
    for (index, item) in tasks.into_iter().enumerate() {
        if index % 2 == 0 {
            table.add_row(vec![
                Cell::new(item.id.to_string()).bg(Color::Black),
                Cell::new(item.entry.to_string()).bg(Color::Black),
                Cell::new(item.description.to_string()).bg(Color::Black),
                Cell::new(item.tags.clone().unwrap_or(vec![]).join(", ")).bg(Color::Black),
            ]);
        } else {
            table.add_row(vec![
                item.id.to_string(),
                item.entry.to_string(),
                item.description.to_string(),
                item.tags.clone().unwrap_or(vec![]).join(", "),
            ]);
        }
    }
    println!("{table}");
}

fn create_table(headers: &[&str]) -> Table {
    let mut table = Table::new();
    table.load_preset(NOTHING);
    let table_headers: Vec<Cell> = headers
        .iter()
        .map(|header| Cell::new(header).add_attribute(Attribute::Underlined))
        .collect();

    table.set_header(table_headers);
    return table;
}
