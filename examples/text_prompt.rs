use strawr::prompt::{traits::TextPrompt, UserInput};

fn main() {
    let input = UserInput;
    match input.text("Please enter some text:") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
