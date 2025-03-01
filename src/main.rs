use strawr::run_app;

fn main() {
    if let Err(error) = run_app() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
