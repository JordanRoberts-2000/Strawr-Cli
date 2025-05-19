use strawr::services::prompt::PromptService;

fn main() {
    let prompt = PromptService::new();
    match prompt.text("Please enter some text:") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
