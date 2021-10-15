use crate::integrations;
use crate::settings;
use chrono::{DateTime, Local};
use std::fs::{self, DirBuilder, File};
pub mod types;

static HOURS_FILENAME: &str = "hours.toml";

/// Show your current monthly progress
pub fn show_monthly_hours() {
    let mut time_entries = load_monthly_entries();
    println!("{:?}", time_entries.entries.len());
    if refresh_required(&time_entries) {
        time_entries = refresh_hours();
    }
    println!("{:?}", time_entries.entries.len());
}

/// Refresh hours for current month from the integrations
pub fn refresh_all() {
    refresh_hours();
}

fn refresh_hours() -> types::TimeEntries {
    let time_entries = integrations::get_monthly_time_entries();
    match save_monthly_entries(&time_entries) {
        Ok(_) => println!("Updated monthly hours from integrations"),
        Err(err) => println!("Error occured during refreshing hours: {}", err),
    }
    return time_entries;
}

fn save_monthly_entries(time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
    let monthly_hours_path =
        settings::app_path(&HOURS_FILENAME).expect(&format!("Failed to locate {}", HOURS_FILENAME));
    let toml = toml::to_string(&time_entries).unwrap();
    return fs::write(monthly_hours_path, toml);
}

fn load_monthly_entries() -> types::TimeEntries {
    let monthly_hours_path =
        settings::app_path(&HOURS_FILENAME).expect(&format!("Failed to locate {}", HOURS_FILENAME));
    let hours_str = fs::read_to_string(monthly_hours_path).expect("Couldn't load settings");
    return toml::from_str(&hours_str).unwrap();
}

fn refresh_required(time_entries: &types::TimeEntries) -> bool {
    let settings = settings::load();
    let treshold_minutes;
    match settings.refresh_treshold {
        None => return true,
        Some(minutes) => treshold_minutes = minutes,
    }
    let mut cloned_entries = time_entries.entries.to_vec();
    cloned_entries.sort_by(|a, b| a.end.partial_cmp(&b.end).unwrap());
    let minutes_from_last_entry = Local::now()
        .signed_duration_since(cloned_entries.last().unwrap().end.unwrap())
        .num_minutes();

    return minutes_from_last_entry >= treshold_minutes as i64;
}
