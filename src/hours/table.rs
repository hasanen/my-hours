//! Print time entries to terminal in table
use crate::hours::types::{self, TimeEntryCalculations};
use crate::settings::{ProjectConfig, ProjectConfigs};
use prettytable::{format, Attr, Cell, Row, Table};

/// Prints given entries to terminal
pub fn print(time_entries: &types::TimeEntries, project_configs: &ProjectConfigs) {
    println!("");
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        header_cell(&"Project"),
        header_cell(&"Today"),
        header_cell(&"Current week / Daily AVG"),
        header_cell(&"Current month / Daily AVG"),
        header_cell(&"Target (day / week / month)"),
    ]));
    for project in time_entries.uniq_projects() {
        let project_config = project_configs.get(&project).unwrap();
        table.add_row(Row::new(vec![
            Cell::new(&format_project_title(&project)),
            Cell::new(&format_duration(&project.total_hours_for_current_day())).style_spec(
                &format!(
                    "b{}",
                    target_hours_color(
                        &project_config.target_daily_hours,
                        &time_entries.total_hours_for_current_day()
                    )
                ),
            ),
            Cell::new(&format_weekly_hours(&project)),
            Cell::new(&format_duration(&project.total_hours())),
            Cell::new(&format_targets(project_config)),
        ]));
    }
    table.add_row(Row::new(vec![
        Cell::new("Total").style_spec("b"),
        Cell::new(&format_duration(
            &time_entries.total_hours_for_current_day(),
        ))
        .style_spec("b"),
        Cell::new(""),
        Cell::new(&format_duration(&time_entries.total_hours())).style_spec("b"),
        Cell::new(""),
    ]));
    table.printstd();
}

fn header_cell(title: &str) -> Cell {
    return Cell::new(title).with_style(Attr::Bold);
}
fn format_duration(duration: &chrono::Duration) -> String {
    if duration.num_minutes() > 0 {
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - hours * 60;
        format!("{:3}h {:2}m", hours, minutes)
    } else {
        "".to_string()
    }
}
fn target_hours_color(target_hours: &Option<u8>, duration: &chrono::Duration) -> String {
    if target_hours.is_some() {
        let hours_as_i64 = target_hours.unwrap() as i64;
        if &hours_as_i64 - 1 > duration.num_hours() {
            "Fr".to_string()
        } else if &hours_as_i64 < &duration.num_hours() {
            "Fg".to_string()
        } else {
            "Fy".to_string()
        }
    } else {
        "".to_string()
    }
}
fn format_weekly_hours(project: &types::Project) -> String {
    let weekly_hours = project.total_hours_for_current_week();
    if weekly_hours.is_zero() {
        "".to_string()
    } else {
        format!(
            "{} / {}",
            &format_duration(&weekly_hours),
            &format_duration(&project.daily_avg_for_current_week()),
        )
    }
}

fn format_project_title(project: &types::Project) -> String {
    if project.client.is_some() {
        format!("{} / {}", &project.client.clone().unwrap(), project.title)
    } else {
        project.title.to_string()
    }
}

fn format_targets(project_config: &ProjectConfig) -> String {
    if project_config.is_any_target_set() {
        format!(
            "{} / {} / {}",
            format_target_hour(project_config.target_daily_hours),
            format_target_hour(project_config.target_weekly_hours),
            format_target_hour(project_config.target_monthly_hours)
        )
    } else {
        "".to_string()
    }
}
fn format_target_hour(target: Option<u8>) -> String {
    match target {
        Some(target) => format!("{}h", target),
        None => "-".to_string(),
    }
}
