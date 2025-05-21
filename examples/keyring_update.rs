use colored::*;
use inquire::PasswordDisplayMode;
use strawr::services::keyring::KeyringService;

fn main() {
    let mut service = KeyringService::new("Example");
    service.set_password_mode(&PasswordDisplayMode::Masked);
    if let Err(e) = service.update("example_field") {
        eprintln!("{}", format!("Error: {}", e).red());
    };
}
