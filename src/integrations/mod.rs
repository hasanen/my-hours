use crate::hours::types::TimeEntry;
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
pub fn get_monthly_time_entries() -> Vec<TimeEntry> {
    let mut time_entries = Vec::new();
    let settings = settings::load();

    let time_entries =  settings.toggl.unwrap().iter().map(|toggl_config|  {
        let toggl_entries = toggl::time_entries_for_month(toggl_config, Local::today());
    })
    println!("{:?}", time_entries.concat().len());

    time_entries
}
