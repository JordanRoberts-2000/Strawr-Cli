use strawr::services::prompt::PromptService;

fn main() {
    let prompt = PromptService::new();
    match prompt.confirm("Would you like to code in rust?") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
