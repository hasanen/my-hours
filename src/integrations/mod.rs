use crate::hours::types::{TimeEntries, TimeEntry};
use crate::settings;
use chrono::Local;
use structopt::StructOpt;

pub mod toggl;

#[derive(StructOpt, Debug)]
pub enum Action {
    #[structopt(name = "setup")]
    /// Setup new integration
    Setup {
        #[structopt(subcommand)]
        integration: Integration,
    },
}

#[derive(StructOpt, Debug)]
pub enum Integration {
    #[structopt(name = "toggl")]
    TogglIntegration,
}

pub fn execute(action: &Action) {
    match action {
        Action::Setup { integration } => match integration {
            Integration::TogglIntegration => toggl::setup(),
        },
    }
}

/// Loop over integrations and get time entries for current month
pub fn get_monthly_time_entries() -> TimeEntries {
    let settings = settings::load();

    let entries: Vec<Vec<TimeEntry>> = settings
        .toggl
        .unwrap()
        .iter()
        .map(|toggl_config| {
            let toggl_entries = toggl::time_entries_for_month(toggl_config, Local::today());
            return toggl_entries;
        })
        .collect();

    let time_entries = TimeEntries {
        entries: entries.concat(),
    };

    return time_entries;
}
