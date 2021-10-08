//! Store and load settings
use config;
use directories_next::ProjectDirs;
use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::path::Path;
use std::str::FromStr;

/// Load all settings
pub fn load() -> config::Config {
    let mut settings = config::Config::default();

    // Lin: /home/alice/.config/barapp
    // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
    // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App

    // settings
    //     // Add in `./Settings.toml`
    //     .merge(config::File::with_name("Settings"))
    //     .unwrap()
    let settings_path = settings_path().expect("Couldn't load settings");
    settings
        .merge(config::File::with_name(&settings_path))
        .expect("Couldn't load settings");

    settings
}

/// Store general setting
pub fn store_value(key: &str, value: &str) {
    let mut settings = load();
    settings
        .set(key, value)
        .expect("There was a problem on saving the setting");
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
