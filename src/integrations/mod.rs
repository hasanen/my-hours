use crate::dates;
use crate::hours::types::{TimeEntries, TimeEntry};
use crate::integrations::toggl::Config as TogglConfig;
use crate::settings;
use chrono::Local;
use clap::Parser;

pub mod toggl;

#[derive(Parser, Debug)]
pub enum Action {
    /// Setup new integration
    Setup {
        #[clap(subcommand)]
        integration: Integration,
    },
    /// List enable integrations
    List,
}

#[derive(Parser, Debug, Clone)]
pub enum Integration {
    #[clap(name = "toggl")]
    TogglIntegration,
}

pub fn execute(action: &Action) {
    match action {
        Action::Setup { integration } => match integration {
            Integration::TogglIntegration => toggl::setup(),
        },
        Action::List => {
            let settings = settings::load();
            list_integrations(&settings, &mut std::io::stdout())
        }
    }
}

/// Loop over integrations and get time entries for current month
pub fn get_monthly_time_entries() -> TimeEntries {
    let settings = settings::load();
    let (start_date, end_date) = dates::month_first_and_last_dates(&Local::now().date_naive());

    let entries: Vec<Vec<TimeEntry>> = match settings.toggl {
        Some(toggl) => toggl
            .iter()
            .map(|toggl_config| toggl::time_entries_for_dates(toggl_config, &start_date, &end_date))
            .collect(),
        None => Vec::new(),
    };

    TimeEntries {
        entries: entries.concat(),
    }
}

/// List integrations to given writer
pub fn list_integrations(config: &settings::Config, mut writer: impl std::io::Write) {
    match &config.toggl {
        Some(toggl) => {
            writeln!(writer, "Enabled integrations:").unwrap();
            writeln!(writer).unwrap();
            list_toggl_integrations(toggl, &mut writer);
        }
        None => {
            writeln!(writer, "No integrations set up yet.").unwrap();
        }
    }
}

fn list_toggl_integrations(toggl: &[TogglConfig], mut writer: impl std::io::Write) {
    for t in toggl.iter() {
        let workspace_names: Vec<String> =
            t.workspaces.iter().map(|ws| ws.name.as_str().to_string()).collect();
        writeln!(writer, "Toggl, workspaces: {}", workspace_names.join(", ")).unwrap();
    }
}

#[cfg(test)]
mod tests {
    mod show_integrations {
        use super::super::*;
        use crate::integrations;
        use crate::string_types::{ApiKey, Email, Fullname, WorkspaceName};

        static DEFAULT_CONFIG: settings::Config = settings::Config {
            refresh_treshold: Some(180),
            refreshed_at: None,
            project_configs: None,
            toggl: None,
        };
        #[test]
        fn no_integrations_shows_notification() {
            let mut result = Vec::new();

            list_integrations(&DEFAULT_CONFIG, &mut result);

            assert_eq!(result, b"No integrations set up yet.\n");
        }
        #[test]
        fn toggl_integration_shows_workspaces() {
            let mut result = Vec::new();
            let config = settings::Config {
                toggl: Some(
                    [TogglConfig {
                        key: ApiKey("key".to_string()),
                        user: integrations::toggl::User {
                            id: 1,
                            fullname: Fullname("John Doe".to_string()),
                            email: Email("john.doe@example.com".to_string()),
                        },
                        workspaces: [integrations::toggl::Workspace {
                            id: 1,
                            name: WorkspaceName("Test".to_string()),
                        }]
                        .to_vec(),
                    }]
                    .to_vec(),
                ),
                refresh_treshold: Some(180),
                refreshed_at: None,
                project_configs: None,
            };

            list_integrations(&config, &mut result);

            assert_eq!(
                result,
                b"Enabled integrations:\n\nToggl, workspaces: Test\n"
            );
        }
    }
}
