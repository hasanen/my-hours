use crate::integrations;
use crate::settings;
use chrono::Local;
mod store;
mod table;
pub mod types;

/// Show your current monthly progress
pub fn show_monthly_hours() {
    let mut time_entries = store::load();
    if refresh_required() {
        time_entries = refresh_hours();
    }

    table::print(&time_entries)
}

/// Refresh hours for current month from the integrations
pub fn refresh_all() {
    refresh_hours();
}

fn refresh_hours() -> types::TimeEntries {
    let time_entries = integrations::get_monthly_time_entries();
    match store::save(&time_entries) {
        Ok(_) => println!("Updated monthly hours from integrations"),
        Err(err) => println!("Error occured during refreshing hours: {}", err),
    }
    settings::hours_refreshed();
    return time_entries;
}

fn refresh_required() -> bool {
    let settings = settings::load();
    let treshold_minutes;
    match settings.refresh_treshold {
        None => return true,
        Some(minutes) => treshold_minutes = minutes,
    }
    match settings.refreshed_at {
        None => return true,
        Some(timestamp) => {
            return Local::now().signed_duration_since(timestamp).num_minutes()
                >= treshold_minutes as i64;
        }
    }
}
