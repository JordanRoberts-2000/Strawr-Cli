use strawr::services::prompt::{user::UserInputRepo, PromptService};

fn main() {
    let prompt = PromptService::new(UserInputRepo);
    match prompt.multi_select_with_defaults(
        &vec!["option1", "option2"],
        &[1],
        "Would you like to code in rust?",
    ) {
        Ok(input) => println!("Input: '{input:?}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
