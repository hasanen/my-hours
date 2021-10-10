//! Toggl hours tracking - <https://track.toggl.com>
//!
//! API docs: <https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md>

use crate::hours;
use crate::settings;
use chrono::{offset::TimeZone, Date, Datelike, Local, NaiveDate};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    key: String,
    pub workspaces: Vec<Workspace>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
}

/// Setup a new toggl integration. You will need an API key, which you can get from your profile page <https://track.toggl.com/profile>
pub fn setup() {
    println!("Toggl API key:");
    let api_key = input::<String>().get();

    let workspaces = api::get_workspaces(&api_key)
        .iter()
        .map(|w| Workspace {
            id: w.id,
            name: w.name.to_string(),
        })
        .collect();

    let toggl = Config {
        key: api_key,
        workspaces: workspaces,
    };

    let mut config = settings::load();
    if config.toggl.is_none() {
        config.toggl = Some(vec![toggl])
    } else {
        let mut toggls = config.toggl.unwrap();
        toggls.push(toggl);
        config.toggl = Some(toggls);
    }

    match settings::save(config) {
        Ok(_config) => println!("New toggle configuration saved!"),
        Err(err) => println!("Couldn't add new toggl configuration: {}", err),
    }
}

pub fn time_entries_for_month(config: &Config, date: Date<Local>) -> Vec<hours::types::TimeEntry> {
    let workspace_ids: Vec<usize> = config.workspaces.iter().map(|w| w.id).collect();
    let year = date.year();
    let month = date.month();
    let start_date = NaiveDate::from_ymd(year, month, 1);
    let end_date = NaiveDate::from_ymd(year, month + 1, 1).pred();

    let time_entries: Vec<Vec<api::types::TimeEntry>> = workspace_ids
        .iter()
        .map(|workspace_id| {
            return api::get_time_entries(
                workspace_id,
                &Local.from_local_date(&start_date).unwrap(),
                &Local.from_local_date(&end_date).unwrap(),
                &config.key,
            );
        })
        .collect();

    return time_entries
        .concat()
        .iter()
        .map(|api_entry| hours::types::TimeEntry {
            description: String::from(api_entry.description.as_ref().unwrap_or(&String::from(""))),
            project: [api_entry.client.as_ref(), api_entry.project.as_ref()]
                .iter()
                .filter(|string| string.is_some())
                .map(|string| String::from(string.unwrap()))
                .collect::<Vec<String>>()
                .join(" / "),
            start: api_entry.start,
            end: api_entry.end,
        })
        .collect();
}
