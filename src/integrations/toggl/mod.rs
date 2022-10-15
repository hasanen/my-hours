//! Toggl hours tracking - <https://track.toggl.com>
//!
//! API docs: <https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md>

use crate::hours::{self, ui};
use crate::settings;
use chrono::{Date, Local};
use serde::{Deserialize, Serialize};
mod api;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub key: String,
    pub workspaces: Vec<Workspace>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
}

/// Setup a new toggl integration. You will need an API key, which you can get from your profile page <https://track.toggl.com/profile>
pub fn setup() {
    let api_key = ui::ask_input::<String>("Toggl API key:");

    if api_key.is_none() {
        println!("You need to provide Toggl API key");
        std::process::exit(2);
    }

    let api_key = api_key.unwrap();

    let workspaces = api::get_workspaces(&api_key)
        .iter()
        .map(|w| Workspace {
            id: w.id,
            name: w.name.to_string(),
        })
        .collect();

    let toggl = Config {
        key: api_key,
        workspaces,
    };

    let mut config = settings::load();
    if config.toggl.is_none() {
        config.toggl = Some(vec![toggl])
    } else {
        let mut toggls = config.toggl.unwrap();
        toggls.push(toggl);
        config.toggl = Some(toggls);
    }

    match settings::save(&config) {
        Ok(_config) => println!("New toggle configuration saved!"),
        Err(err) => println!("Couldn't add new toggl configuration: {}", err),
    }
}

pub fn time_entries_for_dates(
    config: &Config,
    start_date: &Date<Local>,
    end_date: &Date<Local>,
) -> Vec<hours::types::TimeEntry> {
    let workspace_ids: Vec<usize> = config.workspaces.iter().map(|w| w.id).collect();

    let time_entries: Vec<Vec<api::types::TimeEntry>> = workspace_ids
        .iter()
        .map(|workspace_id| api::get_time_entries(workspace_id, start_date, end_date, &config.key))
        .collect();

    return time_entries
        .concat()
        .iter()
        .map(|api_entry| hours::types::TimeEntry {
            description: String::from(api_entry.description.as_ref().unwrap_or(&String::from(""))),
            client: api_entry.client.clone(),
            project: String::from(api_entry.project.as_ref().unwrap()),
            start: api_entry.start,
            end: api_entry.end,
            billable_amount_cents: (api_entry.billable.unwrap_or(0.0) * 100.0) as usize,
        })
        .collect();
}
