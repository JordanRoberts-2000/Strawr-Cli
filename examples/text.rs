use strawr::utils::input::{TextInput, UserInput};

fn main() {
    let input = UserInput;
    match input.text("Please enter some text:") {
        Ok(input) => println!("Input: '{input}'"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
