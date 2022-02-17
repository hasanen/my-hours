//! Keep track on your tracked hours in different services
//!
//! Currently supported services:
//!   - Toggl track - <https://track.toggl.com>

#![deny(missing_docs)]
use clap::{Parser};
pub mod dates;
mod hours;
mod integrations;
pub mod settings;

#[derive(Parser, Debug)]
#[structopt(name = "My hours")]
struct Cli {
    /// Command to use: hours, integrations etc
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Parser, Debug)]
enum Command {
    #[clap(name = "integrations")]
    /// Manage integrations
    IntegrationsCommand {
        #[clap(subcommand)]
        action: integrations::Action,
    },
    /// Refresh hours through integrations
    Refresh,
}
fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Command::IntegrationsCommand { action }) => integrations::execute(action),
        Some(_refresh) => hours::refresh_all(),
        None => hours::show_monthly_hours(),
    }
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Args::into_app().debug_assert()
}