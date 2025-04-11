use clap::Parser;
use colored::*;
use strawr::{cli::Cli, run_cli};

fn main() {
    let cli = Cli::parse();
    let debug = cli.debug;

    if let Err(error) = run_cli(cli) {
        if debug {
            eprintln!("Error: {:#?}", error);
        } else {
            eprintln!("{}", format!("{}", error).red());
        }
        std::process::exit(1);
    }
}
