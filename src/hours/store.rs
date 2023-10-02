//! File storage module.
use crate::hours::types;
use crate::settings;
use crate::string_types::FilePath;
use std::fs;


pub trait Store {
    fn save(&self, time_entries: &types::TimeEntries) -> Result<(), std::io::Error>;
    fn load(&self) -> types::TimeEntries;
}
pub struct DiskStore {
    pub path: FilePath,
}

impl Store for DiskStore {
    fn save(&self, time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
        // let monthly_hours_path = settings::app_path(HOURS_FILENAME)
        //     .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
        // let toml = toml::to_string(&time_entries).unwrap();
        // fs::write(monthly_hours_path, toml)
        Ok(())
    }

        fn load(&self) -> types::TimeEntries {
        let hours_str = fs::read_to_string(self.path.as_str()).expect("Couldn't load hours");
        if hours_str.trim().is_empty() {
            types::TimeEntries {
                entries: Vec::new(),
            }
        } else {
            toml::from_str(&hours_str).unwrap()
        }
    }
}

// /// Store time entries to file on disk
// pub fn save(time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
//     let monthly_hours_path = settings::app_path(HOURS_FILENAME)
//         .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
//     let toml = toml::to_string(&time_entries).unwrap();
//     fs::write(monthly_hours_path, toml)
// }

// /// Load time entries from file on disk
// pub fn load() -> types::TimeEntries {
//     let monthly_hours_path = settings::app_path(HOURS_FILENAME)
//         .unwrap_or_else(|| panic!("Failed to locate {}", HOURS_FILENAME));
//     let hours_str = fs::read_to_string(monthly_hours_path).expect("Couldn't load settings");
//     if hours_str.trim().is_empty() {
//         types::TimeEntries {
//             entries: Vec::new(),
//         }
//     } else {
//         toml::from_str(&hours_str).unwrap()
//     }
// }

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::string_types::{FilePath, Description, ProjectName};
    use chrono::Duration;
    use std::fs::File;

    #[test]
    fn saves_entries_into_file() {
        let filepath = FilePath("/tmp/test.toml".to_string());
        File::create(&filepath.as_str())?;
        let store = DiskStore {
            path: FilePath("/tmp/test.toml".to_string()),
        };
        let time_entries = types::TimeEntries {
            entries: [types::TimeEntry {
                description: Description("Description".to_string()),
                client: None,
                project: ProjectName("TestProject".to_string()),
                billable_amount_cents: 0,
                start: Some(Local::now()),
                end: Some(
                    Local::now()
                        .checked_add_signed(Duration::minutes(60))
                        .unwrap(),
                ),
            }]
            .to_vec()
        };
        store.save(&time_entries).unwrap();
        let loaded_entries = store.load();
        assert_eq!(loaded_entries.entries.len(), 1);
    }
}
