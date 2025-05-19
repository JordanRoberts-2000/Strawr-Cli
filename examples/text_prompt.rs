use strawr::services::prompt::{user::UserInputRepo, PromptService};

fn main() {
    let prompt = PromptService::new(UserInputRepo);
    match prompt.text("Please enter some text:") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
