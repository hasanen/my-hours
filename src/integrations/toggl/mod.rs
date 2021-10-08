//! Toggl hours tracking - <https://track.toggl.com>
//!
//! API docs: <https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md>
use crate::settings;
use read_input::prelude::*;
use serde::{Deserialize, Serialize};

static NAME: &str = "toggl";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    key: String,
}

/// Setup a new toggl integration. You will need an API key, which you can get from your profile page <https://track.toggl.com/profile>
pub fn setup() {
    println!("Toggl API key:");
    // let api_key = input::<String>().get();
    let api_key = "keykeykey";

    let toggl = Config {
        key: String::from(api_key),
    };

    let mut config = settings::load();

    println!("{:?}", config);
}

fn key(key: &str) -> String {
    return format!("{}.{}", NAME, key);
}
