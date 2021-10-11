use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryResponse {
    pub total_count: usize,
    pub per_page: usize,
    pub data: Vec<TimeEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeEntry {
    pub id: usize,
    pub description: Option<String>,
    pub client: Option<String>,
    pub project: Option<String>,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
    pub billable: Option<f32>,
}
