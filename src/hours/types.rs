use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub description: String,
    #[serde(with = "ts_seconds_option")]
    pub start: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end: Option<DateTime<Utc>>,
}
