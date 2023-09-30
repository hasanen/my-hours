use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
use digest::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashSet;
use std::str;
use crate::string_types::{Description, ClientName, ProjectName, ProjectHash};

pub trait TimeEntryCalculations {
    fn entries(&self) -> &Vec<TimeEntry>;
    fn total_hours(&self) -> Duration {
        let durations: Vec<Duration> = self
            .entries()
            .iter()
            .map(|entry| entry.duration())
            .collect();
        Self::sum(&durations)
    }
    fn total_hours_for_current_day(&self) -> Duration {
        let durations: Vec<Duration> = self
            .entries()
            .iter()
            .filter(|entry| entry.is_for_current_date())
            .map(|entry| entry.duration())
            .collect();
        Self::sum(&durations)
    }

    fn total_hours_for_current_week(&self) -> Duration {
        let dates = self.dates_from_monday();
        let durations: Vec<Duration> = self
            .entries()
            .iter()
            .filter(|entry| dates.contains(&entry.start.unwrap().date_naive()))
            .map(|entry| entry.duration())
            .collect();
        Self::sum(&durations)
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

    fn daily_avg_for_current_month(&self) -> Duration {
        let working_days = self.current_month_work_days().len() as i64;
        if working_days > 0 {
            let total_minutes = self.total_hours().num_minutes();
            let minutes_per_day = total_minutes / working_days;
            Duration::minutes(minutes_per_day)
        } else {
            Duration::minutes(0)
        }
    }

    fn sum(durations: &[Duration]) -> Duration {
        return durations
            .iter()
            .fold(Duration::minutes(0), |total_dur, entry| {
                total_dur.checked_add(entry).unwrap()
            });
    }

    fn dates_from_monday(&self) -> Vec<NaiveDate> {
        let current_date = Local::now().date_naive();
        let monday = NaiveDate::from_isoywd_opt(
            current_date.year(),
            current_date.iso_week().week(),
            Weekday::Mon,
        )
        .unwrap();
        monday
            .iter_days()
            .take(current_date.weekday().number_from_monday() as usize)
            .collect()
    }

    fn current_week_work_days(&self) -> HashSet<NaiveDate> {
        let dates_from_monday = self.dates_from_monday();
        let mut working_dates = HashSet::new();
        for entry in self
            .entries()
            .iter()
            .filter(|entry| dates_from_monday.contains(&entry.start.unwrap().date_naive()))
        {
            working_dates.insert(entry.start.unwrap().date_naive());
        }

        working_dates
    }

    fn current_month_work_days(&self) -> HashSet<NaiveDate> {
        let mut working_dates = HashSet::new();
        for entry in self.entries().iter() {
            working_dates.insert(entry.start.unwrap().date_naive());
        }

        working_dates
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TimeEntry {
    pub description: Description,
    pub client: Option<ClientName>,
    pub project: ProjectName,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
    pub billable_amount_cents: usize,
}

impl TimeEntry {
    pub fn duration(&self) -> Duration {
        self.end.unwrap().signed_duration_since(self.start.unwrap())
    }
    pub fn is_for_current_date(&self) -> bool {
        self.start.unwrap().date_naive() == Local::now().date_naive()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    pub entries: Vec<TimeEntry>,
}
impl TimeEntryCalculations for TimeEntries {
    fn entries(&self) -> &Vec<TimeEntry> {
        &self.entries
    }
}
impl TimeEntries {
    pub fn uniq_projects(&self) -> Vec<Project> {
        let mut projects = HashSet::new();

        for entry in self.entries.iter() {
            let mut hasher = Sha256::default();
            hasher.update(&entry.project.as_str());
            let finalized_hash = format!("{:x}", &hasher.finalize());
            let project = Project {
                title: entry.project.clone(),
                client: entry.client.clone(),
                key: ProjectHash(finalized_hash),
                entries: self.entries_for_project(&entry.project),
            };
            projects.insert(project);
        }
        let mut projects_as_vec: Vec<Project> = projects.iter().cloned().collect();
        projects_as_vec.sort_by(|a, b| a.title.partial_cmp(&b.title).unwrap());
        projects_as_vec
    }

    fn entries_for_project(&self, project_title: &ProjectName) -> Vec<TimeEntry> {
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
    pub client: Option<ClientName>,
    pub title: ProjectName,
    pub key: ProjectHash,
    pub entries: Vec<TimeEntry>,
}

impl TimeEntryCalculations for Project {
    fn entries(&self) -> &Vec<TimeEntry> {
        &self.entries
    }
}

#[derive(Debug)]
pub struct CommonHours<'a> {
    pub target_daily_hours: &'a u8,
}
