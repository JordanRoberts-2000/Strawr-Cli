use strawr::utils::input::{SelectWithoutFilterInput, UserInput};

fn main() {
    let fruits = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ];

    let input = UserInput;
    match input.select_without_filter(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
