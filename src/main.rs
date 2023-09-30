//! Keep track on your tracked hours in different services
//!
//! Currently supported services:
//!   - Toggl track - <https://track.toggl.com>

#![deny(missing_docs)]
use clap::Parser;
pub mod dates;
pub mod settings;
mod hours;
mod integrations;
mod string_types;

#[derive(Parser, Debug)]
#[structopt(name = "My hours")]
struct Cli {
    /// Command to use: hours, integrations etc
    #[clap(subcommand)]
    command: Option<Command>,
    /// Refresh hours from integrations before printing them
    #[clap(long)]
    refresh: bool,
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
    /// Show some basic info
    Info,
}
fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Command::IntegrationsCommand { action }) => integrations::execute(action),
        Some(Command::Info) => hours::print_info(),
        Some(Command::Refresh) => hours::refresh_all(),
        None => {
            if args.refresh {
                hours::refresh_all()
            }
            hours::show_monthly_hours()
        }
    }
}
