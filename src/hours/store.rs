//! Storage module
use crate::hours::types;
use crate::string_types::FilePath;
use std::fs;

/// Common store functionality
pub trait Store {
    fn save(&self, time_entries: &types::TimeEntries) -> Result<(), std::io::Error>;
    fn load(&self) -> types::TimeEntries;
}
/// Filesystem storage for hours
pub struct DiskStore {
    /// Path to the file where hours are stored in filesystem
    pub path: FilePath,
}

impl Store for DiskStore {
    fn save(&self, time_entries: &types::TimeEntries) -> Result<(), std::io::Error> {
        let toml = toml::to_string(&time_entries).unwrap();
        fs::write(self.path.as_str(), toml)
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

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::string_types::{Description, FilePath, ProjectName};
    use chrono::Duration;
    use std::fs::File;

    #[test]
    fn return_empty_list_of_entries_by_default() {
        let filepath = FilePath("/tmp/test.toml".to_string());
        File::create(filepath.as_str()).unwrap();
        let store = DiskStore {
            path: FilePath("/tmp/test.toml".to_string()),
        };

        let loaded_entries = store.load();

        assert_eq!(loaded_entries.entries.len(), 0);
    }
    #[test]
    fn saves_entries_into_file() {
        let filepath = FilePath("/tmp/test.toml".to_string());
        File::create(filepath.as_str()).unwrap();
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
            .to_vec(),
        };
        store.save(&time_entries).unwrap();
        let loaded_entries = store.load();
        assert_eq!(loaded_entries.entries.len(), 1);
    }
}
