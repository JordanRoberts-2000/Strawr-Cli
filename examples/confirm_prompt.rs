use strawr::services::prompt::{traits::ConfirmPrompt, user::UserInputRepo, PromptService};

fn main() {
    let prompt = PromptService::new(UserInputRepo);
    match prompt.confirm("Would you like to code in rust?") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
