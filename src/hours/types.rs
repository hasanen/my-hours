use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeEntry {
    pub description: String,
    pub project: String,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
    pub billable_amount_cents: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    pub entries: Vec<TimeEntry>,
}

impl TimeEntries {
    pub fn uniq_projects(&self) -> Vec<String> {
        let mut projects = HashSet::new();
        let mut projects_as_vec = Vec::new();

        for entry in self.entries.iter() {
            projects.insert(entry.project.to_string());
        }
        projects_as_vec = projects.iter().map(|a| a.to_string()).collect();
        projects_as_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        return projects_as_vec;
    }
}
