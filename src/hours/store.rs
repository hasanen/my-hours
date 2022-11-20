//! File storage module.
use crate::hours::types;
use crate::settings;
use std::fs;

static HOURS_FILENAME: &str = "hours.toml";

/// Store time entries to file on disk
pub fn save(time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
    let monthly_hours_path = settings::app_path(HOURS_FILENAME)
        .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
    let toml = toml::to_string(&time_entries).unwrap();
    fs::write(monthly_hours_path, toml)
}

/// Load time entries from file on disk
pub fn load() -> types::TimeEntries {
    let monthly_hours_path = settings::app_path(HOURS_FILENAME)
        .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
    let hours_str = fs::read_to_string(monthly_hours_path).expect("Couldn't load settings");
    if hours_str.trim().is_empty() {
        types::TimeEntries {
            entries: Vec::new(),
        }
    } else {
        toml::from_str(&hours_str).unwrap()
    }
}
