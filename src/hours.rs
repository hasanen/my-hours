use crate::hours::types::TimeEntryCalculations;
use crate::integrations;
use crate::settings;
use chrono::Local;
mod store;
mod table;
pub mod types;
pub mod ui;

/// Show your current monthly progress
pub fn show_monthly_hours() {
    let mut config = settings::load();
    let mut time_entries = store::load();
    if refresh_required(&config) {
        time_entries = refresh_hours();
    }
    if None == config.hours {
        let hours = ui::ask_input::<u8>(&"What is your target daily hours?");
        config.hours = Some(hours);
        match settings::save(&config) {
            Ok(_) => {}
            Err(err) => println!("Error occured during saving target hours: {}", err),
        }
    }

    let common_hours = types::CommonHours {
        target_daily_hours: &config.hours.unwrap(),
        monthly_work_days: &16,
        monthly_work_days_used: &17,
        total_hours: &time_entries.total_hours(),
    };
    table::print(&time_entries, &common_hours);
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

fn refresh_required(config: &settings::Config) -> bool {
    let treshold_minutes;
    match config.refresh_treshold {
        None => return true,
        Some(minutes) => treshold_minutes = minutes,
    }
    match config.refreshed_at {
        None => return true,
        Some(timestamp) => {
            return Local::now().signed_duration_since(timestamp).num_minutes()
                >= treshold_minutes as i64;
        }
    }
}
