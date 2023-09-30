pub mod types;
use chrono::NaiveDate;
use std::collections::HashMap;
use crate::string_types::ApiKey;

static API_URL: &str = "https://api.track.toggl.com";
static API_BASIC_AUTH_PW: &str = "api_token";
static USER_AGENT: &str = "https://github.com/hasanen/my-hours";
static DATE_FORMAT: &str = "%Y-%m-%d";

#[tokio::main]
/// Get current user's profile
pub async fn get_me(api_key: &ApiKey) -> types::User {
    let user: types::User = get("api/v9/me", api_key, &None)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    user
}

#[tokio::main]
/// Get all workspaces where user has access to
pub async fn get_workspaces(api_key: &ApiKey) -> Vec<types::Workspace> {
    let workspaces: Vec<types::Workspace> = get("api/v9/workspaces", api_key, &None)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    workspaces
}

#[tokio::main]
/// Get time entries for given workspaces.
pub async fn get_time_entries(
    workspace_id: &usize,
    user_id: &usize,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    api_key: &ApiKey,
) -> Vec<types::TimeEntry> {
    let mut time_entries = Vec::new();

    let mut items_fetch: usize = 0;
    let mut items_left = true;
    let mut page = 1;

    while items_left {
        let params = time_entries_params(workspace_id, user_id, start_date, end_date, &page);
        let time_entry_response: types::TimeEntryResponse =
            get("reports/api/v2/details", api_key, &params)
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
        time_entries.push(time_entry_response.data.clone());
        items_fetch += time_entry_response.data.len();
        page += 1;

        if items_fetch >= time_entry_response.total_count {
            items_left = false;
        }
    }

    time_entries.concat()
}

async fn get(
    path: &str,
    api_key: &ApiKey,
    params: &Option<HashMap<String, String>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let request_url = api_url(path);
    let mut request = reqwest::Client::new()
        .get(request_url)
        .basic_auth(api_key.as_str(), Some(API_BASIC_AUTH_PW));

    if params.is_some() {
        request = request.query(&params.as_ref().unwrap());
    }
    let response = request.send().await?;

    check_status(&response);
    Ok(response)
}

fn api_url(path: &str) -> String {
    format!("{}/{}", API_URL, path)
}

fn check_status(response: &reqwest::Response) {
    match response.error_for_status_ref() {
        Ok(_res) => (),
        Err(err) => {
            match err.status() {
                Some(reqwest::StatusCode::FORBIDDEN) => {
                    println! {"API responded with 403, check your api key."}
                }
                _ => {
                    println! {"API responded with {}, not sure what to do.", err.status().unwrap().as_u16()}
                }
            }
            std::process::exit(1);
        }
    }
}

fn time_entries_params(
    workspace_id: &usize,
    user_id: &usize,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    page: &usize,
) -> Option<HashMap<String, String>> {
    let params = [
        ("workspace_id".to_string(), workspace_id.to_string()),
        ("user_ids".to_string(), user_id.to_string()),
        (
            "since".to_string(),
            start_date.format(DATE_FORMAT).to_string(),
        ),
        (
            "until".to_string(),
            end_date.format(DATE_FORMAT).to_string(),
        ),
        ("user_agent".to_string(), USER_AGENT.to_string()),
        ("page".to_string(), page.to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    Some(params)
}
