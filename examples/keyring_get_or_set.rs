use colored::*;
use inquire::PasswordDisplayMode;
use strawr::services::keyring::KeyringService;

fn main() {
    let mut service = KeyringService::new("Example");
    service.set_password_mode(&PasswordDisplayMode::Masked);
    match service.get_or_set("example_field") {
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
        Ok(retrieved) => println!("Retrieved: {retrieved}"),
    };
}
