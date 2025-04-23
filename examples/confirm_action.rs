use inquire::InquireError;
use strawr::utils::input;

fn main() {
    match input::confirm_action("Would you like to code in rust?").prompt() {
        Ok(input) => println!("{input}"),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            println!("Canceled")
        }
        Err(e) => eprintln!("Error: {e}"),
    };
}
