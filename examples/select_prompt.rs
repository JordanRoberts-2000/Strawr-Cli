use strawr::prompt::{traits::SelectPrompt, UserInput};

fn main() {
    let fruits: Vec<String> = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let input = UserInput;
    match input.select(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
