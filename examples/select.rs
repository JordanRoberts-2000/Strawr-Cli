use inquire::InquireError;
use strawr::utils::input;

fn main() {
    let fruits = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ];

    match input::select(&fruits, "Choose a fruit:\n")
        .without_filtering()
        .prompt()
    {
        Ok(input) => println!("{input}"),
        Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
            println!("Canceled")
        }
        Err(e) => eprintln!("Error: {e}"),
    };
}
