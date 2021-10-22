use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub trait TimeEntryCalculations {
    fn entries(&self) -> &Vec<TimeEntry>;
    fn total_hours(&self) -> Duration {
        let durations = self
            .entries()
            .iter()
            .map(|entry| entry.duration())
            .collect();
        return Self::sum(&durations);
    }
    fn total_hours_for_current_day(&self) -> Duration {
        let durations = self
            .entries()
            .iter()
            .filter(|entry| entry.is_for_current_date())
            .map(|entry| entry.duration())
            .collect();
        return Self::sum(&durations);
    }

    fn sum(durations: &Vec<Duration>) -> Duration {
        return durations
            .iter()
            .fold(Duration::minutes(0), |total_dur, entry| {
                total_dur.checked_add(&entry).unwrap()
            });
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TimeEntry {
    pub description: String,
    pub project: String,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
    pub billable_amount_cents: usize,
}

impl TimeEntry {
    pub fn duration(&self) -> Duration {
        return self.end.unwrap().signed_duration_since(self.start.unwrap());
    }
    pub fn is_for_current_date(&self) -> bool {
        return self.start.unwrap().date() == Local::today();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    pub entries: Vec<TimeEntry>,
}
impl TimeEntryCalculations for TimeEntries {
    fn entries(&self) -> &Vec<TimeEntry> {
        return &self.entries;
    }
}
impl TimeEntries {
    pub fn uniq_projects(&self) -> Vec<Project> {
        let mut projects = HashSet::new();

        for entry in self.entries.iter() {
            let project = Project {
                title: entry.project.to_string(),
                entries: self.entries_for_project(&entry.project),
            };
            projects.insert(project);
        }
        let mut projects_as_vec: Vec<Project> = projects.iter().map(|a| a.clone()).collect();
        projects_as_vec.sort_by(|a, b| a.title.partial_cmp(&b.title).unwrap());
        return projects_as_vec;
    }

    fn entries_for_project(&self, project_title: &str) -> Vec<TimeEntry> {
        return self
            .entries
            .iter()
            .filter(|entry| entry.project.eq(project_title))
            .cloned()
            .collect();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Project {
    pub title: String,
    pub entries: Vec<TimeEntry>,
}
impl TimeEntryCalculations for Project {
    fn entries(&self) -> &Vec<TimeEntry> {
        return &self.entries;
    }
}
