use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
