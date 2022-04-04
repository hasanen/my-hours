//! Store and load settings
use crate::hours::types::Project;
use crate::integrations::toggl::Config as TogglConfig;
use chrono::{DateTime, Local};
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::path::Path;
use std::str::FromStr;

static CONFIG_FILENAME: &str = "settings.toml";

/// Configs for the app
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Refresh time entries from integrations if latest entry end time is older than current time + this treshold. Use minutes
    pub refresh_treshold: Option<usize>,
    /// When hours were refreshed last time
    pub refreshed_at: Option<DateTime<Local>>,
    /// Toggl configurations
    pub toggl: Option<Vec<TogglConfig>>,
    /// Settings for projects
    pub project_configs: Option<ProjectConfigs>,
}

/// Configs for the projects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfigs {
    /// Existing configs
    pub configs: HashMap<String, ProjectConfig>,
}

/// Single config project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    /// Target daily hours
    pub target_daily_hours: Option<u8>,
    /// Target weekly hours
    pub target_weekly_hours: Option<u8>,
    /// Target monthly hours
    pub target_monthly_hours: Option<u8>,
}

/// Load all settings
pub fn load() -> Config {
    let settings_path = settings_path().expect("Couldn't load settings");
    let settings_str = fs::read_to_string(settings_path).expect("Couldn't load settings");
    toml::from_str(&settings_str).unwrap()
}

/// Store config to filestystem
pub fn save(config: &Config) -> Result<(), std::io::Error> {
    let settings_path =
        settings_path().unwrap_or_else(|| panic!("Failed to locate {}", CONFIG_FILENAME));
    let toml = toml::to_string(&config).unwrap();
    fs::write(settings_path, toml)
}

/// Mark hours as refreshed
pub fn hours_refreshed() {
    let mut settings = load();
    settings.refreshed_at = Some(Local::now());
    match save(&settings) {
        Ok(_) => {}
        Err(err) => println!("Error occured during refreshing hours: {}", err),
    };
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
            File::create(&settings_file_path)
                .unwrap_or_else(|_| panic!("Couldn't create settings file {}", settings_file_path));
        }
        return Some(settings_file_path);
    };
    None
}
/// Path for settings
pub fn settings_path() -> Option<String> {
    app_path(CONFIG_FILENAME)
}

impl Config {
    /// Get config for project
    pub fn set_project_configs(mut self, project_configs: ProjectConfigs) {
        self.project_configs = Some(project_configs)
    }
}

impl ProjectConfigs {
    /// Get config for project
    pub fn get(&self, project: &Project) -> Option<&ProjectConfig> {
        self.configs.get(&project.key)
    }
}

impl ProjectConfig {
    /// Returns true if any of daily, weekly or monthly target is set
    pub fn is_any_target_set(&self) -> bool {
        self.target_daily_hours.is_some()
            || self.target_weekly_hours.is_some()
            || self.target_monthly_hours.is_some()
    }
}
