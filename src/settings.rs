//! Store and load settings
use crate::integrations::toggl::Config as TogglConfig;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, DirBuilder, File};
use std::path::Path;
use std::str::FromStr;

static CONFIG_FILENAME: &str = "settings.toml";

/// Configs for the app
/// - Toggl configs
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Target daily hours
    pub hours: Option<u8>,
    /// Refresh time entries from integrations if latest entry end time is older than current time + this treshold. Use minutes
    pub refresh_treshold: Option<usize>,
    /// Toggl configurations
    pub toggl: Option<Vec<TogglConfig>>,
}

/// Load all settings
pub fn load() -> Config {
    let settings_path = settings_path().expect("Couldn't load settings");
    let settings_str = fs::read_to_string(settings_path).expect("Couldn't load settings");
    return toml::from_str(&settings_str).unwrap();
}
/// Store config to filestystem
pub fn save(config: Config) -> Result<(), std::io::Error> {
    let settings_path = settings_path().expect(&format!("Failed to locate {}", CONFIG_FILENAME));
    let toml = toml::to_string(&config).unwrap();
    return fs::write(settings_path, toml);
}
/// Get path to a file in app's folder. If file doesn't exist, it will be created
pub fn app_path(file: &str) -> Option<String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Piece Of Code", "Hours") {
        if !proj_dirs.config_dir().exists() {
            DirBuilder::new()
                .recursive(true)
                .create(proj_dirs.config_dir())
                .unwrap()
        }
        let folder = proj_dirs.config_dir().to_str().unwrap();
        let settings_file_path = format!("{}/{}", String::from_str(folder).unwrap(), file);
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
fn settings_path() -> Option<String> {
    app_path(&CONFIG_FILENAME)
}
