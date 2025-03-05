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
        // Set colors
        let color = if cfg.color {
            determine_proj_color(item.tasks as usize)
        } else {
            Color::Reset
        };
        let bg_color = if cfg.color && index % 2 == 0 {
            Color::Black
        } else {
            Color::Reset
        };

        if !cfg.short || item.tasks == 0 {
            table.add_row(vec![
                Cell::new(item.index.to_string()).fg(color).bg(bg_color),
                Cell::new(item.name.to_string()).fg(color).bg(bg_color),
                Cell::new(item.tasks.to_string()).fg(color).bg(bg_color),
            ]);
        }
    }
    println!("{table}");
}

pub fn project_details_table(cfg: &GtdConfig, project: &Project, tasks: &[Task]) {
    let headers = vec!["Name", "Value"];
    let mut table = create_table(&headers);
    let color = if cfg.color {
        determine_proj_color(tasks.len())
    } else {
        Color::Reset
    };
    table.add_row(vec![Cell::new("Name"), Cell::new(&project.name).fg(color)]);

    println!("{table}");
    if tasks.len() > 0 {
        task_list_table(cfg, tasks);
    }
}

fn task_list_table(cfg: &GtdConfig, tasks: &[Task]) {
    let headers = vec!["ID", "Entry", "Description", "Tags"];
    let mut table = create_table(&headers);
    for (index, item) in tasks.into_iter().enumerate() {
        let bg_color = if cfg.color && index % 2 == 0 {
            Color::Black
        } else {
            Color::Reset
        };
        table.add_row(vec![
            Cell::new(item.id.to_string()).bg(bg_color),
            Cell::new(item.entry.to_string()).bg(bg_color),
            Cell::new(item.description.to_string()).bg(bg_color),
            Cell::new(item.tags.clone().unwrap_or(vec![]).join(", ")).bg(bg_color),
        ]);
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

fn determine_proj_color(task_count: usize) -> Color {
    if task_count == 0 {
        return Color::Yellow;
    } else {
        return Color::Green;
    }
}
