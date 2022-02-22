use crate::dates;
use crate::hours::types::{TimeEntries, TimeEntry};
use crate::settings;
use chrono::Local;
use clap::{Parser};

pub mod toggl;

#[derive(Parser, Debug)]
pub enum Action {
    /// Setup new integration
    Setup {
        #[clap(subcommand)]
        integration: Integration,
    },
}

#[derive(Parser, Debug, Clone)]
pub enum Integration {
    #[clap(name = "toggl")]
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
    let (start_date, end_date) = dates::month_first_and_last_dates(&Local::today());

    let entries: Vec<Vec<TimeEntry>> = match settings.toggl {
        Some(toggl) => toggl
            .iter()
            .map(|toggl_config| {
                
                toggl::time_entries_for_dates(toggl_config, &start_date, &end_date)
            })
            .collect(),
        None => Vec::new(),
    };

    

    TimeEntries {
        entries: entries.concat(),
    }
}
