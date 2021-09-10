use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Action {
    #[structopt(name = "setup")]
    /// Setup new integration
    Setup,
}

pub fn execute(action: &Action) {
    match action {
        Setup => println!("Setup a new integration"),
    }
}
