use strawr::utils::input::{SelectWithoutFilterInput, UserInput};

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
    match input.select_without_filter(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
