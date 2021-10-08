use config;
use directories_next::BaseDirs;
use structopt::StructOpt;
mod hours;
mod integrations;

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
    let config = load_config();
    println!("{:?}", config);

    let args = Arguments::from_args();
    match &args.subcommand {
        Some(Subcommand::IntegrationsCommand { action }) => integrations::execute(action),
        None => hours::execute(),
    }
}

fn load_config() -> config::Config {
    if let Some(base_dirs) = BaseDirs::new() {
        println!("{}", base_dirs.config_dir().display());
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    }
    return config::Config::default();
}
