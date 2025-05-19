use inquire::PasswordDisplayMode;
use strawr::services::prompt::PromptService;

fn main() {
    let mut prompt = PromptService::new();
    prompt.set_password_mode(&PasswordDisplayMode::Masked);
    match prompt.password("Set password:") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
