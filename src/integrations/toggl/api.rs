mod types;

static API_URL: &str = "https://api.track.toggl.com/api/";
static API_BASIC_AUTH_PW: &str = "api_token";

#[tokio::main]
/// Get all workspaces where user has access to
pub async fn get_workspaces(api_key: &str) -> Vec<types::Workspace> {
    let workspaces: Vec<types::Workspace> = get(&"v8/workspaces", api_key)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    workspaces
}

async fn get(path: &str, api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let request_url = api_url(&path);
    let response = reqwest::Client::new()
        .get(request_url)
        .basic_auth(api_key, Some(API_BASIC_AUTH_PW))
        .send()
        .await?;

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
