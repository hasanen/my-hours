//! Toggl hours tracking - <https://track.toggl.com>
//!
//! API docs: <https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md>
use crate::integrations::Integration;
use crate::settings;
use read_input::prelude::*;
use std::collections::HashMap;

static NAME: &str = "toggl";

/// Setup a new toggl integration. You will need an API key, which you can get from your profile page <https://track.toggl.com/profile>
pub fn setup() {
    println!("Toggl API key:");
    // let api_key = input::<String>().get();
    let api_key = "keykeykey";
    settings::store_value(&key("api_key"), api_key);
    println!(
        "{:?}",
        settings::load()
            .try_into::<HashMap<String, String>>()
            .unwrap()
    );
}

fn key(key: &str) -> String {
    return format!("{}.{}", NAME, key);
}
