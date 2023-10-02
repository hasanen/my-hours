use crate::integrations;
use crate::settings;
use crate::string_types::FilePath;
use chrono::Local;
use std::collections::HashMap;
mod table;
mod store;
use store::{Store, DiskStore};
pub mod types;
pub mod ui;



static HOURS_FILENAME: &str = "hours.toml";

/// Show your current monthly progress
pub fn show_monthly_hours() {
    let config = settings::load();
    let mut time_entries = get_disk_store().load();

    if refresh_required(&config) {
        time_entries = refresh_hours();
    }

    let project_configs = ensure_and_get_projects_configs(config, &time_entries.uniq_projects());
    let table = table::generate_table(&time_entries, &project_configs);
    println!("\n{table}");
}

/// Refresh hours for current month from the integrations
pub fn refresh_all() {
    refresh_hours();
}

/// Print some basic info
pub fn print_info() {
    println!("Settings folder: {}", settings::settings_path().unwrap())
}

fn refresh_hours() -> types::TimeEntries {
    let time_entries = integrations::get_monthly_time_entries();
    match get_disk_store().save(&time_entries) {
        Ok(_) => println!("Updated monthly hours from integrations"),
        Err(err) => println!("Error occured during refreshing hours: {}", err),
    }
    settings::hours_refreshed();
    time_entries
}

fn refresh_required(config: &settings::Config) -> bool {
    let treshold_minutes = match config.refresh_treshold {
        None => return true,
        Some(minutes) => minutes,
    };
    match config.refreshed_at {
        None => true,
        Some(timestamp) => {
            Local::now().signed_duration_since(timestamp).num_minutes() >= treshold_minutes as i64
        }
    }
}

fn ensure_and_get_projects_configs(
    config: settings::Config,
    projects_from_entries: &[types::Project],
) -> settings::ProjectConfigs {
    let mut config_changed = false;
    let mut project_configs = match &config.project_configs {
        Some(project_configs) => project_configs.clone(),
        None => settings::ProjectConfigs {
            configs: HashMap::new(),
        },
    };
    for project in projects_from_entries {
        let project_config = project_configs.get(project);
        if project_config.is_none() {
            let target_daily_hours = ask_target(&format!(
                "What is your target daily hours for {}?",
                project.title
            ));
            let target_weekly_hours = ask_target(&format!(
                "What is your target weekly hours for {}?",
                project.title
            ));
            let target_monthly_hours = ask_target(&format!(
                "What is your target monthly hours for {}?",
                project.title
            ));
            let new_config = settings::ProjectConfig {
                target_daily_hours,
                target_weekly_hours,
                target_monthly_hours,
            };
            config_changed = true;
            project_configs
                .configs
                .insert(project.key.clone(), new_config);
        };
    }
    if config_changed {
        let updated_config = settings::Config {
            project_configs: Some(project_configs.clone()),
            ..config
        };
        match settings::save(&updated_config) {
            Ok(_) => println!("New project configs saved"),
            Err(err) => println!("Error occured during saving new project configs: {}", err),
        }
    }
    project_configs
}

fn ask_target(question: &str) -> Option<u8> {
    match ui::ask_input::<u8>(question) {
        Some(num) if num > 0 => Some(num),
        _ => None,
    }
}


fn get_disk_store() -> DiskStore {
    let filepath_str =  settings::app_path(HOURS_FILENAME)
    .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
    DiskStore{
        path: FilePath(filepath_str)
    }
}