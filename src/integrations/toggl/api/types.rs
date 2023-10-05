use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::strict_string::{Email, Fullname, WorkspaceName, Description, ClientName, ProjectName };

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: WorkspaceName,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub fullname: Fullname,
    pub email: Email,
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
    pub description: Option<Description>,
    pub client: Option<ClientName>,
    pub project: Option<ProjectName>,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
    pub billable: Option<f32>,
}
