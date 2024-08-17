use clap::Parser;

use util::Result;

mod commands;
mod util;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    /// create new workspace
    New(commands::new::Create),
    /// initialized workspace
    Init(commands::init::Init),
    /// list workspace members
    List(commands::list::List),
}

#[derive(Debug, Parser)]
#[clap(name = "cargo-workspaces", bin_name = "cargo", version)]
enum Cargo {
    #[clap(version)]
    Workspaces(Cli),
}

fn main() -> Result {
    let Cargo::Workspaces(opt) = Cargo::parse();
    match &opt.command {
        Commands::New(x) => x.exec(),
        Commands::Init(x) => x.exec(),
        Commands::List(x) => x.exec(),
    }
}
