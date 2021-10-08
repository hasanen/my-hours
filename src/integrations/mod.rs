use structopt::StructOpt;

pub mod toggl;

#[derive(StructOpt, Debug)]
pub enum Action {
    #[structopt(name = "setup")]
    /// Setup new integration
    Setup {
        #[structopt(subcommand)]
        integration: Integration,
    },
}

#[derive(StructOpt, Debug)]
pub enum Integration {
    #[structopt(name = "toggl")]
    TogglIntegration,
}

pub fn execute(action: &Action) {
    match action {
        Action::Setup { integration } => match integration {
            Integration::TogglIntegration => toggl::setup(),
        },
    }
}
