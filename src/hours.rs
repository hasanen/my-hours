use crate::integrations;
use crate::settings;
use std::fs::{self, DirBuilder, File};
pub mod types;

static HOURS_FILENAME: &str = "hours.toml";

/// Show your current monthly progress
pub fn show_monthly_hours() {
    refresh_all()
}

/// Refresh hours for current month from the integrations
pub fn refresh_all() {
    // 1. tarkista onko tarvetta synkata
    // 1.a synkkaa jos edellisestä synkista on yli 3 tuntia (tää vois olla asetuksissa?)
    // 2. hae kuukauden tunnit
    // 3. printtaile ne ruudulle
    println!("refresh");
    let time_entries = integrations::get_monthly_time_entries();
    save_monthly_entries(&time_entries);
    println!("{:?}", time_entries.entries.len());
}

fn save_monthly_entries(time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
    let monthly_hours_path =
        settings::app_path(&HOURS_FILENAME).expect(&format!("Failed to locate {}", HOURS_FILENAME));
    let toml = toml::to_string(&time_entries).unwrap();
    return fs::write(monthly_hours_path, toml);
}
