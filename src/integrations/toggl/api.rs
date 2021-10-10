pub mod types;
use chrono::{Date, Datelike, Local};
use std::collections::HashMap;

static API_URL: &str = "https://api.track.toggl.com/";
static API_BASIC_AUTH_PW: &str = "api_token";
static USER_AGENT: &str = "https://github.com/hasanen/my-hours";
static DATE_FORMAT: &str = "%Y-%m-%d";

#[tokio::main]
/// Get all workspaces where user has access to
pub async fn get_workspaces(api_key: &str) -> Vec<types::Workspace> {
    let workspaces: Vec<types::Workspace> = get(&"api/v8/workspaces", api_key, &None)
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
    start_date: &Date<Local>,
    end_date: &Date<Local>,
    api_key: &str,
) -> Vec<types::TimeEntry> {
    let mut time_entries = Vec::new();
    let params: HashMap<String, String> = [
        ("workspace_id".to_string(), workspace_id.to_string()),
        (
            "since".to_string(),
            start_date.format(DATE_FORMAT).to_string(),
        ),
        (
            "until".to_string(),
            end_date.format(DATE_FORMAT).to_string(),
        ),
        ("user_agent".to_string(), USER_AGENT.to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let time_entry_response: types::TimeEntryResponse =
        get(&"reports/api/v2/details", api_key, &Some(params))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

    println!("{:?}", time_entry_response.per_page);
    println!("{:?}", time_entry_response.total_count);
    println!("{:?}", time_entry_response.data.len());
    if (time_entry_response.total_count > time_entry_response.per_page) {}
    time_entries.push(time_entry_response.data);
    time_entries.concat()
}

async fn get(
    path: &str,
    api_key: &str,
    params: &Option<HashMap<String, String>>,
) -> Result<reqwest::Response, reqwest::Error> {
    let request_url = api_url(&path);
    let mut request = reqwest::Client::new()
        .get(request_url)
        .basic_auth(api_key, Some(API_BASIC_AUTH_PW));

    if params.is_some() {
        request = request.query(&params.as_ref().unwrap());
    }
    let response = request.send().await?;

    check_status(&response);
    Ok(response)
}

fn api_url(path: &str) -> String {
    String::from(format!("{}/{}", API_URL, path))
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
