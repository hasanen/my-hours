use chrono::{offset::TimeZone, Date, DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
use digest::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashSet;
use std::str;

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

    fn total_hours_for_current_week(&self) -> Duration {
        let dates = self.dates_from_monday();
        let durations = self
            .entries()
            .iter()
            .filter(|entry| dates.contains(&entry.start.unwrap().date()))
            .map(|entry| entry.duration())
            .collect();
        return Self::sum(&durations);
    }
    fn daily_avg_for_current_week(&self) -> Duration {
        let working_days = self.current_week_work_days().len() as i64;
        if working_days > 0 {
            let total_minutes = self.total_hours_for_current_week().num_minutes();
            let minutes_per_day = total_minutes / working_days;
            Duration::minutes(minutes_per_day)
        } else {
            Duration::minutes(0)
        }
    }

    fn sum(durations: &Vec<Duration>) -> Duration {
        return durations
            .iter()
            .fold(Duration::minutes(0), |total_dur, entry| {
                total_dur.checked_add(&entry).unwrap()
            });
    }

    fn dates_from_monday(&self) -> Vec<Date<Local>> {
        let current_date = Local::today();
        let monday = NaiveDate::from_isoywd(
            current_date.year(),
            current_date.iso_week().week(),
            Weekday::Mon,
        );
        monday
            .iter_days()
            .take(current_date.weekday().number_from_monday() as usize)
            .enumerate()
            .map(|d| Local.from_local_date(&d.1).unwrap())
            .collect()
    }

    fn current_week_work_days(&self) -> HashSet<Date<Local>> {
        let dates_from_monday = self.dates_from_monday();
        let mut working_dates = HashSet::new();
        for entry in self
            .entries()
            .iter()
            .filter(|entry| dates_from_monday.contains(&entry.start.unwrap().date()))
        {
            working_dates.insert(entry.start.unwrap().date());
        }

        return working_dates;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TimeEntry {
    pub description: String,
    pub client: Option<String>,
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
            let mut hasher = Sha256::default();
            hasher.update(entry.project.to_string());
            let finalized_hash = format!("{:x}", &hasher.finalize());
            let project = Project {
                title: entry.project.to_string(),
                client: entry.client.clone(),
                key: finalized_hash,
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
    pub client: Option<String>,
    pub title: String,
    pub key: String,
    pub entries: Vec<TimeEntry>,
}

impl TimeEntryCalculations for Project {
    fn entries(&self) -> &Vec<TimeEntry> {
        return &self.entries;
    }
}

#[derive(Debug)]
pub struct CommonHours<'a> {
    pub target_daily_hours: &'a u8,
}
