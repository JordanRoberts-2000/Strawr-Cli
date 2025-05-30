use clap::Parser;
use colored::*;
use strawr::{run_cli, Cli};

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run_cli(&cli) {
        if cli.debug {
            eprintln!("{}", format!("Error: {:#?}", error).red());
        } else {
            eprintln!("{}", format!("Error: {}", error).red());
        }
        std::process::exit(1);
    }
}
