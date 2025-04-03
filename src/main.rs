use colored::*;
use strawr::run_cli;

fn main() {
    if let Err(error) = run_cli() {
        eprintln!("{}", format!("{}", error).red());
        std::process::exit(1);
    }
}
