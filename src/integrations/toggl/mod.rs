//! Toggl hours tracking - <https://track.toggl.com>
//!
//! API docs: <https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md>

use crate::settings;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    key: String,
    workspaces: Vec<Workspace>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    id: usize,
    name: String,
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
