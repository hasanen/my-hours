//! Keep track on your tracked hours in different services
//!
//! Currently supported services:
//!   - Toggl track - <https://track.toggl.com>

#![deny(missing_docs)]
use structopt::StructOpt;
mod hours;
mod integrations;
pub mod settings;

#[derive(StructOpt, Debug)]
#[structopt(name = "My hours")]
struct Arguments {
    /// Command to use: hours, integrations etc
    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(name = "integrations")]
    IntegrationsCommand {
        #[structopt(subcommand)]
        action: integrations::Action,
    },
}

fn main() {
    let args = Arguments::from_args();
    match &args.subcommand {
        Some(Subcommand::IntegrationsCommand { action }) => integrations::execute(action),
        None => hours::execute(),
    }
}
