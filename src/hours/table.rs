//! Print time entries to terminal in table
use crate::hours::types::{self, TimeEntryCalculations};
use crate::settings::{ProjectConfig, ProjectConfigs};
use chrono::Duration;
use comfy_table::{presets::ASCII_NO_BORDERS, Attribute, Cell, Color, Table};

/// Prints given entries to terminal
pub fn print(time_entries: &types::TimeEntries, project_configs: &ProjectConfigs) {
    println!();
    let mut table = Table::new();
    table.load_preset(ASCII_NO_BORDERS).set_header(vec![
        header_cell("Project"),
        header_cell("Today"),
        header_cell("Current week / Daily AVG"),
        header_cell("Current month / Daily AVG"),
        header_cell("Target (day / week / month)"),
    ]);

    let total_hours_for_current_day = &time_entries.total_hours_for_current_day();

    for project in time_entries.uniq_projects() {
        let project_config = project_configs.get(&project).unwrap();
        table.add_row(vec![
            Cell::new(format_project_title(&project)),
            bold_cell(format_duration(total_hours_for_current_day)).fg(target_hours_color(
                &project_config.target_daily_hours,
                total_hours_for_current_day,
            )),
            Cell::new(format_weekly_hours(&project)),
            Cell::new(format_monthly_hours(&project)),
            Cell::new(format_targets(project_config)),
        ]);
    }

    table.add_row(vec![
        bold_cell("Total"),
        bold_cell(format_duration(total_hours_for_current_day)),
        Cell::new(""),
        bold_cell(format_duration(&time_entries.total_hours())),
        Cell::new(""),
    ]);

    println!("{table}");
}

fn header_cell<T: ToString>(content: T) -> Cell {
    bold_cell(content)
}

fn bold_cell<T: ToString>(content: T) -> Cell {
    Cell::new(content).add_attribute(Attribute::Bold)
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

fn target_hours_color(target_hours: &Option<u8>, duration: &chrono::Duration) -> Color {
    if target_hours.is_some() {
        let hours_as_i64 = target_hours.unwrap() as i64;
        if &hours_as_i64 - 1 > duration.num_hours() {
            Color::Red
        } else if hours_as_i64 <= duration.num_hours() {
            Color::Green
        } else {
            Color::Yellow
        }
    } else {
        Color::Reset
    }
}

fn format_weekly_hours(project: &types::Project) -> String {
    format_hours(
        &project.total_hours_for_current_week(),
        &project.daily_avg_for_current_week(),
    )
}

fn format_monthly_hours(project: &types::Project) -> String {
    format_hours(
        &project.total_hours(),
        &project.daily_avg_for_current_month(),
    )
}

fn format_hours(total_hours: &Duration, avg_hours: &Duration) -> String {
    if total_hours.is_zero() {
        "".to_string()
    } else {
        format!(
            "{} / {}",
            &format_duration(total_hours),
            &format_duration(avg_hours),
        )
    }
}

fn format_project_title(project: &types::Project) -> String {
    project
        .client
        .as_ref()
        .map(|c| format!("{} / {}", c.clone(), project.title))
        .unwrap_or_else(|| project.title.to_string())
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
    target
        .map(|hours| format!("{hours}h"))
        .unwrap_or_else(|| "-".to_string())
}
