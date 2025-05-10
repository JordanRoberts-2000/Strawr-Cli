use strawr::prompt::{traits::SearchPrompt, UserInput};

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
    match input.search(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
