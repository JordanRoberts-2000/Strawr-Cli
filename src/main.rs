use colored::*;
use strawr::run_app;

fn main() {
    if let Err(error) = run_app() {
        eprintln!("{}", format!("{}", error).red());
        std::process::exit(1);
    }
}
