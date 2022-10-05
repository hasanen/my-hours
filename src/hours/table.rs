//! Print time entries to terminal in table
use crate::hours::types::{self, TimeEntryCalculations};
use crate::settings::{ProjectConfig, ProjectConfigs};
use chrono::Duration;
use comfy_table::{presets::ASCII_NO_BORDERS, Attribute, Cell, Color, Table};

/// Generate ASCII table of entries
pub fn generate_table(
    time_entries: &types::TimeEntries,
    project_configs: &ProjectConfigs,
) -> Table {
    let mut table = Table::new();
    table.load_preset(ASCII_NO_BORDERS).set_header(vec![
        header_cell("Project"),
        header_cell("Today"),
        header_cell("Current week / Daily AVG"),
        header_cell("Current month / Daily AVG"),
        header_cell("Target (day / week / month)"),
    ]);

    for project in time_entries.uniq_projects() {
        let project_config = project_configs.get(&project).unwrap();
        table.add_row(vec![
            Cell::new(format_project_title(&project)),
            bold_cell(format_duration(&project.total_hours_for_current_day())).fg(
                target_hours_color(
                    &project_config.target_daily_hours,
                    &project.total_hours_for_current_day(),
                ),
            ),
            Cell::new(format_weekly_hours(&project)),
            Cell::new(format_monthly_hours(&project)),
            Cell::new(format_targets(project_config)),
        ]);
    }

    table.add_row(vec![
        bold_cell("Total"),
        bold_cell(format_duration(&time_entries.total_hours_for_current_day())),
        bold_cell(format_hours(
            &time_entries.total_hours_for_current_week(),
            &time_entries.daily_avg_for_current_week(),
        )),
        bold_cell(format_duration(&time_entries.total_hours())),
        Cell::new(""),
    ]);

    return table;
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

#[cfg(test)]
mod tests {

    mod colors {
        use super::super::*;

        #[test]
        fn it_gives_hours_color() {
            assert_eq!(target_hours_color(&None, &Duration::hours(0)), Color::Reset);
            assert_eq!(
                target_hours_color(&Some(0), &Duration::hours(0)),
                Color::Green
            );
            assert_eq!(
                target_hours_color(&Some(1), &Duration::hours(3)),
                Color::Green
            );
            assert_eq!(
                target_hours_color(&Some(3), &Duration::hours(1)),
                Color::Red
            );
            assert_eq!(
                target_hours_color(&Some(2), &Duration::hours(1)),
                Color::Yellow
            );
        }
    }

    mod formats {
        use super::super::*;
        use chrono::Local;
        #[test]
        fn it_formats_weekly_monthly_hours() {
            let project = types::Project {
                client: None,
                title: "my-hours".to_string(),
                key: "".to_string(),
                entries: vec![],
            };
            assert_eq!(format_monthly_hours(&project), "");
            assert_eq!(format_weekly_hours(&project), "");

            let mut project = types::Project {
                client: None,
                title: "my-hours".to_string(),
                key: "".to_string(),
                entries: vec![],
            };

            let today = Local::today();
            let start = today.and_hms_milli(1, 0, 00, 0);
            let end = today.and_hms_milli(13, 0, 0, 0);

            project.entries.push(types::TimeEntry {
                description: "Monday".to_string(),
                client: None,
                project: "my-hours".to_string(),
                start: Some(start),
                end: Some(end),
                billable_amount_cents: 1,
            });

            assert_eq!(format_monthly_hours(&project), " 12h  0m /  12h  0m");
            assert_eq!(format_weekly_hours(&project), " 12h  0m /  12h  0m");
        }

        #[test]
        fn it_formats_hours() {
            assert_eq!(
                format_hours(&Duration::hours(1), &Duration::hours(2)),
                "  1h  0m /   2h  0m"
            );
            assert_eq!(format_hours(&Duration::hours(0), &Duration::hours(2)), "");
            assert_eq!(
                format_hours(&Duration::hours(1), &Duration::hours(0)),
                "  1h  0m / "
            );
        }

        #[test]
        fn it_formats_project_title() {
            let project_no_client = types::Project {
                client: None,
                title: "my-hours".to_string(),
                key: "".to_string(),
                entries: vec![],
            };
            assert_eq!(format_project_title(&project_no_client), "my-hours");

            let project_with_client = types::Project {
                client: Some("hasanen".to_string()),
                title: "my-hours".to_string(),
                key: "".to_string(),
                entries: vec![],
            };
            assert_eq!(
                format_project_title(&project_with_client),
                "hasanen / my-hours"
            );
        }

        #[test]
        fn it_formats_targets() {
            let config = ProjectConfig {
                target_daily_hours: Some(1),
                target_weekly_hours: Some(2),
                target_monthly_hours: Some(3),
            };

            assert_eq!(format_targets(&config), "1h / 2h / 3h");

            let config_without_targets = ProjectConfig {
                target_daily_hours: None,
                target_weekly_hours: None,
                target_monthly_hours: None,
            };
            assert_eq!(format_targets(&config_without_targets), "");
        }

        #[test]
        fn it_formats_target_hours() {
            assert_eq!(format_target_hour(Some(1)), "1h");
            assert_eq!(format_target_hour(None), "-");
        }
    }

    mod formats_table {
        use super::super::*;
        use chrono::Local;
        use digest::Digest;
        use sha2::Sha256;
        use std::collections::HashMap;

        #[test]
        fn formats_table_with_one_project_work_done_current_day_no_targets() {
            let project_name = "Project".to_string();
            let time_entries = types::TimeEntries {
                entries: [types::TimeEntry {
                    description: "Description".to_string(),
                    client: None,
                    project: project_name.clone(),
                    billable_amount_cents: 0,
                    start: Some(Local::now()),
                    end: Some(
                        Local::now()
                            .checked_add_signed(Duration::minutes(60))
                            .unwrap(),
                    ),
                }]
                .to_vec(),
            };
            let mut hasher = Sha256::default();
            hasher.update(&project_name);
            let project_key = format!("{:x}", &hasher.finalize());

            let project_configs = ProjectConfigs {
                configs: HashMap::from([(
                    project_key,
                    ProjectConfig {
                        target_daily_hours: None,
                        target_weekly_hours: None,
                        target_monthly_hours: None,
                    },
                )]),
            };
            let mut hours_table = generate_table(&time_entries, &project_configs);
            hours_table.force_no_tty();
            let expected = "
 Project | Today    | Current week / Daily AVG | Current month / Daily AVG | Target (day / week / month) 
=========================================================================================================
 Project |   1h  0m |   1h  0m /   1h  0m      |   1h  0m /   1h  0m       |                             
---------+----------+--------------------------+---------------------------+-----------------------------
 Total   |   1h  0m |                          |   1h  0m                  |                             ";

            assert_eq!("\n".to_string() + &hours_table.to_string(), expected)
        }
    }
}
