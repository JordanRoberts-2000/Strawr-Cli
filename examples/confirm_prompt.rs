use strawr::prompt::{traits::ConfirmPrompt, UserInput};

fn main() {
    let input = UserInput;
    match input.confirm("Would you like to code in rust?") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
