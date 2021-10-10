//! Store and load settings
use crate::integrations::toggl::Config as TogglConfig;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, DirBuilder, File};
use std::path::Path;
use std::str::FromStr;

/// Configs for the app
/// - Toggl configs
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Target daily hours
    pub hours: Option<u8>,
    /// Toggl configurations
    pub toggls: Option<Vec<TogglConfig>>,
}

impl Config {
    fn save() {
        println!("SAVEE");
    }
}
/// Load all settings
pub fn load() -> Config {
    let settings_path = settings_path().expect("Couldn't load settings");
    let settings_str = fs::read_to_string(settings_path).expect("Couldn't load settings");
    return toml::from_str(&settings_str).unwrap();
}

fn settings_path() -> Option<String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Piece Of Code", "Hours") {
        if !proj_dirs.config_dir().exists() {
            DirBuilder::new()
                .recursive(true)
                .create(proj_dirs.config_dir())
                .unwrap()
        }
        let folder = proj_dirs.config_dir().to_str().unwrap();
        let settings_file_path = format!("{}/settings.toml", String::from_str(folder).unwrap());
        if !Path::new(&settings_file_path).exists() {
            File::create(&settings_file_path).expect(&format!(
                "Couldn't create settings file {}",
                settings_file_path
            ));
        }
        return Some(settings_file_path);
    };
    None
}
